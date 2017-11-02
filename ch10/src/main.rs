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
// main.rs -- For testing functions from Chapter 10 of fM2GP.
// -------------------------------------------------------------------

mod ch10;
use ch10::*;

fn equal_3(x: &&i32) -> bool {
    **x == 3
}

fn foo(x: &i32) -> bool {
    *x == 3
}

fn less_3(x: &&i32) -> bool {
    **x < 3
}

fn bar(x: &i32) -> bool {
    *x < 3
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut l = std::collections::LinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    l.push_back(5);
    println!("Initial sequence:  1 2 3 4 5");
    println!("vector v and list l initialized to this sequence.");

    let mut x1 = fmgp::begin_random_access(v.as_slice());
    let mut y1 = fmgp::end_random_access(v.as_slice());
    let mut x2 = fmgp::begin(l.iter());
    let mut y2 = fmgp::end(l.iter());
    println!("Setting x1 = begin(v); y1 = end(v)");
    println!("Setting x2 = begin(l); y2 = end(l)");

    println!(
        "fmgp::distance(x1, y1) is {}",
        fmgp::distance_random_access(x1.clone(), y1.clone())
    );
    println!(
        "fmgp::distance(x2, y2) is {}",
        fmgp::distance_input(x2, &y2)
    );
    fmgp::advance_random_access(&mut x1, 3);
    print!("After advance(x1, 3): ");
    println!("(x1 == y1) is {}", if x1 == y1 { 1 } else { 0 });
    x2 = fmgp::begin(l.iter());
    fmgp::advance_input(&mut x2, 3);
    print!("After advance(x2, 3): ");
    println!("(x2 == y2) is {}\n", if x2 == y2 { 1 } else { 0 });

    x1 = fmgp::begin_random_access(v.as_slice());
    y1 = fmgp::end_random_access(v.as_slice());
    x2 = fmgp::begin(l.iter());
    y2 = fmgp::end(l.iter());
    println!("Setting x1 = begin(v); y1 = end(v)");
    println!("Setting x2 = begin(l); y2 = end(l)");

    println!(
        "*fmgp::find_if(x1, y1, equal_3) is {}",
        *fmgp::find_if(x1.clone(), &y1, foo)
    );
    println!(
        "*fmgp::find_if(x2, y2, equal_3) is {}",
        *fmgp::find_if(x2, &y2, equal_3)
    );
    println!(
        "*fmgp::find_if_n(x1, 5, equal_3).first is {}",
        *fmgp::find_if_n(x1.clone(), 3, foo).0
    );
    println!(
        "*fmgp::find_if_n(x2, 5, equal_3).first is {}",
        *fmgp::find_if_n(fmgp::begin(l.iter()), 3, equal_3).0
    );

    println!(
        "*fmgp::partition_point(x1, y1, less_3) is {}",
        *fmgp::partition_point(x1.clone(), &y1, bar)
    );
    println!(
        "*fmgp::partition_point(x2, y2, less_3) is {}",
        *fmgp::partition_point(fmgp::begin(l.iter()), &y2, less_3)
    );
    println!(
        "*fmgp::partition_point_n(x1, 5, less_3) is {}",
        *fmgp::partition_point_n(x1.clone(), 3, bar)
    );
    println!(
        "*fmgp::partition_point_n(x2, 5, less_3) is {}",
        *fmgp::partition_point_n(fmgp::begin(l.iter()), 3, less_3)
    );
    println!(
        "*fmgp::upper_bound(x1, y1, 2) is {}",
        *fmgp::upper_bound(x1.clone(), &y1, &2)
    );
    println!(
        "*fmgp::upper_bound(x2, y2, 2) is {}",
        *fmgp::upper_bound(fmgp::begin(l.iter()), &y2, &&2)
    );

    println!(
        "*fmgp::lower_bound(x1, y1, 2) is {}",
        *fmgp::lower_bound(x1, &y1, &2)
    );
    println!(
        "*fmgp::lower_bound(x2, y2, 2) is {}",
        *fmgp::lower_bound(fmgp::begin(l.iter()), &y2, &&2)
    );
}
