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
// ch03.cpp -- For testing functions from Chapter 3 of fM2GP.
// -------------------------------------------------------------------

mod ch03;
use ch03::*;

fn print_sieve<'a, I>(first: I)
where
    I: IntoIterator<Item = &'a bool>,
{
    print!("2");
    for (i, x) in first.into_iter().enumerate() {
        if *x {
            print!(" {}", 2 * i + 3);
        }
    }
    println!();
}

fn main() {
    let mut v = vec![false; 500];
    sift0(&mut v[..50]);
    println!("sift0(begin(v), 50):");
    print_sieve(&v[..50]);
    sift1(&mut v[..50]);
    println!("sift1(begin(v), 50):");
    print_sieve(&v[..50]);
    sift(&mut v);
    println!("sift(begin(v), 500):");
    print_sieve(&v);
    println!("gcm(15, 9) = {}", gcm(15, 9));
}
