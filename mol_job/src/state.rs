#![allow(unused, dead_code)]
use d_vector::{DVector, Real};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell, RefMut},
    fmt::Debug,
};

pub trait MolecularState<const D: usize>: Debug {
    fn get_pos(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_vel(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_acc(&self) -> RefMut<Vec<DVector<D>>>;
    fn sync(&self, time_now: Real) {}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State<const D: usize> {
    pos: RefCell<Vec<DVector<D>>>,
    vel: RefCell<Vec<DVector<D>>>,
    acc: RefCell<Vec<DVector<D>>>,
}

impl<const D: usize> MolecularState<D> for State<D> {
    fn get_pos(&self) -> RefMut<Vec<DVector<D>>> {
        self.pos.borrow_mut()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<D>>> {
        self.vel.borrow_mut()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<D>>> {
        self.acc.borrow_mut()
    }
}
