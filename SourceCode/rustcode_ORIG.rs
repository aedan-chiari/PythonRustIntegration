use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

// This module integrates Python with Rust and provides several functions.
#[pymodule]
fn rust_python_integration(_py: Python, m: &PyModule) -> PyResult<()> {
    // Prints "Hello World" and returns the string.
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    // Prints "Exit All Done" and returns the string.
    m.add_function(wrap_pyfunction!(exit_world, m)?)?;
    // Generates two random one-dimensional arrays of integers and returns them.
    m.add_function(wrap_pyfunction!(generate_random_one_d_array, m)?)?;
    // Calculates the total sum of two given arrays and returns the sum.
    m.add_function(wrap_pyfunction!(calculate_total_sum, m)?)?;
    // Generates an array where each element is the sum of the corresponding elements in the input arrays plus the sum of all previous elements.
    m.add_function(wrap_pyfunction!(generate_increment_sum, m)?)?;
    Ok(())
}

// Prints "Hello World" and returns the string.
#[pyfunction]
fn hello_world() -> PyResult<String> {
    let str_one: &str = "Hello World";
    println!("{}", str_one);
    Ok(str_one.to_string())
}

// Prints "Exit All Done" and returns the string.
#[pyfunction]
fn exit_world() -> PyResult<String> {
    let str_one: &str = "Exit All Done";
    println!("{}", str_one);
    Ok(str_one.to_string())
}

// Generates two random one-dimensional arrays of integers and returns them.
#[pyfunction]
fn generate_random_one_d_array() -> PyResult<(Vec<i32>, Vec<i32>)> {
    let mut rng = rand::thread_rng();
    let mut array1: Vec<i32> = vec![0; 10];
    let mut array2: Vec<i32> = vec![0; 10];

    for i in 0..10 {
        array1[i] = rng.gen_range(0..10);
        array2[i] = rng.gen_range(0..10);
    }
    println!("Array1 {:?}", array1);
    println!("Array2 {:?}", array2);
    Ok((array1, array2))
}

// Calculates the total sum of two given arrays and returns the sum.
#[pyfunction]
fn calculate_total_sum(array1: Vec<i32>, array2: Vec<i32>) -> PyResult<i32> {
    let array1_sum: i32 = array1.iter().sum();
    let array2_sum: i32 = array2.iter().sum();
    let total_sum: i32 = array1_sum + array2_sum;
    println!("Total Sum: {}", total_sum);
    Ok(total_sum)
}

// Generates an array where each element is the sum of the corresponding elements in the input arrays plus the sum of all previous elements.
#[pyfunction]
fn generate_increment_sum(array1: Vec<i32>, array2: Vec<i32>) -> PyResult<Vec<i32>> {
    let mut increment_sum_array: Vec<i32> = vec![0; array1.len()];
    let mut ongoing_sum: i32 = 0;

    for i in 0..array1.len() {
        let iteration_sum: i32 = array1[i] + array2[i];
        increment_sum_array[i] = iteration_sum + ongoing_sum;
        ongoing_sum += iteration_sum;
    }
    println!("Increment Sum: {:?}", increment_sum_array);

    Ok(increment_sum_array)
}