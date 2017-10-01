use super::Expr;
use super::Symbol;
use num::{BigInt, BigUint};
use std::ops::{Add, Sub, Mul, Div};

// impls for operators

/// Expand to the functuon details in implaccording to the given trait.
macro_rules! op_func_impl {
    // Add: self + rhs
    (Add, $type_:ty) => {
        fn add(self, rhs: $type_) -> Self::Output {
            $crate::sym::Expr::Sum(vec![self.into(), rhs.into()])
        }
    };
    // Mul: self * rhs
    (Mul, $type_:ty) => {
        fn mul(self, rhs: $type_) -> Self::Output {
            $crate::sym::Expr::Product(vec![self.into(), rhs.into()])
        }
    };
    // Div: self / rhs
    (Div, $type_:ty) => {
        fn div(self, rhs: $type_) -> Self::Output {
            $crate::sym::Expr::Ratio(Box::new(self.into()), Box::new(rhs.into()))
        }
    };
    // There isn't a minus constructor, should add a negative.
    // Sub: self + (-rhs)
    (Sub, $type_:ty) => {
        fn sub(self, rhs: $type_) -> Self::Output {
            $crate::sym::Expr::Sum(vec![self.into(), $crate::sym::Expr::negative(rhs)])
        }
    };
}

/// Expand to operator impls.
macro_rules! op_impls {
    // The form `T op Expr/Symbol`.
    (impl<T: Into<Expr>> $trait:ident<T> for $lhs_t:ty; $($rest:tt)* ) => {
        impl<T: Into<Expr>> $trait<T> for $lhs_t {
            type Output = Expr;
            op_func_impl! {$trait, T}
        }
        op_impls! {$($rest)*}
    };
    // The form `Expr/Symbol op T`.
    (impl $trait:ident<$rhs_t:ty> for $type_:ty; $($rest:tt)*) => {
        impl $trait<$rhs_t> for $type_ {
            type Output = Expr;
            op_func_impl! {$trait, $rhs_t}
        }
        op_impls! {$($rest)*}
    };
    // End point for recursion.
    () => {};
}

// Split into multiple blocks due to the recursion limit.
// Expr op T
// Symbol op T
op_impls! {
    impl<T: Into<Expr>> Add<T> for Expr;
    impl<T: Into<Expr>> Mul<T> for Expr;
    impl<T: Into<Expr>> Div<T> for Expr;
    impl<T: Into<Expr>> Sub<T> for Expr;

    impl<T: Into<Expr>> Add<T> for Symbol;
    impl<T: Into<Expr>> Mul<T> for Symbol;
    impl<T: Into<Expr>> Div<T> for Symbol;
    impl<T: Into<Expr>> Sub<T> for Symbol;
}

// T op Expr
op_impls! {
	impl Add<Expr> for u8;
	impl Sub<Expr> for u8;
	impl Mul<Expr> for u8;
	impl Div<Expr> for u8;
	impl Add<Expr> for u64;
	impl Sub<Expr> for u64;
	impl Mul<Expr> for u64;
	impl Div<Expr> for u64;
	impl Add<Expr> for i16;
	impl Sub<Expr> for i16;
	impl Mul<Expr> for i16;
	impl Div<Expr> for i16;
	impl Add<Expr> for isize;
	impl Sub<Expr> for isize;
	impl Mul<Expr> for isize;
	impl Div<Expr> for isize;
	impl Add<Expr> for i32;
	impl Sub<Expr> for i32;
	impl Mul<Expr> for i32;
	impl Div<Expr> for i32;
	impl Add<Expr> for i64;
	impl Sub<Expr> for i64;
	impl Mul<Expr> for i64;
	impl Div<Expr> for i64;
	impl Add<Expr> for u16;
	impl Sub<Expr> for u16;
	impl Mul<Expr> for u16;
	impl Div<Expr> for u16;
	impl Add<Expr> for i8;
	impl Sub<Expr> for i8;
	impl Mul<Expr> for i8;
	impl Div<Expr> for i8;
	impl Add<Expr> for usize;
	impl Sub<Expr> for usize;
	impl Mul<Expr> for usize;
	impl Div<Expr> for usize;
	impl Add<Expr> for u32;
	impl Sub<Expr> for u32;
	impl Mul<Expr> for u32;
	impl Div<Expr> for u32;
	impl Add<Expr> for BigUint;
	impl Sub<Expr> for BigUint;
	impl Mul<Expr> for BigUint;
	impl Div<Expr> for BigUint;
	impl Add<Expr> for BigInt;
	impl Sub<Expr> for BigInt;
	impl Mul<Expr> for BigInt;
	impl Div<Expr> for BigInt;
	impl Add<Expr> for f32;
	impl Sub<Expr> for f32;
	impl Mul<Expr> for f32;
	impl Div<Expr> for f32;
	impl Add<Expr> for f64;
	impl Sub<Expr> for f64;
	impl Mul<Expr> for f64;
	impl Div<Expr> for f64;
}

