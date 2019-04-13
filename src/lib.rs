//! # Future Union
//! When you use impl traits, specifically with futures,
//! sometimes you will want to have a branching expression
//! (e.g. an `if` or `match`) in which the different branches
//! return different types that both impl Future. This does
//! not work since in current stable rust impl trait can only
//! refer to a single type.
//!
//! One solution to this problem is to use `futures::future::Either`
//! to combine together all your different possible futures together
//! into one type to return. Doing this by hand is really annoying
//! and requires sweeping changes when you change the number of
//! possible branches.
//!
//! This macro `future_union` does this automatically.
//! Currently you still have to keep the total count per function, and then
//! also the index (starting from 0). If those values are inaccurate then
//! you'll get horrible type errors.
//!
//! ### Example
//! ```no_run
//! use futures::future::{self, Future};
//! use future_union::future_union;
//!
//! fn impl_demo(n: usize) -> impl Future<Item=(), Error=()> {
//!    match n {
//!        0 => future_union!(3, 0, future::ok(())),
//!        1 => future_union!(3, 1, future::ok(()).map(|_| ())),
//!        _ => future_union!(3, 2, future::ok(()).map(|_| ()).map(|_| ())),
//!    }
//!}
//! ```
//!
//! ### Future (heh) plans:
//! - support for futures-0.3
//! - implement a function attribute macro that detects `future_union` calls
//!   in a given function and automatically adds the correct count and index e.g.:
//!     ```ignore
//!     use futures::future::Future;
//!     #[future_union_fn]
//!     fn impl_demo(n: usize) -> impl Future<Item=(), Error=()> {
//!         match n {
//!             0 => future_union_auto!(future::ok(())),
//!             1 => future_union_auto!(future::ok(()).map(|_| ())),
//!             _ => future_union_auto!(future::ok(()).map(|_| ()).map(|_| ())),
//!         }
//!     }
//!     ```
//! Contributions welcome!

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use future_union_impl::future_union_impl as future_union;

#[cfg(test)]
mod tests {
    use futures::future;
    use futures::future::Future;
    use future_union;

    #[test]
    fn demo_compiles() {

        fn _impl_demo(n: usize) -> impl Future<Item=(), Error=()> {
            match n {
                0 => future_union!(3, 0, future::ok(())),
                1 => future_union!(3, 1, future::ok(()).map(|_| ())),
                _ => future_union!(3, 2, future::ok(()).map(|_| ()).map(|_| ())),
            }
        }
    }

