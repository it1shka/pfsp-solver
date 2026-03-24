use std::{collections::HashMap, str::FromStr};

use pfsp_solver::solver::algorithm::operators::{BinaryOperator, UnaryOperator};
use rand::Rng;

pub fn get_numeric_param<T: FromStr>(
    settings: &HashMap<String, String>,
    field_name: &str,
    default_value: T,
) -> T {
    settings
        .get(field_name)
        .map(|raw| raw.parse::<T>().ok())
        .flatten()
        .unwrap_or(default_value)
}

pub fn get_optional_numeric_param<T: FromStr>(
    settings: &HashMap<String, String>,
    field_name: &str,
    default_value: Option<T>,
) -> Option<T> {
    settings
        .get(field_name)
        .and_then(|raw| raw.parse::<T>().ok())
        .or(default_value)
}

pub fn add_unary_op<T: UnaryOperator<R> + 'static, R: Rng>(
    settings: &HashMap<String, String>,
    container: &mut Vec<Box<dyn UnaryOperator<R>>>,
    field_name: &str,
    default_value: f32,
) {
    let probability = get_numeric_param(settings, field_name, default_value);
    let operator = T::new(probability);
    container.push(Box::new(operator));
}

pub fn add_binary_op<T: BinaryOperator<R> + 'static, R: Rng>(
    settings: &HashMap<String, String>,
    container: &mut Vec<Box<dyn BinaryOperator<R>>>,
    field_name: &str,
    default_value: f32,
) {
    let probability = get_numeric_param(settings, field_name, default_value);
    let operator = T::new(probability);
    container.push(Box::new(operator));
}
