use nom::IResult;
use sym::Symbol;
use super::*;

// Just to assume the types are right.
const _ASSERT_TYPE: &[fn(&str) -> IResult<&str, Expr>] = &[integer, float, symbol];

macro_rules! assert_floats {
    ($parser:expr) => {
        assert_finished_and_near!($parser("123.3"), 123.3);
        assert_finished_and_near!($parser("123.3e3"), 123.3e3);
        assert_finished_and_near!($parser("-123.3"), -123.3);
        assert_finished_and_near!($parser("-123.3e3"), -123.3e3);
    }
}

macro_rules! assert_integers {
    ($parser:expr) => {
        assert_finished_and_eq!($parser("123"), Expr::integer(123));
        assert_finished_and_eq!($parser("-123"), Expr::integer(-123));
    }
}

macro_rules! assert_symbols {
    ($parser:expr) => {
        assert_finished_and_eq!($parser("x"), Expr::symbol("x"));
        assert_finished_and_eq!($parser("x1"), Expr::symbol("x1"));
        assert_finished_and_eq!($parser("_x"), Expr::symbol("_x"));
        assert_finished_and_eq!($parser("x_1"), Expr::symbol("x_1"));
        assert_finished_and_eq!($parser("α_1"), Expr::symbol("α_1"));
    }
}

macro_rules! assert_negtive {
    ($parser:expr) => {{
        let x = Symbol::new("x");
        assert_finished_and_eq!($parser("-x"), - x);
        assert_finished_and_eq!($parser("- x"), - x);
        assert_finished_and_eq!($parser("-(x)"), - x);
        assert_finished_and_eq!($parser("-(12)"), - Expr::integer(12));
    }}
}

macro_rules! assert_pow {
    ($parser:expr) => {{
        let x = Symbol::new("x");
        assert_finished_and_eq!($parser("x^x"), x.pow(x));
        assert_finished_and_eq!($parser("x ^ x"), x.pow(x));
        assert_finished_and_eq!($parser("x ^ x^ x"), x.pow(x.pow(x)));
        assert_finished_and_eq!($parser("x ^x   ^ x"), x.pow(x.pow(x)));
        assert_finished_and_eq!($parser("(x ^ x) ^ x"), x.pow(x).pow(x));
        assert_finished_and_eq!($parser("(x ^ 2) ^ x"), x.pow(2).pow(x));
        // println!("{:?}", $parser("(x ^ -2) ^ - x"));
        assert_finished_and_eq!($parser("(x ^ -2) ^ - x"), x.pow(-2).pow(- x));
    }}
}

#[test]
fn integer_works() {
    assert_integers!(integer);
}

#[test]
fn float_works() {
    assert_floats!(float);
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
fn pow_works() {
    assert_integers!(pow);
    assert_floats!(pow);
    assert_symbols!(pow);
    assert_pow!(pow);
}

#[test]
fn negative_works() {
    assert_integers!(negative);
    assert_floats!(negative);
    assert_symbols!(negative);
    assert_pow!(negative);
    assert_negtive!(negative);
}

#[test]
fn parse_works() {
    assert_integers!(parse_expr);
    assert_floats!(parse_expr);
    assert_symbols!(parse_expr);
    assert_pow!(parse_expr);
    assert_negtive!(parse_expr);

    assert_finished_and_eq!(parse_expr("(123)"), Expr::integer(123));
    assert_finished_and_eq!(parse_expr("( 123 )"), Expr::integer(123));
    assert_finished_and_eq!(parse_expr("((123))"), Expr::integer(123));

    let x = Symbol::new("x");
    let y = Symbol::new("y");
    assert_finished_and_eq!(parse_expr("2*x"), 2i32 * x);
    assert_finished_and_eq!(parse_expr("2 * x"), 2i32 * x);
    assert_finished_and_eq!(parse_expr("x *x"), x * x);
    assert_finished_and_eq!(parse_expr("2 * x * 5"), 2 * x * 5);
    assert_finished_and_eq!(parse_expr("2 * x / y"), 2i32 * x / y);
    assert_finished_and_eq!(parse_expr("2 * (x / y)"), 2i32 * (x / y));
    assert_finished_and_eq!(parse_expr("2 * (x * y)"), 2i32 * (x * y));
    assert_finished_and_eq!(parse_expr("(2 * x) * y"), (2i32 * x) * y);
    assert_finished_and_eq!(parse_expr("2*-x"), 2i32 * - x);
    assert_finished_and_eq!(parse_expr("-2 * x"), -2i32 * x);
    println!("{:?}", parse_expr("2 * -(-x / - y)"));
    assert_finished_and_eq!(parse_expr("2 * -(-x / - y)"), 2i32 * - (- x / - y));
    assert_finished_and_eq!(parse_expr("2 * -(-x / 3 ^ - y)"), 2i32 * - (- x / Expr::pow(3.into(), - y)));
}

