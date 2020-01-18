use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

extern crate hex;
use sha2::{Digest, Sha256};

#[pymodule]
fn fizz_buzzer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fizz_buzz))?;

    Ok(())
}

#[pyfunction]
fn fizz_buzz(n: i32) -> PyResult<String> {
    let mut res = "".to_string();
    for i in 0..n {
        if i % 15 == 0 {
            res.push_str("FizzBuzz");
        } else if i % 3 == 0 {
            res.push_str("Fizz");
        } else if i % 5 == 0 {
            res.push_str("Buzz");
        } else {
            res.push_str(&i.to_string());
        }
    }
    Ok(hash(&res))
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_bytes());
    hex::encode(hasher.result())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            "b62853f2ca7c728276460c30a3a5dce856654b23426f87f218b2c2c2c47a9d25",
            fizz_buzz(1000).unwrap()
        );
    }
}
