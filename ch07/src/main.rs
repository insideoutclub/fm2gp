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
// main.rs -- For testing functions from Chapter 7 of fM2GP.
// -------------------------------------------------------------------

mod ch07;
use ch07::*;
extern crate num_traits;
use ch07::MonoidOperation;

struct Plus();

impl<A> SemigroupOperation<A> for Plus
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    fn apply(&self, x: &A, y: &A) -> A {
        x + y
    }
}

impl<A> MonoidOperation<A> for Plus
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    A: num_traits::Zero,
{
    fn identity_element(&self) -> A {
        num_traits::zero()
    }
}

impl<A> GroupOperation<A> for Plus
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    A: num_traits::Zero,
    for<'a> &'a A: std::ops::Neg<Output = A>,
{
    fn inverse_operation(&self, x: &A) -> A {
        -x
    }
}

fn main() {
    println!("mult_acc4(0, 7, 8) = {}", mult_acc4(0, 7, 8));
    println!(
        "multiply_accumulate0(0, 7, 8) = {}",
        multiply_accumulate0(0, 7, 8)
    );
    println!(
        "multiply_accumulate(0, 7, 8) = {}",
        multiply_accumulate(0, 7, 8)
    );
    println!(
        "multiply_accumulate_semigroup(0, 7, 8) = {}",
        multiply_accumulate_semigroup(0, 7, 8)
    );
    println!("multiply_semigroup(7, 8) = {}", multiply_semigroup(7, 8));
    println!("multiply_monoid(7, 8) = {}", multiply_monoid(7, 8));
    println!("multiply_group(7, 8) = {}", multiply_group(7, 8));
    println!(
        "power_accumulate_semigroup(1, 2, 10) = {}",
        power_accumulate_semigroup(1, 2, 10)
    );
    println!("power_semigroup(2, 10) = {}", power_semigroup(2, 10));
    println!("power_monoid(2, 10) = {}", power_monoid(2, 10));
    println!("power_monoid(2, 0) = {}", power_monoid(2, 0));
    println!("power_group(2., -10) = {}", power_group(2., -10));
    let plus_int = Plus();
    println!(
        "power_accumulate_semigroup(0, 7, 8, plus_int) = {}",
        power_accumulate_semigroup_with_op(0, 7, 8, &plus_int)
    );
    println!(
        "power_semigroup(7, 8, plus_int) = {}",
        power_semigroup_with_op(7, 8, &plus_int)
    );
    println!(
        "power_monoid(0, 8, plus_int) = {}",
        power_monoid_with_op(0, 8, &plus_int)
    );
    println!(
        "power_group(7, -8, plus_int) = {}",
        power_group_with_op(7, -8, &plus_int)
    );
    println!("fib0(5) = {}", fib0(5));
    println!("fibonacci_iterative(5) = {}", fibonacci_iterative(5));
}
