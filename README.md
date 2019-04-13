# Future Union
When you use impl traits, specifically with futures,
sometimes you will want to have a branching expression
(e.g. an `if` or `match`) in which the different branches
return different types that both impl Future. This does
not work since in current stable rust impl trait can only
refer to a single type.

One solution to this problem is to use `futures::future::Either`
to combine together all your different possible futures together
into one type to return. Doing this by hand is really annoying
and requires sweeping changes when you change the number of
possible branches.

This macro `future_union` does this automatically.
Currently you still have to keep the total count per function, and then
also the index (starting from 0). If those values are inaccurate then
you'll get horrible type errors.

## Example
```no_run
use futures::future::{self, Future};
use future_union::future_union;

fn impl_demo(n: usize) -> impl Future<Item=(), Error=()> {
    match n {
        0 => future_union!(3, 0, future::ok(())),
        1 => future_union!(3, 1, future::ok(()).map(|_| ())),
        _ => future_union!(3, 2, future::ok(()).map(|_| ()).map(|_| ())),
    }
}
```

## Future (heh) plans:
- support for futures-0.3
- implement a function attribute macro that detects `future_union` calls
  in a given function and automatically adds the correct count and index e.g.:
    ```ignore
    use futures::future::{self, Future};
    use future_union::*;
    
    #[future_union_fn]
    fn impl_demo(n: usize) -> impl Future<Item=(), Error=()> {
        match n {
            0 => future_union_auto!(future::ok(())),
            1 => future_union_auto!(future::ok(()).map(|_| ())),
            _ => future_union_auto!(future::ok(()).map(|_| ()).map(|_| ())),
        }
    }
    ```
Contributions welcome!

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
