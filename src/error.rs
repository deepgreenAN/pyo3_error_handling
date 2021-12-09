#[derive(thiserror::Error, Debug)]
pub enum MathError {
    #[error(transparent)]
    LogSmallError(#[from] LogSmallError),

    #[error(transparent)]
    ExpLargeError(#[from] ExpLargeError)
}

#[derive(thiserror::Error, Debug)]
#[error("too small x for log(x) x:{0}. (expected x > 0.01)")]
pub struct LogSmallError(pub f64);

#[derive(thiserror::Error, Debug)]
#[error("too large x for exp(x) x:{0}. (expected x < 10)")]
pub struct ExpLargeError(pub f64);

#[derive(thiserror::Error, Debug)]
#[error("too small abs y for x / y y:{0}. (expected y > 0.001)")]
pub struct DivSmallAbsError(pub f64);

