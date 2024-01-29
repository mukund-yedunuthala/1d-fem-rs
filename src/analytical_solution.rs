use crate::input_parameters::*;

extern crate ndarray;

fn exact_solution(r: f64) -> f64 {
    let _factor_1: f64 = 1.0 + NU;
    let _factor_2: f64 = <u32 as Into<f64>>::into(P_MAX) / E;
    let _factor_3: f64 = <u32 as Into<f64>>::into(INNER_RADIUS.pow(2))
        / <u32 as Into<f64>>::into(OUTER_RADIUS.pow(2) - INNER_RADIUS.pow(2));
    let _factor_4: f64 = ((1.0 - 2.0 * NU) * r) + (<u32 as Into<f64>>::into(OUTER_RADIUS) / r);
    let exact_displacement = _factor_1 * _factor_2 * _factor_3 * _factor_4;
    exact_displacement
}

pub fn compute(radial_nodes: Vec<f64>) -> Vec<f64> {
    let mut solution: Vec<f64> = vec![];
    for (_, &radius) in radial_nodes.iter().enumerate() {
        let exact_displacement: f64 = exact_solution(radius);
        solution.push(exact_displacement);
    }
    solution
}
