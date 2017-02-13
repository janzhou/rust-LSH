extern crate num as libnum;
extern crate rulinalg;

use std::ops::{Mul, Add};
use libnum::Zero;
use rulinalg::vector::Vector;

pub struct LSH<T> {
    seed:Vector<T>,
}

impl<T> LSH<T> {
    pub fn new(data: Vector<T>) -> LSH<T> {
        LSH {
            seed: data
        }
    }

    /// Returns a non-mutable reference to the underlying data.
    pub fn seed(&self) -> &Vector<T> {
        &self.seed
    }

    /// Consumes the Vector and returns the Vec of data.
    pub fn into_vec(self) -> Vector<T> {
        self.seed
    }
}

impl<T: Copy + Zero + Mul<T, Output=T> + Add<T, Output=T>> LSH<T> {
    pub fn hash(&self, data: &Vector<T>) -> T {
        self.seed.dot(data)
    }

    pub fn into_hash(&self, data: Vector<T>) -> T {
        self.seed.dot(&data)
    }
}
