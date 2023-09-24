mod iec60909_4_2;
mod iec60909_4_3;
mod iec60909_4_4;
mod iec60909_4_5;
mod iec60909_4_6;

mod impedance_test;

#[macro_export]
macro_rules! assert_cmplx_eq {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        {
        approx::assert_abs_diff_eq!($given.re, $expected.re $(, $opt = $val)*);
        approx::assert_abs_diff_eq!($given.im, $expected.im $(, $opt = $val)*);
        }
    };
}
