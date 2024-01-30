pub use crate::input_parameters::*;
use ndarray::{prelude::*, Zip};
use ndarray_linalg::*;

fn generate_time_span() -> Vec<f64> {
    let start: f64 = T_START.into();
    let stop: f64 = T_END.into();
    let step: f64 = T_STEP.into();
    let mut current = start;
    let mut result: Vec<f64> = Vec::new();
    while current < stop {
        result.push(current);
        current += step;
    }
    result
}

pub fn solver(mat_size: usize, radial_nodes: Vec<f64>) {
    // Initial variables
    let mut _load_scale: f64;
    let mut load: f64;
    let mut sigma_ov: Array1<f64> = Array1::zeros(2);
    let mut sigma: Array1<f64>;
    let mut strain: Array1<f64>;
    let mut pl_sig_ov: Array1<f64> = Array1::zeros(2); //Placeholder
    let mut g_displacements: Array1<f64> = Array1::zeros(mat_size + 1);
    let mut pl_disp: Array1<f64> = Array1::zeros(mat_size + 1);
    let mut displacement_evolution: Vec<f64> = Vec::new();

    let time_span = generate_time_span();
    // Solver
    for (_, &t_step) in time_span.iter().enumerate() {
        // TIME LOOP
        if t_step <= T_LOAD_END {
            _load_scale = (1.0 / T_LOAD_END) * t_step;
            load = <u32 as Into<f64>>::into(P_MAX * INNER_RADIUS) * _load_scale;
        } else {
            load = <u32 as Into<f64>>::into(P_MAX * INNER_RADIUS);
        }
        let mut newton_iterations: u32 = 0;
        while newton_iterations <= MAX_ITERATIONS {
            // NEWTON LOOP
            let mut e_stiffness: Array2<f64> = Array2::zeros((2, 2));
            let mut g_stiffness: Array2<f64> = Array2::zeros((mat_size + 1, mat_size + 1));
            let mut e_forces: Array1<f64> = Array1::zeros(2);
            let mut g_int_forces: Array1<f64> = Array1::zeros(mat_size + 1);
            let mut g_ext_forces: Array1<f64> = Array1::zeros(mat_size + 1);
            let mut delta_u: Array1<f64> = Array1::zeros(mat_size + 1);
            g_ext_forces[0] = load;
            let mut sigma_evolution: Vec<Array1<f64>> = Vec::new();
            // Element routine
            for element in 0..(mat_size) {
                let r1 = radial_nodes[element];
                let r2 = radial_nodes[element + 1];
                let mut r_nodes: Array1<f64> = Array1::zeros(2);
                r_nodes[0] = r1;
                r_nodes[1] = r2;
                let mut e_displacement: Array1<f64> = Array1::zeros(2);
                e_displacement[0] = g_displacements[element];
                e_displacement[1] = g_displacements[element + 1];
                let mut delta_u_element: Array1<f64> = Array1::zeros(2);
                delta_u_element[0] = pl_disp[element];
                delta_u_element[1] = pl_disp[element + 1];
                let prev_sigma_ov = pl_sig_ov.clone();
                let jacobian = (r2 - r1) / 2.0;
                let xi: f64 = N_INTEGRATION_POINTS.into();
                let mut shape_fn: Array1<f64> = Array1::zeros(2);
                shape_fn[0] = 0.5 * (1.0 - xi);
                shape_fn[1] = 0.5 * (1.0 + xi);
                let strain_disp_element1 = -1.0 / (2.0 * jacobian);
                let strain_disp_element2 = 1.0 / (2.0 * jacobian);
                let strain_disp_element3 = 1.0 / (r1 + r2);
                let strain_disp_element4 = 1.0 / (r1 + r2);
                let b_mat: Array2<f64> = Array::from_shape_vec(
                    (2, 2),
                    vec![
                        strain_disp_element1,
                        strain_disp_element2,
                        strain_disp_element3,
                        strain_disp_element4,
                    ],
                )
                .unwrap();
                strain = b_mat.dot(&e_displacement);
                let delta_strain = b_mat.dot(&delta_u_element);
                // Material Routine
                let mut c_t: Array2<f64> = C.clone();
                let del_sum = delta_strain.sum();
                let deviator = delta_strain - (del_sum / 3.0);
                sigma_ov = (1. / (1. + (T_STEP / (2. * TIME_SCALE))))
                    * (prev_sigma_ov * (1.0 - (T_STEP / (2.0 * TIME_SCALE))) + (Q * deviator));
                // Material tangent
                let quant_to_add_1 = Q / (1. + (T_STEP / (2. * TIME_SCALE)));
                let quant_to_add_2: Array2<f64> = array![[2. / 3., -1. / 3.], [-1. / 3., 2. / 3.]];
                c_t.mapv_inplace(|x| x + quant_to_add_1);
                let mut tangent_stiffness: Array2<f64> = c_t.clone();
                Zip::from(&mut tangent_stiffness)
                    .and(&quant_to_add_2)
                    .for_each(|x, &y| {
                        *x += y;
                    });
                sigma = sigma_ov.clone() + C.dot(&strain);
                // Element stiffness
                e_forces = 2.0 * b_mat.t().dot(&sigma) * shape_fn.t().dot(&r_nodes) * jacobian;
                e_stiffness = 2.0
                    * jacobian
                    * shape_fn.t().dot(&r_nodes)
                    * b_mat.t().dot(&(tangent_stiffness.dot(&b_mat)));
                // Assemble
                sigma_evolution.push(sigma);
            } // End element routine
              // Solver
            let g_forces: Array1<f64> = g_ext_forces - g_int_forces;
            match g_stiffness.solve(&g_forces) {
                Ok(x) => {
                    delta_u = x;
                    for (u, &del_u) in g_displacements.iter_mut().zip(delta_u.iter()) {
                        *u += del_u;
                    }
                    pl_disp = delta_u.clone();
                    // Newton convergence criteria
                    if 2 > 0 {
                    } else {
                        newton_iterations += 1;
                    }
                }
                Err(e) => {
                    panic!("Panic. Error: {}", e);
                }
            }
        } // End newton loop
        displacement_evolution.push(g_displacements[mat_size]);
        pl_sig_ov = sigma_ov.clone();
    }
}
