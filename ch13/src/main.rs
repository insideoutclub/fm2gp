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
// main.rs -- For testing functions from Chapter 13 of fM2GP.
// -------------------------------------------------------------------

mod ch07;
mod ch12;
mod ch13;
use ch13::*;

fn main() {
    println!("is_prime(101) = {}", is_prime(&101));
    println!("is_prime(105) = {}", is_prime(&105));
    println!("is_prime(10007) = {}", is_prime(&10_007));
    println!(
        "(24 * multiplicative_inverse_fermat(24, 101)) % 101 = {}",
        (24 * multiplicative_inverse_fermat(24, 101)) % 101
    );
    println!(
        "(24 * multiplicative_inverse_fermat(24, 10007)) % 10007 = {}",
        (24 * multiplicative_inverse_fermat(24, 10_007)) % 10_007
    );
    println!(
        "(24 * multiplicative_inverse(24, 10007)) % 10007 = {}",
        (24 * multiplicative_inverse(24, 10_007)) % 10_007
    );
    println!(
        "fermat_test(10001, 7) = {}",
        if fermat_test(10_001, 7) { 1 } else { 0 }
    );
    println!(
        "fermat_test(10007, 7) = {}",
        if fermat_test(10_007, 7) { 1 } else { 0 }
    );
    println!(
        "miller_rabin_test(10001, 625, 4, 7) = {}",
        if miller_rabin_test(&10_001, 625, &4, 7) {
            1
        } else {
            0
        }
    );
    println!(
        "miller_rabin_test(10007, 5003, 1, 7) = {}",
        if miller_rabin_test(&10_007, 5003, &1, 7) {
            1
        } else {
            0
        }
    );
    println!(
        "fermat_test(1729, 2) = {}",
        if fermat_test(1729, 2) { 1 } else { 0 }
    );
    println!(
        "miller_rabin_test(1729, 27, 6, 2) = {}",
        if miller_rabin_test(&1729, 27, &6, 2) {
            1
        } else {
            0
        }
    );
}
