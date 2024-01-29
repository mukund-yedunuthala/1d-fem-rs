pub use crate::input_parameters;

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

pub fn solver() {
    let time_span = generate_time_span();
}