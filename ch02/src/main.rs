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
// main.rs -- For testing functions from Chapter 2 of fM2GP.
// -------------------------------------------------------------------

mod ch02;
use ch02::*;

fn main() {
    println!("multiply0(7, 8) = {}", multiply0(7, 8));
    println!("multiply1(7, 8) = {}", multiply1(7, 8));
    println!("multiply_by_15(15) = {}", multiply_by_15(15));
    println!("mult_acc0(0, 7, 8) = {}", mult_acc0(0, 7, 8));
    println!("mult_acc1(0, 7, 8) = {}", mult_acc1(0, 7, 8));
    println!("mult_acc2(0, 7, 8) = {}", mult_acc2(0, 7, 8));
    println!("mult_acc3(0, 7, 8) = {}", mult_acc3(0, 7, 8));
    println!("mult_acc4(0, 7, 8) = {}", mult_acc4(0, 7, 8));
    println!("multiply2(7, 8) = {}", multiply2(7, 8));
    println!("multiply3(7, 8) = {}", multiply3(7, 8));
    println!("multiply4(7, 8) = {}", multiply4(7, 8));
}
