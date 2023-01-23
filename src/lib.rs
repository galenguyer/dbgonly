/*
 * The source code for the macro is taken from the dbg! macro, provided from the following source under the MIT license
 * https://github.com/rust-lang/rust/blob/master/library/std/src/macros.rs#L212-L361
 */

/// Prints and returns the value of a given expression for quick and dirty
/// debugging. This version of the macro will print nothing and be optmized
/// out in release builds.
///
/// An example:
///
/// ```rust
/// use dbgonly::dbgonly;
/// let a = 2;
/// let b = dbgonly!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// The macro works by using the `Debug` implementation of the type of
/// the given expression to print the value to [stderr] along with the
/// source location of the macro invocation as well as the source code
/// of the expression.
///
/// Invoking the macro on an expression moves and takes ownership of it
/// before returning the evaluated expression unchanged. If the type
/// of the expression does not implement `Copy` and you don't want
/// to give up ownership, you can instead borrow with `dbgonly!(&expr)`
/// for some expression `expr`.
///
/// The `dbgonly!` macro is optimized out in release builds.
///
/// Note that the macro is intended as a debugging tool and therefore you
/// should avoid having uses of it in version control for long periods
/// (other than in tests and similar).
/// Debug output from production code is better done with other facilities
/// such as the [`debug!`] macro from the [`log`] crate.
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon
/// and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Further examples
///
/// With a method call:
///
/// ```rust
/// use dbgonly::dbgonly;
/// fn foo(n: usize) {
///     if let Some(_) = dbgonly!(n.checked_sub(4)) {
///         // ...
///     }
/// }
///
/// foo(3)
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:4] n.checked_sub(4) = None
/// ```
///
/// Naive factorial implementation:
///
/// ```rust
/// use dbgonly::dbgonly;
/// fn factorial(n: u32) -> u32 {
///     if dbgonly!(n <= 1) {
///         dbgonly!(1)
///     } else {
///         dbgonly!(n * factorial(n - 1))
///     }
/// }
///
/// dbgonly!(factorial(4));
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = true
/// [src/main.rs:4] 1 = 1
/// [src/main.rs:5] n * factorial(n - 1) = 2
/// [src/main.rs:5] n * factorial(n - 1) = 6
/// [src/main.rs:5] n * factorial(n - 1) = 24
/// [src/main.rs:11] factorial(4) = 24
/// ```
///
/// The `dbgonly!(..)` macro moves the input:
///
/// ```compile_fail
/// use dbgonly::dbgonly;
/// /// A wrapper around `usize` which importantly is not Copyable.
/// #[derive(Debug)]
/// struct NoCopy(usize);
///
/// let a = NoCopy(42);
/// let _ = dbgonly!(a); // <-- `a` is moved here.
/// let _ = dbgonly!(a); // <-- `a` is moved again; error!
/// ```
///
/// You can also use `dbgonly!()` without a value to just print the
/// file and line whenever it's reached.
///
/// Finally, if you want to `dbgonly!(..)` multiple values, it will treat them as
/// a tuple (and return it, too):
///
/// ```
/// use dbgonly::dbgonly;
/// assert_eq!(dbgonly!(1usize, 2u32), (1, 2));
/// ```
///
/// However, a single argument with a trailing comma will still not be treated
/// as a tuple, following the convention of ignoring trailing commas in macro
/// invocations. You can use a 1-tuple directly if you need one:
///
/// ```
/// use dbgonly::dbgonly;
/// assert_eq!(1, dbgonly!(1u32,)); // trailing comma ignored
/// assert_eq!((1,), dbgonly!((1u32,))); // 1-tuple
/// ```
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbgonly {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbgonly!($val)),+,)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbgonly {
    () => {};
    ($val:expr $(,)?) => {
        match $val {
            tmp => tmp
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbgonly!($val)),+,)
    };
}
