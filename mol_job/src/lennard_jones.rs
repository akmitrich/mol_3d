#![allow(unused, dead_code)]

use std::sync::atomic::Ordering;
use crate::{boundaries::BoundaryConditions, potential::PotentialEnergy};
use d_vector::{reset_array, DVector, Real};
use atomic_float::AtomicF32;

#[derive(Debug)]
pub struct LennardJones {
    r_cut: Real,
    u_sum: AtomicF32,
    v_sum: AtomicF32,
}

impl Default for LennardJones {
    fn default() -> Self {
        Self {
            r_cut: 2.5,
            u_sum: AtomicF32::new(0.0),
            v_sum: AtomicF32::new(0.0),
        }
    }
}

impl<const D: usize> PotentialEnergy<D> for LennardJones {
    fn compute_forces(
        &self,
        pos: &[DVector<D>],
        acc: &mut [DVector<D>],
        boundaries: &dyn BoundaryConditions<D>,
    ) {
        let n_mol = pos.len();
        assert_eq!(n_mol, acc.len());

        let rr_cut = self.r_cut * self.r_cut;
        reset_array(acc);
        let mut u_sum = 0 as Real;
        let mut v_sum = 0 as Real;

        if n_mol > 0 {
            for j1 in 0..(n_mol - 1) {
                for j2 in (j1 + 1)..n_mol {
                    let mut dr = &pos[j1] - &pos[j2];
                    boundaries.wrap(&mut dr);
                    let rr = dr.square_length();
                    if rr < rr_cut {
                        let rri = 1. / rr;
                        let rri3 = rri * rri * rri;

                        let force_value = 48. * rri3 * (rri3 - 0.5) * rri;
                        let force = force_value * dr;

                        acc[j1] += &force;
                        acc[j2] -= &force;

                        u_sum += 4. * rri3 * (rri3 - 1.) + 1.;
                        v_sum += force_value * rr;
                    }
                }
            }
        }
        self.u_sum.store(u_sum, Ordering::SeqCst);
        self.v_sum.store(v_sum, Ordering::SeqCst);
    }

    fn u_sum(&self) -> Real {
        self.u_sum.load(Ordering::SeqCst)
    }

    fn virial_sum(&self) -> Real {
        self.v_sum.load(Ordering::SeqCst)
    }
}

impl LennardJones {
    pub fn new(r_cut: Real) -> Self {
        Self {
            r_cut,
            ..Default::default()
        }
    }
}
