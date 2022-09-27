#![allow(unused, dead_code)]

use crate::boundaries::BoundaryConditions;
use d_vector::{DVector, Real};
use std::{cell::Cell, fmt::Debug};

pub trait PotentialEnergy<const D: usize>: Debug {
    fn compute_forces(
        &self,
        pos: &[DVector<D>],
        acc: &mut [DVector<D>],
        boundaries: &dyn BoundaryConditions<D>,
    );
    fn u_sum(&self) -> Real {
        0.0
    }
    fn virial_sum(&self) -> Real {
        0.0
    }
}

#[derive(Debug, Default)]
pub struct NoInteraction;
impl<const D: usize> PotentialEnergy<D> for NoInteraction {
    fn compute_forces(
        &self,
        _: &[DVector<D>],
        _: &mut [DVector<D>],
        _: &dyn BoundaryConditions<D>,
    ) {
    }

}
