extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use std::iter::FromIterator;

use quote::quote;

#[proc_macro_hack]
pub fn future_union_impl(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();

    let count_arg_token = iter.next().unwrap_or_else(|| panic!("Too few arguments"));
    let count_arg = syn::parse::<syn::LitInt>(
        TokenStream::from(count_arg_token)
    ).unwrap_or_else(|_| panic!("Expecting integer literal")).value();

    let comma_1 = iter.next().unwrap_or_else(|| panic!("Too few arguments"));
    match comma_1 {
        TokenTree::Punct(ref p) if p.as_char() == ',' => (),
        _ => panic!("Invalid syntax, expected a comma"),
    }

    let n_arg_token = iter.next().unwrap_or_else(|| panic!("Too few arguments"));
    let n_arg = syn::parse::<syn::LitInt>(
        TokenStream::from(n_arg_token)
    ).unwrap_or_else(|_| panic!("Expecting integer literal")).value();

    let comma_2 = iter.next().unwrap_or_else(|| panic!("Too few arguments"));
    match comma_2 {
        TokenTree::Punct(ref p) if p.as_char() == ',' => (),
        _ => panic!("Invalid syntax, expected a comma"),
    }

    let remaining_tokens = proc_macro2::TokenStream::from(TokenStream::from_iter(iter));

    TokenStream::from(
        future_union_make_tree(count_arg, n_arg, remaining_tokens)
    )
}

fn future_union_make_tree(count: u64, n: u64, expr: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    assert!(n < count);

    if count <= 0 {
        panic!()
    } else if count == 1 {
        expr
    } else if count == 2 {
        if n & 1 == 0 {
            quote!( futures::future::Either::A(#expr) )
        } else {
            quote!( futures::future::Either::B(#expr) )
        }
    } else {
        let max_cap = round_up_to_power_of_2(count);
        if first_half(max_cap, n) {
            let sub_tree = future_union_make_tree(max_cap/2, n, expr);
            quote!( futures::future::Either::A(#sub_tree) )
        } else {
            let sub_tree = future_union_make_tree(count-max_cap/2, n-max_cap/2, expr);
            quote!( futures::future::Either::B(#sub_tree) )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_up_to_power_of_2_test() {
        assert_eq!(round_up_to_power_of_2(2), 2);
        assert_eq!(round_up_to_power_of_2(3), 4);
        assert_eq!(round_up_to_power_of_2(4), 4);
        assert_eq!(round_up_to_power_of_2(5), 8);
        assert_eq!(round_up_to_power_of_2(6), 8);
        assert_eq!(round_up_to_power_of_2(7), 8);
        assert_eq!(round_up_to_power_of_2(8), 8);
        assert_eq!(round_up_to_power_of_2(9), 16);
    }

    #[test]
    fn first_half_test() {
        assert!(first_half(2, 0));
        assert!(!first_half(2, 1));

        assert!(first_half(4, 0));
        assert!(first_half(4, 1));
        assert!(!first_half(4, 2));
        assert!(!first_half(4, 3));

        assert!(first_half(8, 0));
        assert!(first_half(8, 1));
        assert!(first_half(8, 2));
        assert!(first_half(8, 3));
        assert!(!first_half(8, 4));
        assert!(!first_half(8, 5));
        assert!(!first_half(8, 6));
        assert!(!first_half(8, 7));
    }
}

fn first_half(cap: u64, n: u64) -> bool {
    assert!(n < cap);
    n < cap/2
}

// Rounds up to the nearest power of 2. Probably can just check the most significant bit?
fn round_up_to_power_of_2(n: u64) -> u64 {
    (n as f64).log2().ceil().exp2() as u64
}

