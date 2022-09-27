#![allow(unused, dead_code)]
use rand::Rng;
use serde::{
    de::{self, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
    Deserialize,
};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

pub type Real = f32;

pub fn reset_array<const D: usize>(vectors: &mut [DVector<D>]) {
    for vector in vectors.iter_mut() {
        *vector = Default::default();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DVector<const D: usize> {
    components: [Real; D],
}

impl<const D: usize> DVector<D> {
    pub fn components(&self) -> &[Real; D] {
        &self.components
    }

    pub fn square_length(&self) -> Real {
        self * self
    }

    pub fn length(&self) -> Real {
        self.square_length().sqrt()
    }

    pub fn random_vector() -> Self {
        let mut components = [0 as Real; D];
        let mut rng = rand::thread_rng();
        for component in components.iter_mut() {
            *component = rng.gen();
            *component -= 0.5;
        }
        Self::from(components)
    }
}

impl<const D: usize> Default for DVector<D> {
    fn default() -> Self {
        Self {
            components: [0.; D],
        }
    }
}

impl<const D: usize> From<&[Real; D]> for DVector<D> {
    fn from(data: &[Real; D]) -> Self {
        Self::from(*data)
    }
}

impl<const D: usize> From<[Real; D]> for DVector<D> {
    fn from(components: [Real; D]) -> Self {
        Self { components }
    }
}

impl<const D: usize> AddAssign for DVector<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<const D: usize> AddAssign<&Self> for DVector<D> {
    fn add_assign(&mut self, rhs: &Self) {
        for (i, c) in self.components.iter_mut().enumerate() {
            c.add_assign(rhs.components()[i]);
        }
    }
}

impl<const D: usize> Add for DVector<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self;
        sum += rhs;
        sum
    }
}

impl<const D: usize> Add for &DVector<D> {
    type Output = DVector<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum: DVector<D> = self.clone();
        sum += rhs;
        sum
    }
}

impl<const D: usize> Mul<DVector<D>> for Real {
    type Output = DVector<D>;

    fn mul(self, rhs: DVector<D>) -> Self::Output {
        let mut components = *rhs.components();
        for c in components.iter_mut() {
            c.mul_assign(self);
        }
        Self::Output { components }
    }
}

impl<const D: usize> Mul<&DVector<D>> for &DVector<D> {
    type Output = Real;

    fn mul(self, rhs: &DVector<D>) -> Self::Output {
        self.components()
            .iter()
            .zip(rhs.components())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}

impl<const D: usize> Mul<&DVector<D>> for Real {
    type Output = DVector<D>;

    fn mul(self, rhs: &DVector<D>) -> Self::Output {
        let mut components = *rhs.components();
        for c in components.iter_mut() {
            c.mul_assign(self);
        }
        Self::Output { components }
    }
}

impl<const D: usize> SubAssign<&Self> for DVector<D> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (i, c) in self.components.iter_mut().enumerate() {
            c.sub_assign(rhs.components()[i]);
        }
    }
}

impl<const D: usize> Sub<&DVector<D>> for &DVector<D> {
    type Output = DVector<D>;

    fn sub(self, rhs: &DVector<D>) -> Self::Output {
        let mut sub = self.clone();
        sub -= rhs;
        sub
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = DVector::from(&[1., 0.22, 1e-6]);
        assert_eq!(&[1., 0.22, 1e-6], v.components());
    }

    #[test]
    fn add_assign() {
        let mut v = DVector::from([1., 2., 3.]);
        v += DVector::from([1., 0., 1.]);
        assert_eq!(&[2., 2., 4.], v.components());
        v += DVector::from([0., 0., -2.]);
        assert_eq!(&[2.; 3], v.components());
        v += v.clone(); // How to make v += &v?
        assert_eq!(&[4.; 3], v.components());
        let b = DVector::from([0., 1.5, 6.3]);
        v += &b;
        assert_eq!(&[4., 5.5, 10.3], v.components());
    }

    #[test]
    fn add() {
        let a = DVector::from([1., 2., 3.]);
        let b = DVector::from([4., 5., 6.]);
        assert_eq!(&[2., 4., 6.], (&a + &a).components());
        assert_eq!(&[5., 7., 9.], (&a + &b).components());
        assert_eq!(&[5., 7., 9.], (&b + &a).components());
        assert_eq!(&[5., 7., 9.], (a + b).components());
    }

    #[test]
    fn number_mul_vector() {
        let v = DVector::from([1., 2., 3.]);
        assert_eq!(&[1., 2., 3.], (1. * &v).components());
        assert_eq!(&[-1., -2., -3.], (-1. * &v).components());
        assert_eq!(&[1.5, 3., 4.5], (1.5 * v).components());
    }

    #[test]
    fn mul_square() {
        let a = DVector::from([1., 2., 3.]);
        let b = DVector::from([4., 5., 6.]);
        assert_eq!(32., &a * &b);
        assert_eq!(32., &b * &a);
        assert_eq!(14., &a * &a);
        assert_eq!(77., &b * &b);
        assert_eq!(&a * &a, a.square_length());
        assert_eq!(&b * &b, b.square_length());
    }
}

impl<const D: usize> Serialize for DVector<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(D))?;
        for e in self.components.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct DVectorVisitor<const D: usize>;

impl<'de, const D: usize> Visitor<'de> for DVectorVisitor<D> {
    type Value = DVector<D>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let expect = format!("an array of {} floats", D);
        formatter.write_str(&expect)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut components = [0 as Real; D];
        for c in components.iter_mut() {
            let val: Option<Real> = seq.next_element()?;
            if let Some(component) = val {
                *c = component;
            } else {
                return Err(de::Error::invalid_length(D, &self));
            }
        }
        Ok(DVector::from(components))
    }
}

impl<'de, const D: usize> Deserialize<'de> for DVector<D> {
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(DVectorVisitor)
    }
}
