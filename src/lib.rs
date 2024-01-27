use pyo3::prelude::*;

extern "C" {
    fn c_double_input(input: libc::c_int) -> libc::c_int;
    // fn c_po_file_read(filename: *const libc::c_char) -> *mut libc::c_void;
    // fn c_po_message_iterator(file: *mut libc::c_void) -> *mut libc::c_void;
    // fn c_po_next_message(iterator: *mut libc::c_void) -> *mut libc::c_void;
    // fn c_po_message_msgid(message: *mut libc::c_void) -> *mut libc::c_char;
    // fn c_po_message_msgstr(message: *mut libc::c_void) -> *mut libc::c_char;
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double_input(inpt: usize) -> PyResult<usize> {
    unsafe {
        Ok(c_double_input(inpt as libc::c_int) as usize)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_gettext_viewer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double_input, m)?)?;
    Ok(())
}
