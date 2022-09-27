#![allow(unused, dead_code)]

use crate::{
    boundaries::{BoundaryConditions, Region},
    lennard_jones::LennardJones,
    potential::PotentialEnergy,
    prop::{Props, TrivialProps},
    state::{MolecularState, State},
    verlet,
};
use d_vector::{DVector, Real};
use std::{
    cell::{Cell, RefCell, RefMut},
    ops::AddAssign,
};

#[derive(Debug)]
pub struct Job<const D: usize> {
    state: Box<dyn MolecularState<D>>,
    boundaries: Box<dyn BoundaryConditions<D>>,
    potential: Box<dyn PotentialEnergy<D>>,
    props: Box<dyn Props<D>>,
    step_count: usize,
    delta_t: Real,
    more_cycles: bool,
}

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Self {
            state: Box::new(State::default()),
            boundaries: Box::new(Region::new([50.; D])),
            potential: Box::new(LennardJones::default()),
            props: Box::new(TrivialProps::default()),
            step_count: 0,
            delta_t: 0.005,
            more_cycles: true,
        }
    }
}

impl<const D: usize> Job<D> {
    pub fn run(&mut self, steps: usize) -> usize {
        self.more_cycles = true;
        let step_limit = self.step_count() + steps;
        while self.more_cycles {
            self.advance_step_count();
            verlet::single_step(
                self.delta_t(),
                &mut self.state.get_pos(),
                &mut self.state.get_vel(),
                &mut self.state.get_acc(),
                self.boundaries.as_ref(),
                self.potential.as_ref(),
            );
            self.update_props();
            self.state.sync(self.time_now());

            if self.step_count() >= step_limit {
                self.more_cycles = false;
            }
        }
        self.step_count() - step_limit
    }

    fn advance_step_count(&mut self) {
        self.step_count += 1;
    }

    fn delta_t(&self) -> Real {
        self.delta_t
    }

    fn update_props(&self) {
        /*         println!(
            "{}. u = {}, v = {} <--- update_props()",
            self.step_count(),
            self.potential.u_sum(),
            self.potential.virial_sum()
        );*/
        self.props.eval_props(
            self.potential.as_ref(),
            &self.state.get_pos(),
            &self.state.get_vel(),
        );
        self.props.accum_props();
        if self.props.need_avg(self.step_count()) {
            self.props.avg_props();
            self.props.summarize();
            self.props.reset();
        }
    }

    pub fn time_now(&self) -> Real {
        self.delta_t() * self.step_count() as Real
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }

    pub fn vel_sum(&self) -> DVector<D> {
        let mut result = DVector::default();
        for velocity in self.state.get_vel().iter() {
            result += velocity;
        }
        result
    }
}

pub struct JobSetup<const D: usize>(Job<D>);

impl<const D: usize> JobSetup<D> {
    pub fn build() -> Self {
        let mut job = Job::default();
        Self(job)
    }

    pub fn delta_t(mut self, dt: Real) -> Self {
        self.0.delta_t = dt;
        self
    }

    pub fn state(mut self, state: impl MolecularState<D> + 'static) -> Self {
        self.0.state = Box::new(state);
        self
    }

    pub fn potential(mut self, potential: impl PotentialEnergy<D> + 'static) -> Self {
        self.0.potential = Box::new(potential);
        self
    }

    pub fn props(mut self, props: impl Props<D> + 'static) -> Self {
        self.0.props = Box::new(props);
        self
    }

    pub fn boundaries(mut self, boundaries: impl BoundaryConditions<D> + 'static) -> Self {
        self.0.boundaries = Box::new(boundaries);
        self
    }

    pub fn init_pos(mut self, pos: Vec<DVector<D>>) -> Self {
        let n_mol = pos.len();
        *self.0.state.get_pos() = pos;
        *self.0.state.get_vel() = vec![DVector::default(); n_mol];
        *self.0.state.get_acc() = vec![DVector::default(); n_mol];
        self
    }

    pub fn random_vel(mut self, temperature: Real) -> Self {
        let n_mol = self.0.state.get_pos().len();
        let vel_mag = (temperature * (D as Real) * (1. - 1. / (n_mol as Real))).sqrt();
        crate::initial_state::randomize_vectors(&mut self.0.state.get_vel(), vel_mag);
        let sum = self.0.vel_sum();
        let k = -1. / n_mol as Real;
        crate::initial_state::shift_vectors(&mut self.0.state.get_vel(), &(k * sum));
        self
    }

    pub fn job(self) -> Job<D> {
        self.0
    }
}
