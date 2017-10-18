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
// main.rs -- For testing functions from Chapter 4 of fM2GP.
// -------------------------------------------------------------------

mod ch04;
use ch04::*;

fn main() {
    println!("gcm0(121, 66) = {}", gcm0(121, 66));
    println!("gcm1(121, 66) = {}", gcm1(121, 66));
    println!("gcm(121, 66) = {}", gcm(121, 66));
    println!("fast_segment_gcm(121, 66) = {}", fast_segment_gcm(121, 66));
    println!("remainder(100, 7) = {}", remainder(100, 7));
    println!("quotient(100, 7) = {}", quotient(100, 7));
    let p = quotient_remainder(100, 7);
    println!("quotient_remainder(100, 7) = pair<{}, {}>", p.0, p.1);
    println!(
        "remainder_fibonacci(100, 7) = {}",
        remainder_fibonacci(100, 7)
    );
    println!("gcm_remainder(121, 66) = {}", gcm_remainder(121, 66));
    println!("gcd(121, 66) = {}", gcd(121, 66));
}
