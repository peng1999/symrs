//! Test utilities.

use sym::Expr;

/// A helper function to convert `Expr` to f64.
pub fn as_f64(e: &Expr) -> f64 {
    match *e {
        Expr::Approx(f) => f,
        _ => panic!("{} is not a float", e),
    }
}

/// assert_finished_and_near(IResult<T, Expr>, f64);
macro_rules! assert_finished_and_near {
    ($e:expr, $o:expr) => {
        {
            use ::nom::IResult;
            use ::float_cmp::ApproxEqRatio;
            use $crate::test_util::as_f64;

            if let IResult::Done(i, o) = $e {
                assert_empty!(i);
                assert!(as_f64(&o).approx_eq_ratio(&$o, 1e-6));
            } else {
                panic!("parser did not complete");
            }
        }
    }
}
