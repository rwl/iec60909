pub const SQRT_3: f64 = 1.732050807568877293527446341505872366942805253810380628055;

#[macro_export]
macro_rules! cmplx {
    () => {
        num_complex::Complex64::new(0.0, 0.0)
    };
    ($arg1:expr) => {
        num_complex::Complex64::new($arg1 as f64, 0.0)
    };
    ($arg1:expr, $arg2:expr) => {
        num_complex::Complex64::new($arg1 as f64, $arg2 as f64)
    };
}
