pub mod error;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::create_exception;

// PyErrとしてMathErrorを定義
create_exception!(error_handling, MathError, pyo3::exceptions::PyException);

create_exception!(error_handling, LogSmallError, MathError);
create_exception!(error_handling, ExpLargeError, MathError);


// crate::error::ErrorをPyErrに変換できるようにする(pyfunctionが認識できる)
impl From<crate::error::MathError> for PyErr {
    fn from(err: crate::error::MathError) -> PyErr {
        MathError::new_err(err.to_string())
    }
}

impl From<crate::error::LogSmallError> for PyErr {
    fn from(err: crate::error::LogSmallError) -> PyErr {
        LogSmallError::new_err(err.to_string())
    }
}


impl From<crate::error::ExpLargeError> for PyErr {
    fn from(err: crate::error::ExpLargeError) -> PyErr {
        ExpLargeError::new_err(err.to_string())
    }
}

impl From<crate::error::DivSmallAbsError> for PyErr {
    fn from(err: crate::error::DivSmallAbsError) -> PyErr {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}


fn log(x: f64) -> Result<f64, crate::error::LogSmallError> {
    if x < 0.01 {
        return Err(crate::error::LogSmallError(x));
    } else {
        return Ok(x.ln());
    }
}

fn exp(x: f64) -> Result<f64, crate::error::ExpLargeError> {
    if x > 10.0 {
        return Err(crate::error::ExpLargeError(x));
    } else {
        return Ok(x.exp());
    }
}

fn div(x: f64, y: f64) -> Result<f64, crate::error::DivSmallAbsError> {
    if y < 0.001 {
        return Err(crate::error::DivSmallAbsError(y));
    } else {
        return Ok(x / y);
    }
}

fn log_with_exp(x: f64) -> Result<(f64, f64), crate::error::MathError> {
    let log_x = log(x)?;
    let exp_x = exp(x)?;
    Ok((log_x, exp_x))
}

#[pyfunction]
fn rust_log(x: f64) -> PyResult<f64> {
    Ok(log(x)?)
}

#[pyfunction]
fn rust_exp(x: f64) -> PyResult<f64> {
    Ok(exp(x)?)
}

#[pyfunction]
fn rust_div(x: f64, y: f64) -> PyResult<f64> {
    Ok(div(x, y)?)
}

#[pyfunction]
fn rust_log_with_exp(x: f64) -> PyResult<(f64, f64)> {
    let log_x = log(x)?;
    let exp_x = exp(x)?;
    Ok((log_x, exp_x))
}

#[pyfunction]
fn rust_log_with_exp2(x: f64) -> PyResult<(f64, f64)> {
    Ok(log_with_exp(x)?)
}


#[pyfunction]
fn rust_handle_error(x: f64) -> i64 {
    let res = log_with_exp(x);
    if let Err(error) = res {
        match error {
            crate::error::MathError::LogSmallError(_) => { return 1;},
            crate::error::MathError::ExpLargeError(_) => { return 2;}
        }
    } else {
        return 0;
    }
}

#[pymodule]
fn error_handling(py: Python, m:&PyModule) -> PyResult<()> {
    m.add("MathError", py.get_type::<MathError>())?;
    m.add("LogSmallError", py.get_type::<LogSmallError>())?;
    m.add("ExpLargeError", py.get_type::<ExpLargeError>())?;

    m.add_function(wrap_pyfunction!(rust_log, m)?)?;
    m.add_function(wrap_pyfunction!(rust_exp, m)?)?;
    m.add_function(wrap_pyfunction!(rust_div, m)?)?;
    m.add_function(wrap_pyfunction!(rust_log_with_exp, m)?)?;
    m.add_function(wrap_pyfunction!(rust_log_with_exp2, m)?)?;
    m.add_function(wrap_pyfunction!(rust_handle_error, m)?)?;

    Ok(())
}