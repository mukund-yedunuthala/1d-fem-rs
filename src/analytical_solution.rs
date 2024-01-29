extern crate ndarray;

pub use crate::input_parameters::*;

use ndarray::s;

pub fn compute() {
    println!("Printing from analytical solution file! C = {}", C.slice(s![0,0]));
}