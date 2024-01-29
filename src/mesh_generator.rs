pub use crate::input_parameters;
pub fn generate() -> Vec<f64> {
    let b: f64 = input_parameters::OUTER_RADIUS.into();
    let a: f64 = input_parameters::INNER_RADIUS.into();
    let n_elements:f64 = input_parameters::N_ELEMENTS.into();
    let mesh_refinement_factor: f64 = 2.0;
    // Ratio between elements in the geometric series
    let _exp:f64 = 1.0/(n_elements-1.0);
    let q: f64 = f64::powf(mesh_refinement_factor, _exp);
    // println!("{}",q);
    // factor
    let _num_1:f64 = (b-a) * (1.0-q);
    let _den_1:f64 = 1.0 - (mesh_refinement_factor * q);
    let mut d_r: f64 = _num_1 / _den_1; 
    // radial nodes array
    let mut r_node: f64 = a.into();
    let mut radial_nodes: Vec<f64> = vec![r_node];
    for _ in 0..input_parameters::N_ELEMENTS {
        r_node = r_node + d_r;
        radial_nodes.push(r_node);
        d_r = d_r * q;
    }
    radial_nodes
}