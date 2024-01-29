extern crate ndarray;
// Imports
pub mod input_parameters;
mod analytical_solution;
mod mesh_generator;
fn main() {
    let _radial_nodes: Vec<f64> = mesh_generator::generate();
    println!("The nodes are: {:?}", _radial_nodes);
    let _exact_solution: Vec<f64> = analytical_solution::compute(_radial_nodes);
    println!("The exact displacements are: {:?}", _exact_solution);
}
