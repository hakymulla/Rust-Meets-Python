use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs;

/// Takes a file path as input and return the count of each words in the file
#[pyfunction]
fn count_words(file_path:String) -> Py<PyAny> {
    let file = fs::read_to_string(file_path)
    .expect("Was not able to read file",);

    let dict = wordcount(&file);
   
    return  Python::with_gil(|py: Python| {
        dict.to_object(py)
    });
}

fn wordcount(file: &str) -> HashMap<&str, u8> {
    let mut hashmap = HashMap::new();
    for row in file.split(" "){
        let count = hashmap.entry(row).or_insert(0);
        *count += 1;
    }
    hashmap
}

/// A Python module implemented in Rust.
#[pymodule]
fn word_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
