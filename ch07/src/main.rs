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
// ch07.cpp -- For testing functions from Chapter 7 of fM2GP.
// -------------------------------------------------------------------

mod ch07;
use ch07::*;

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
    let plus_int = |x: &i32, y: &i32| x + y;
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
        power_monoid_with_op(0, 8, &plus_int, 0)
    );
    let negate = |x: &i32| -x;
    println!(
        "power_group(7, -8, plus_int) = {}",
        power_group_with_op(7, -8, &plus_int, &negate, 0)
    );
    println!("fib0(5) = {}", fib0(5));
    println!("fibonacci_iterative(5) = {}", fibonacci_iterative(5));
}
