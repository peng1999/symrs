use std::sync::Mutex;
use std::fmt::{self, Display};
use symtern::prelude::*;
use symtern::{Pool, Sym};
use symtern::adaptors::{self, Inline};
use sym::Expr;

type InlinePool = Inline<Pool<str, u32>>;
type InlineSym = adaptors::InlineSym<Sym<u32>>;

lazy_static! {
    static ref SYMPOOL: Mutex<InlinePool> = Mutex::new(Inline::from(Pool::<str, u32>::new()));
}

/// The symbol type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Symbol(InlineSym);

impl Symbol {
    /// Construct a new symbol.
    pub fn new(s: &str) -> Symbol {
        let sym = SYMPOOL.lock().unwrap().intern(s).unwrap();
        Symbol(sym)
    }

    /// Construct a power expression.
    ///
    /// # Examples
    /// ```
    /// use symrs::sym::{Expr, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// assert_eq!(x.pow(2), Expr::pow(x, 2));
    /// ```
    /// 
    pub fn pow<E:Into<Expr>>(self, rhs: E) -> Expr {
        Expr::pow(self.into(), rhs)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            SYMPOOL
                .lock()
                .map_err(|_| fmt::Error)?
                .resolve(&self.0)
                // The symbol is put into the pool when it's created, so it's safe to unwrap.
                .unwrap()
        )
    }
}