// T op Symbol
op_impls! {
	impl Add<Symbol> for u8;
	impl Sub<Symbol> for u8;
	impl Mul<Symbol> for u8;
	impl Div<Symbol> for u8;
	impl Add<Symbol> for u64;
	impl Sub<Symbol> for u64;
	impl Mul<Symbol> for u64;
	impl Div<Symbol> for u64;
	impl Add<Symbol> for i16;
	impl Sub<Symbol> for i16;
	impl Mul<Symbol> for i16;
	impl Div<Symbol> for i16;
	impl Add<Symbol> for isize;
	impl Sub<Symbol> for isize;
	impl Mul<Symbol> for isize;
	impl Div<Symbol> for isize;
	impl Add<Symbol> for i32;
	impl Sub<Symbol> for i32;
	impl Mul<Symbol> for i32;
	impl Div<Symbol> for i32;
	impl Add<Symbol> for i64;
	impl Sub<Symbol> for i64;
	impl Mul<Symbol> for i64;
	impl Div<Symbol> for i64;
	impl Add<Symbol> for u16;
	impl Sub<Symbol> for u16;
	impl Mul<Symbol> for u16;
	impl Div<Symbol> for u16;
	impl Add<Symbol> for i8;
	impl Sub<Symbol> for i8;
	impl Mul<Symbol> for i8;
	impl Div<Symbol> for i8;
	impl Add<Symbol> for usize;
	impl Sub<Symbol> for usize;
	impl Mul<Symbol> for usize;
	impl Div<Symbol> for usize;
	impl Add<Symbol> for u32;
	impl Sub<Symbol> for u32;
	impl Mul<Symbol> for u32;
	impl Div<Symbol> for u32;
	impl Add<Symbol> for BigUint;
	impl Sub<Symbol> for BigUint;
	impl Mul<Symbol> for BigUint;
	impl Div<Symbol> for BigUint;
	impl Add<Symbol> for BigInt;
	impl Sub<Symbol> for BigInt;
	impl Mul<Symbol> for BigInt;
	impl Div<Symbol> for BigInt;
	impl Add<Symbol> for f32;
	impl Sub<Symbol> for f32;
	impl Mul<Symbol> for f32;
	impl Div<Symbol> for f32;
	impl Add<Symbol> for f64;
	impl Sub<Symbol> for f64;
	impl Mul<Symbol> for f64;
	impl Div<Symbol> for f64;
}

// impls for conversions

/// Expand to conversion impls.
macro_rules! convs {
    ( $(impl From<$from_t:ty> for Expr using $method:ident;)* ) => {
        $(
            impl From<$from_t> for Expr {
                fn from(x: $from_t) -> Expr {
                    $crate::sym::Expr::$method(x)
                }
            }
        )*
    }
}

convs! {
    impl From<u8> for Expr using integer;
    impl From<u64> for Expr using integer;
    impl From<i16> for Expr using integer;
    impl From<isize> for Expr using integer;
    impl From<i32> for Expr using integer;
    impl From<i64> for Expr using integer;
    impl From<u16> for Expr using integer;
    impl From<i8> for Expr using integer;
    impl From<usize> for Expr using integer;
    impl From<u32> for Expr using integer;
    impl From<BigUint> for Expr using integer;
    impl From<BigInt> for Expr using integer;

    impl From<f32> for Expr using approximate;
    impl From<f64> for Expr using approximate;
}

impl From<Symbol> for Expr {
    fn from(s: Symbol) -> Expr {
        Expr::Sym(s)
    }
}