    #[test]
    fn trees_as_expected() {
        fn _tree_2() ->
                     future::Either<
                         [(); 0],
                         [(); 1],
                     > {
            match 0 {
                0 => future_union!(2, 0, [(); 0]),
                _ => future_union!(2, 1, [(); 1]),
            }
        }

        fn _tree_3() ->
                     future::Either<
                         future::Either<
                             [(); 0],
                             [(); 1],
                         >,
                         [(); 2],
                     > {
            match 0 {
                0 => future_union!(3, 0, [(); 0]),
                1 => future_union!(3, 1, [(); 1]),
                _ => future_union!(3, 2, [(); 2]),
            }
        }

        fn _tree_4() ->
                     future::Either<
                         future::Either<
                             [(); 0],
                             [(); 1],
                         >,
                         future::Either<
                             [(); 2],
                             [(); 3],
                         >,
                     > {
            match 0 {
                0 => future_union!(4, 0, [(); 0]),
                1 => future_union!(4, 1, [(); 1]),
                2 => future_union!(4, 2, [(); 2]),
                _ => future_union!(4, 3, [(); 3]),
            }
        }

        fn _tree_5() ->
                     future::Either<
                         future::Either<
                             future::Either<
                                 [(); 0],
                                 [(); 1],
                             >,
                             future::Either<
                                 [(); 2],
                                 [(); 3],
                             >,
                         >,
                         [(); 4]
                     > {
            match 0 {
                0 => future_union!(5, 0, [(); 0]),
                1 => future_union!(5, 1, [(); 1]),
                2 => future_union!(5, 2, [(); 2]),
                3 => future_union!(5, 3, [(); 3]),
                _ => future_union!(5, 4, [(); 4]),
            }
        }

        fn _tree_6() ->
                     future::Either<
                         future::Either<
                             future::Either<
                                 [(); 0],
                                 [(); 1],
                             >,
                             future::Either<
                                 [(); 2],
                                 [(); 3],
                             >,
                         >,
                         future::Either<
                             [(); 4],
                             [(); 5],
                         >,
                     > {
            match 0 {
                0 => future_union!(6, 0, [(); 0]),
                1 => future_union!(6, 1, [(); 1]),
                2 => future_union!(6, 2, [(); 2]),
                3 => future_union!(6, 3, [(); 3]),
                4 => future_union!(6, 4, [(); 4]),
                _ => future_union!(6, 5, [(); 5]),
            }
        }

        fn _tree_7() ->
                     future::Either<
                         future::Either<
                             future::Either<
                                 [(); 0],
                                 [(); 1],
                             >,
                             future::Either<
                                 [(); 2],
                                 [(); 3],
                             >,
                         >,
                         future::Either<
                             future::Either<
                                 [(); 4],
                                 [(); 5],
                             >,
                             [(); 6],
                         >,
                     > {
            match 0 {
                0 => future_union!(7, 0, [(); 0]),
                1 => future_union!(7, 1, [(); 1]),
                2 => future_union!(7, 2, [(); 2]),
                3 => future_union!(7, 3, [(); 3]),
                4 => future_union!(7, 4, [(); 4]),
                5 => future_union!(7, 5, [(); 5]),
                _ => future_union!(7, 6, [(); 6]),
            }
        }

        fn _tree_8() ->
                     future::Either<
                         future::Either<
                             future::Either<
                                 [(); 0],
                                 [(); 1],
                             >,
                             future::Either<
                                 [(); 2],
                                 [(); 3],
                             >,
                         >, future::Either<
                             future::Either<
                                 [(); 4],
                                 [(); 5],
                             >,
                             future::Either<
                                 [(); 6],
                                 [(); 7],
                             >,
                         >,
                     > {
            match 0 {
                0 => future_union!(8, 0, [(); 0]),
                1 => future_union!(8, 1, [(); 1]),
                2 => future_union!(8, 2, [(); 2]),
                3 => future_union!(8, 3, [(); 3]),
                4 => future_union!(8, 4, [(); 4]),
                5 => future_union!(8, 5, [(); 5]),
                6 => future_union!(8, 6, [(); 6]),
                _ => future_union!(8, 7, [(); 7]),
            }
        }

        fn _tree_9() ->
                     future::Either<
                         future::Either<
                             future::Either<
                                 future::Either<
                                     [(); 0],
                                     [(); 1],
                                 >,
                                 future::Either<
                                     [(); 2],
                                     [(); 3],
                                 >,
                             >,
                             future::Either<
                                 future::Either<
                                     [(); 4],
                                     [(); 5],
                                 >,
                                 future::Either<
                                     [(); 6],
                                     [(); 7],
                                 >,
                             >,
                         >,
                         [(); 8],
                     > {
            match 0 {
                0 => future_union!(9, 0, [(); 0]),
                1 => future_union!(9, 1, [(); 1]),
                2 => future_union!(9, 2, [(); 2]),
                3 => future_union!(9, 3, [(); 3]),
                4 => future_union!(9, 4, [(); 4]),
                5 => future_union!(9, 5, [(); 5]),
                6 => future_union!(9, 6, [(); 6]),
                7 => future_union!(9, 7, [(); 7]),
                _ => future_union!(9, 8, [(); 8]),
            }
        }
    }
}
