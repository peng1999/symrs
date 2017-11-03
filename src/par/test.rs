use nom::IResult;
use sym::Symbol;
use super::*;

// Just to assume the types are right.
const _ASSERT_TYPE: &[fn(&str) -> IResult<&str, Expr>] = &[integer, float, symbol];

// const INTEGER_ASSERT_DATA: &[(&str, Expr)] =
//     &[("123", Expr::integer(123)), ("-123", Expr::integer(-123))];
//
// macro_rules! assert_all_eq {
//     ($data:expr, $parser:expr) => {
//         for (i, o) in $data {
//             let result = $parser(i);
//             println!("{}", result);
//             assert_finished_and_eq!(result, o);
//         }
//     }
// }

macro_rules! assert_integers {
    ($parser:expr) => {
        assert_finished_and_eq!($parser("123"), Expr::integer(123));
        assert_finished_and_eq!($parser("-123"), Expr::integer(-123));
    }
}

#[test]
fn integer_works() {
    assert_integers!(integer);
}

macro_rules! assert_floats {
    ($parser:expr) => {
        assert_finished_and_near!($parser("123.3"), 123.3);
        assert_finished_and_near!($parser("123.3e3"), 123.3e3);
        assert_finished_and_near!($parser("-123.3"), -123.3);
        assert_finished_and_near!($parser("-123.3e3"), -123.3e3);
    }
}

#[test]
fn float_works() {
    assert_floats!(float);
}

macro_rules! assert_symbols {
    ($parser:expr) => {
        assert_finished_and_eq!($parser("x"), Expr::symbol("x"));
        assert_finished_and_eq!($parser("x1"), Expr::symbol("x1"));
        assert_finished_and_eq!($parser("_x"), Expr::symbol("_x"));
        assert_finished_and_eq!($parser("x_1"), Expr::symbol("x_1"));
    }
}

#[test]
fn symbol_works() {
    assert_symbols!(symbol);
}

#[test]
fn primitive_works() {
    assert_integers!(primitive);
    assert_floats!(primitive);
    assert_symbols!(primitive);
}

#[test]
fn expr_works() {
    assert_integers!(expr);
    assert_floats!(expr);
    assert_symbols!(expr);

    assert_finished_and_eq!(expr("(123)"), Expr::integer(123));
    assert_finished_and_eq!(expr("( 123 )"), Expr::integer(123));
    assert_finished_and_eq!(expr("((123))"), Expr::integer(123));

    let x = Symbol::new("x");
    let y = Symbol::new("y");
    // println!("{:?}", expr("2 * x"));
    assert_finished_and_eq!(expr("2*x"), 2i32 * x);
    assert_finished_and_eq!(expr("2 * x"), 2i32 * x);
    assert_finished_and_eq!(expr("x *x"), x * x);
    assert_finished_and_eq!(
        expr("2 * x * 5"),
        Expr::Product(vec![
            Expr::integer(2),
            Expr::symbol("x"),
            Expr::integer(5),
        ]));
    assert_finished_and_eq!(expr("2 * x / y"), 2i32 * x / y);
    assert_finished_and_eq!(expr("2 * (x / y)"), 2i32 * (x / y));
    assert_finished_and_eq!(expr("2 * (x * y)"), 2i32 * (x * y));
    assert_finished_and_eq!(expr("(2 * x) * y"), (2i32 * x) * y);
}

