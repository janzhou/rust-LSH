#[cfg(test)]

extern crate lsh;
extern crate rulinalg;

use lsh::*;
use rulinalg::vector::Vector;

#[test]
fn it_works() {
    let seed = Vector::new(vec![1,2,3]);
    let lsh = LSH::new(seed);
    let data = Vector::new(vec![1,2,3]);
    assert!(lsh.hash(&data) == 14);
}
