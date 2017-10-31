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

fn less_3(x: &&i32) -> bool {
    **x < 3
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

    let x1 = begin(v.iter());
    let y1 = end(v.iter());
    let mut x2 = begin(l.iter());
    let y2 = end(l.iter());
    println!("Setting x1 = begin(v); y1 = end(v)");
    println!("Setting x2 = begin(l); y2 = end(l)");

    /*
    println!(
        "fmgp::distance(x1, y1) is {}",
        fmgp::distance_random_access(Wrapper {x: v.iter()})
    );
    */
    println!(
        "fmgp::distance(x2, y2) is {}",
        fmgp::distance_input(x2, &y2)
    );
    /*
    x1 = fmgp::advance_random_access(x1, 3);
    print!("After advance(x1, 3): ");
    println!("(x1 == y1) is {}", if x1 == y1 { 1 } else { 0 });
    x2 = l.iter();
    */
    x2 = begin(l.iter());
    fmgp::advance_input(&mut x2, 3);
    print!("After advance(x2, 3): ");
    println!("(x2 == y2) is {}\n", if x2 == y2 { 1 } else { 0 });

    /*
    x1 = &v[..];
    */
    x2 = begin(l.iter());
    println!("Setting x1 = begin(v); y1 = end(v)");
    println!("Setting x2 = begin(l); y2 = end(l)");

    /*
    println!(
        "*fmgp::find_if(x1, y1, equal_3) is {}",
        fmgp::find_if(&mut x1.iter(), equal_3).unwrap()
    );
    */
    println!(
        "*fmgp::find_if(x2, y2, equal_3) is {}",
        fmgp::find_if(x2, &y2, equal_3).source()
    );
    /*
    println!(
        "*fmgp::find_if_n(x1, 5, equal_3).first is {}",
        fmgp::find_if_n(&mut x1.iter(), 3, equal_3).0.unwrap()
    );
    */
    println!(
        "*fmgp::find_if_n(x2, 5, equal_3).first is {}",
        fmgp::find_if_n(begin(l.iter()), 3, equal_3).0.source()
    );

    /*
    println!(
        "*fmgp::partition_point(x1, y1, less_3) is {}",
        fmgp::partition_point(x1.iter(), less_3).next().unwrap()
    );
    */
    println!(
        "*fmgp::partition_point(x2, y2, less_3) is {}",
        fmgp::partition_point(begin(l.iter()), &y2, less_3).source()
    );
    /*
    println!(
        "*fmgp::partition_point_n(x1, 5, less_3) is {}",
        fmgp::partition_point_n(x1.iter(), 3, less_3)
            .next()
            .unwrap()
    );
    */
    println!(
        "*fmgp::partition_point_n(x2, 5, less_3) is {}",
        fmgp::partition_point_n(begin(l.iter()), 3, less_3).source()
    );
    /*
    println!(
        "*fmgp::upper_bound(x1, y1, 2) is {}",
        fmgp::upper_bound(x1.iter(), &&2).next().unwrap()
    );
    */
    println!(
        "*fmgp::upper_bound(x2, y2, 2) is {}",
        fmgp::upper_bound(begin(l.iter()), &y2, &&2).source()
    );

    /*
    println!(
        "*fmgp::lower_bound(x1, y1, 2) is {}",
        fmgp::lower_bound(x1.iter(), &&2).next().unwrap()
    );
    */
    println!(
        "*fmgp::lower_bound(x2, y2, 2) is {}",
        fmgp::lower_bound(begin(l.iter()), &y2, &&2).source()
    );
}
