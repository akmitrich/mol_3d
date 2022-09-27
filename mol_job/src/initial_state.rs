#![allow(unused, dead_code)]

use crate::boundaries::Region;
use d_vector::{DVector, Real};
use std::ops::AddAssign;

pub fn cubic_lattice<const D: usize>(n_mol: usize, density: Real) -> (Region<D>, Vec<DVector<D>>) {
    let vol = n_mol as Real / density;
    let size = vol.powf(1. / D as Real);
    let region = Region::new([size; D]);

    let dim = (n_mol as Real).powf(1. / D as Real) as usize;
    let cells = [dim; D];
    let mut pos = Vec::with_capacity(number_of_atoms(&cells));
    lattice(&cells, &calc_gap(&region, &cells), &mut pos, [0.; D], 0);
    shift_vectors(&mut pos, &(-0.5 * DVector::from(region.dimensions())));

    (region, pos)
}

fn number_of_atoms(cells: &[usize]) -> usize {
    let mut result = 1;
    for cell in cells {
        result *= cell;
    }
    result
}

fn calc_gap<const D: usize>(region: &Region<D>, cells: &[usize; D]) -> [Real; D] {
    let mut result = [0.; D];
    for (i, component) in region.dimensions().iter().enumerate() {
        result[i] = component / cells[i] as Real;
    }
    result
}

fn lattice<const D: usize>(
    cells: &[usize; D],
    gap: &[Real; D],
    pos: &mut Vec<DVector<D>>,
    mut current: [Real; D],
    current_index: usize,
) {
    for i in 0..cells[current_index] {
        current[current_index] = (0.5_f32 + i as Real) * gap[current_index];
        if current_index == D - 1 {
            pos.push(DVector::from(current));
        } else {
            lattice(cells, gap, pos, current, current_index + 1)
        }
    }
}

pub fn shift_vectors<const D: usize>(vectors: &mut [DVector<D>], shift: &DVector<D>) {
    for v in vectors.iter_mut() {
        v.add_assign(shift);
    }
}

pub fn randomize_vectors<const D: usize>(vectors: &mut [DVector<D>], magnitude: Real) {
    for v in vectors.iter_mut() {
        let rnd = DVector::random_vector();
        *v = (magnitude / rnd.length()) * rnd;
    }
}
