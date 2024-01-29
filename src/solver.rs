use std::os::unix::process;

pub use crate::input_parameters;
use ndarray::prelude::*;
use ndarray_linalg::*;
fn generate_time_span() -> Vec<f64> {
    let start: f64 = input_parameters::T_START.into();
    let stop: f64 = input_parameters::T_END.into();
    let step: f64 = input_parameters::T_STEP.into();
    let mut current = start;
    let mut result: Vec<f64> = Vec::new();
    while current < stop {
        result.push(current);
        current += step;
    }
    result
}

pub fn solver(mat_size: usize) {
    let mut _load_scale: f64 = 0.0;
    let mut load: f64 = 0.0;
    let time_span = generate_time_span();
    for (_, &t_step) in time_span.iter().enumerate() {
        // TIME LOOP
        if t_step <= input_parameters::T_LOAD_END {
            _load_scale = (1.0/input_parameters::T_LOAD_END) * t_step;
            load = <u32 as Into<f64>>::into(input_parameters::P_MAX * 
                input_parameters::INNER_RADIUS) * _load_scale;
        } else {
            load = <u32 as Into<f64>>::into(input_parameters::P_MAX * 
                input_parameters::INNER_RADIUS);
        }
        let mut newton_iterations:u32 = 0;
        while newton_iterations <= input_parameters::MAX_ITERATIONS {
            // NEWTON LOOP
            let mut e_stiffness: Array2<f64> = Array2::zeros((2,2));
            let mut g_stiffness: Array2<f64> = Array2::zeros((mat_size+1, mat_size+1));
            let mut e_forces: Array1<f64> = Array1::zeros(2);
            let mut g_int_forces: Array1<f64> = Array1::zeros(mat_size+1);
            let mut g_ext_forces: Array1<f64> = Array1::zeros(mat_size+1);
            let mut delta_u:Array1<f64> = Array1::zeros(mat_size+1);
            g_ext_forces[0] = load;
            let mut sigma_evolution: Vec<f64> = Vec::new();
            // Element routine
            // Solver
            let g_forces: Array1<f64> = g_ext_forces - g_int_forces;
            match g_stiffness.solve(&g_forces) {
                Ok(x) => {
                    delta_u = x;
                }
                Err(e) => {
                    panic!("Panic. Error: {}", e);
                }
            }
            // let _x = g_stiffness.solve(&g_forces);
        }
    }
}