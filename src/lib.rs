use pyo3::prelude::*;

#[pyfunction]
fn calculate() -> PyResult<f64> {
    let mut product = 1.0;
    for c1 in 1..1000 {
        for c2 in 1..1000 {
            let c1 = c1 as f64;
            let c2 = c2 as f64;

            product = product + (c1 * c2 - c1.sin() + c2.cos() * c1.sqrt());
        }
    }
    return Ok(product);
}

#[pymodule]
fn rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    Ok(())
}