// Copyright (c) 2014 Alexander A. Stepanov and Daniel E. Rose
//
// Permission to use, copy, modify, distribute and sell this software
// and its documentation for any purpose is hereby granted without
// fee, provided that the above copyright notice appear in all copies
// and that both that copyright notice and this permission notice
// appear in supporting documentation. The authors make no
// representations about the suitability of this software for any
// purpose. It is provided "as is" without express or implied
// warranty.
//
// This code accompanies the "fM2GP" book:
//
//	From Mathematics to Generic Programming
//	by Alexander Stepanov and Daniel E. Rose
//	Addison-Wesley Professional, 2015
//
// -------------------------------------------------------------------
// ch08.rs -- Functions from Chapter 8 of fM2GP.
// -------------------------------------------------------------------

extern crate num_traits;
extern crate std;

// Section 8.1

pub fn polynomial_value<I, R>(first: I, x: &R) -> R
where
    I: IntoIterator<Item = R>,
    R: num_traits::Zero,
    R: for<'a> std::ops::Mul<&'a R, Output = R>,
    R: for<'a> std::ops::Add<&'a R, Output = R>,
{
    let mut iter = first.into_iter();
    match iter.next() {
        None => num_traits::zero(),
        Some(mut sum) => {
            for first in iter {
                sum = sum * x;
                sum = sum + &first;
            }
            sum
        }
    }
}
