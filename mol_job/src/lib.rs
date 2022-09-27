pub mod boundaries;
pub mod initial_state;
pub mod job;
pub mod lennard_jones;
pub mod potential;
pub mod prop;
pub mod state;
pub mod track;
pub mod verlet;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use job::{Job, JobSetup};
        use potential::NoInteraction;
        let mut j: Job<3> = JobSetup::build()
            .delta_t(1e-3)
            .potential(NoInteraction::default())
            .job();
        assert_eq!(0, j.run(100));
        assert_eq!(0.1, j.time_now())
    }

    #[test]
    fn wrap() {
        use boundaries::{BoundaryConditions, Region};
        use d_vector::DVector;
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([1.5, -4.]);
        region.wrap(&mut p);
        assert_eq!(&[0.5, 1.], p.components());
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([0.2, -1.5]);
        region.wrap(&mut p);
        assert_eq!(&[0.2, -1.5], p.components());
    }

    #[test]
    fn cubic_lattice() {
        use job::{Job, JobSetup};
        use potential::NoInteraction;

        let (boundaries, pos) = initial_state::cubic_lattice(1000, 0.8);
        let mut j: Job<3> = JobSetup::build()
            .boundaries(boundaries)
            .init_pos(pos)
            .potential(NoInteraction::default())
            .job();
        assert_eq!(0, j.run(100));
        assert_eq!(0.5, j.time_now());
        assert!(j.vel_sum().length() < 1e-3);
    }
}
