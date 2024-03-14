extern crate ndarray;
extern crate ndarray_linalg;
// Imports
mod analytical_solution;
mod element_routine;
pub mod input_parameters;
mod material_routine;
mod mesh_generator;
mod solver;
fn main() {
    let radial_nodes: Vec<f64> = mesh_generator::generate();
    println!("The nodes are: {:?}", radial_nodes);
    let connectivity: std::collections::HashMap<u32, (usize, usize)> = mesh_generator::generate_connectivity();
    println!("The connectivity is: {:?}", connectivity);
    let _exact_solution: Vec<f64> = analytical_solution::compute(radial_nodes.clone());
    println!("The exact displacements are: {:?}", _exact_solution);
    let mat_size: usize;
    match <u32 as TryInto<usize>>::try_into(input_parameters::N_ELEMENTS) {
        Ok(u) => {
            mat_size = u;
            solver::solver(mat_size, radial_nodes, connectivity);
        }
        Err(_) => {
            println!("Conversion to size failed.");
        }
    }
}
