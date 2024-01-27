use pyo3::prelude::*;

extern "C" {
    fn c_double_input(input: libc::c_int) -> libc::c_int;
    fn c_po_file_read(filename: *const libc::c_char) -> *mut libc::c_void;
    fn c_po_message_iterator(file: *mut libc::c_void) -> *mut libc::c_void;
    fn c_po_next_message(iterator: *mut libc::c_void) -> *mut libc::c_void;
    fn c_po_message_msgid(message: *mut libc::c_void) -> *mut libc::c_char;
    fn c_po_message_msgstr(message: *mut libc::c_void) -> *mut libc::c_char;
}

#[pyclass(unsendable)]
struct PoFile (*mut libc::c_void);

#[pyclass(unsendable)]
struct PoMessageIterator (*mut libc::c_void);

#[pyclass(unsendable)]
struct PoMessage (*mut libc::c_void);

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

#[pyfunction]
fn po_file_read(filename: &str) -> PyResult<PoFile> {
    unsafe {
        let c_filename = std::ffi::CString::new(filename).unwrap();
        let res = c_po_file_read(c_filename.as_ptr());
        Ok(PoFile(res.as_mut().unwrap()))
    }
}

#[pyfunction]
fn po_message_iterator(file: &PoFile) -> PyResult<PoMessageIterator> {
    unsafe {
        let res = c_po_message_iterator(file.0);
        Ok(PoMessageIterator(res.as_mut().unwrap()))
    }
}

#[pyfunction]
fn po_next_message(iterator: &PoMessageIterator) -> PyResult<PoMessage> {
    unsafe {
        let res = c_po_next_message(iterator.0);
        Ok(PoMessage(res.as_mut().unwrap()))
    }
}

#[pyfunction]
fn po_message_msgid(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let res = c_po_message_msgid(message.0);
        let c_str = std::ffi::CStr::from_ptr(res);
        let str_slice = c_str.to_str().unwrap();
        Ok(str_slice.to_string())
    }
}

#[pyfunction]
fn po_message_msgstr(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let res = c_po_message_msgstr(message.0);
        let c_str = std::ffi::CStr::from_ptr(res);
        let str_slice = c_str.to_str().unwrap();
        Ok(str_slice.to_string())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_gettext_viewer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double_input, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_read, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_iterator, m)?)?;
    m.add_function(wrap_pyfunction!(po_next_message, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgid, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgstr, m)?)?;
    Ok(())
}
