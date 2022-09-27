#![allow(unused, dead_code)]

use crate::state::{MolecularState, State};
use d_vector::{DVector, Real};
use std::{
    cell::{Cell, RefCell, RefMut},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(Debug)]
pub struct Track {
    inner: State<3>,
    output: RefCell<File>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            inner: State::default(),
            output: RefCell::new(open_track().unwrap()),
        }
    }
}

impl MolecularState<3> for Track {
    fn get_pos(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_pos()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_vel()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_acc()
    }

    fn sync(&self, time_now: Real) {
        let json = serde_json::to_string(&self.inner).unwrap();
        writeln!(self.output.borrow_mut(), "{}. {}", time_now, json);
    }
}

impl Track {
    pub fn restore_from<P: AsRef<Path>>(path: P) -> Result<Self, Self> {
        let input = OpenOptions::new().read(true).open(path)?;
        let mut last_line = last_line_of_file(input).ok_or_else(Self::default)?;
        let end_of_track = last_line.split(". ").last().unwrap_or_default();
        if end_of_track.is_empty() {
            return Err(Self::default());
        }
        let last_state: State<3> =
            serde_json::from_str(end_of_track).map_err(|_| Self::default())?;
        Ok(Self {
            inner: last_state,
            output: RefCell::new(open_track()?),
        })
    }
}

impl From<std::io::Error> for Track {
    fn from(_: std::io::Error) -> Self {
        Self::default()
    }
}

fn open_track() -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("track.txt")
}

pub(crate) fn last_line_of_file(f: File) -> Option<String> {
    BufReader::new(f).lines().flatten().last()
}
