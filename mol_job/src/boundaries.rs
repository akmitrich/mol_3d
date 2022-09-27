#![allow(unused, dead_code)]

use crate::verlet;
use d_vector::{DVector, Real};
use std::{
    fmt::Debug,
    ops::{AddAssign, SubAssign},
};

pub trait BoundaryConditions<const D: usize>: Debug {
    fn wrap(&self, pos: &mut DVector<D>);
}

#[derive(Debug)]
pub struct Region<const D: usize> {
    inner: DVector<D>,
}

impl<const D: usize> Region<D> {
    pub fn new(dimensions: [Real; D]) -> Self {
        Self {
            inner: DVector::from(dimensions),
        }
    }

    fn is_above(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] >= self.inner.components()[index] / 2.
    }

    fn is_below(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] < -self.inner.components()[index] / 2.
    }

    pub fn dimensions(&self) -> &[Real; D] {
        self.inner.components()
    }
}

impl<const D: usize> BoundaryConditions<D> for Region<D> {
    fn wrap(&self, position: &mut DVector<D>) {
        let mut shift = [0 as Real; D];
        for (i, s) in shift.iter_mut().enumerate() {
            if self.is_above(position, i) {
                s.sub_assign(self.inner.components()[i]);
            } else if self.is_below(position, i) {
                s.add_assign(self.inner.components()[i]);
            }
        }
        position.add_assign(DVector::from(shift));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
