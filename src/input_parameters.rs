extern crate lazy_static;
extern crate ndarray;

use lazy_static::lazy_static;
use ndarray::Array2;

pub const INNER_RADIUS: u32 = 60; //mm
pub const OUTER_RADIUS: u32 = 10; //mm
pub const E: f64 = 7e4; // Mpa
pub const NU: f64 = 0.3; // Poisson's ratio
pub const Q: f64 = 35000.0; // Bulk modulus
pub const TIME_SCALE: u32 = 3; // Time span in s
pub const P_MAX: u32 = 50; // Pressure applied (upper limit)
pub const T_START: u32 = 0; // starting time
pub const T_LOAD_END: u32 = 6; // Time in s when loading stops
pub const T_END: u32 = 30; // Time when simulation ends in s
pub const T_STEP: f32 = 0.01; // Time step in s
pub const MAX_ITERATIONS: u32 = 20; // Newton iterations upper limit
pub const N_INTEGRATION_POINTS: u32 = 0; // 1-d simulation, integration points are unnecessary
pub const N_ELEMENTS: u32 = 10; // NUmber of elements in mesh. 
static MULT: f64 = E/((1.0+NU)*(1.0-2.0*NU));

lazy_static! {
    pub static ref C: Array2<f64> = {
        Array2::from_shape_fn((2,2), |(i,j)| match (i,j) {
            (0,0) => MULT * (1.0-NU),
            (0,1) => MULT * NU,
            (1,0) => MULT * NU,
            (1,1) => MULT * (1.0-NU),
            _ => unreachable!(),
        })
    };
}