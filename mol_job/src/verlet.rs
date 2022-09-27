#![allow(unused, dead_code)]

use crate::{
    boundaries::BoundaryConditions, potential::PotentialEnergy, prop::Props, state::MolecularState,
};
use d_vector::{DVector, Real};
use std::{cell::RefMut, ops::AddAssign};

pub fn single_step<const D: usize>(
    delta_t: Real,
    pos: &mut [DVector<D>],
    vel: &mut [DVector<D>],
    acc: &mut [DVector<D>],
    boundaries: &dyn BoundaryConditions<D>,
    potential_energy: &dyn PotentialEnergy<D>,
) {
    leapfrog_begin(delta_t, pos, vel, acc);
    apply_boundary_conditions(boundaries, pos);
    potential_energy.compute_forces(pos, acc, boundaries);
    leapfrog_end(delta_t, vel, acc);
}

pub fn apply_boundary_conditions<const D: usize>(
    boundaries: &dyn BoundaryConditions<D>,
    pos: &mut [DVector<D>],
) {
    for position in pos.iter_mut() {
        boundaries.wrap(position)
    }
}

pub fn leapfrog_begin<const D: usize>(
    delta_t: Real,
    pos: &mut [DVector<D>],
    vel: &mut [DVector<D>],
    acc: &[DVector<D>],
) {
    assert_eq!(pos.len(), vel.len());
    calc_vel_for_half_step(delta_t, vel, acc);
    for (position, velocity) in pos.iter_mut().zip(vel.iter()) {
        position.add_assign(delta_t * velocity);
    }
}

pub fn leapfrog_end<const D: usize>(delta_t: Real, vel: &mut [DVector<D>], acc: &[DVector<D>]) {
    assert_eq!(vel.len(), acc.len());
    calc_vel_for_half_step(delta_t, vel, acc);
}

fn calc_vel_for_half_step<const D: usize>(
    delta_t: Real,
    vel: &mut [DVector<D>],
    acc: &[DVector<D>],
) {
    let half_delta_t = delta_t / 2.;
    for (velocity, acceleration) in vel.iter_mut().zip(acc.iter()) {
        velocity.add_assign(half_delta_t * acceleration);
    }
}
