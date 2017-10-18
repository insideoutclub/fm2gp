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
// main.rs -- For testing functions from Chapter 11 of fM2GP.
// -------------------------------------------------------------------

mod ch11;
use ch11::*;

fn print_range<I>(f: I)
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    for x in f {
        print!("{} ", x);
    }
    println!();
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    print!("Initial sequence:\t\t\t\t\t");
    print_range(&v);
    println!("Setting x = begin(v); y = end(v)");
    {
        let (left, right) = v.split_at_mut(3);
        fmgp::swap_ranges(left.iter_mut(), right[1..].iter_mut());
    }
    print!("After swap_ranges(x, x+3, x+4):\t\t\t\t");
    print_range(&v);
    {
        let (left, right) = v.split_at_mut(3);
        fmgp::swap_ranges(left.iter_mut(), right[1..].iter_mut());
    }
    print!("After swap_ranges(x, x+3, x+4):\t\t\t\t");
    print_range(&v);
    {
        let (left, right) = v.split_at_mut(4);
        fmgp::swap_ranges_bounded(left.iter_mut(), right.iter_mut());
    }
    print!("After swap_ranges(x, x+4, x+4, y):\t\t\t");
    print_range(&v);
    {
        let (left, right) = v.split_at_mut(4);
        fmgp::swap_ranges_bounded(left.iter_mut(), right.iter_mut());
    }
    print!("After swap_ranges(x, x+4, x+4, y):\t\t\t");
    print_range(&v);
    {
        let (left, right) = v.split_at_mut(4);
        fmgp::swap_ranges_n(left.iter_mut(), right.iter_mut(), 3);
    }
    print!("After swap_ranges(x, x+4, 3):\t\t\t\t");
    print_range(&v);
    {
        let (left, right) = v.split_at_mut(4);
        fmgp::swap_ranges_n(left.iter_mut(), right.iter_mut(), 3);
    }
    print!("After swap_ranges(x, x+4, 3):\t\t\t\t");
    print_range(&v);
    let m = 4;
    fmgp::gries_mills_rotate(&mut v, m);
    println!("Setting m = x + 4");
    print!("After gries_mills_rotate(x, m, y):\t\t\t");
    print_range(&v);
    fmgp::gries_mills_rotate(&mut v, 3);
    print!("After gries_mills_rotate(x, x+3, y):\t\t\t");
    print_range(&v);
    let mut m1 = fmgp::rotate_forward(&mut v, m);
    println!("Setting m1 = rotate(x, m, y, forward_iterator_tag())");
    print!("After rotate(x, m, y, forward_iterator_tag()):\t\t");
    print_range(&v);
    fmgp::rotate_forward(&mut v, m1);
    print!("After rotate(x, m1, y, forward_iterator_tag()):\t\t");
    print_range(&v);
    m1 = fmgp::rotate_random_access(&mut v, m);
    print!("After rotate(x, m, y, random_access_iterator_tag()):\t");
    print_range(&v);
    fmgp::rotate_random_access(&mut v, m1);
    print!("After rotate(x, m1, y, random_access_iterator_tag()):\t");
    print_range(&v);
    fmgp::three_reverse_rotate(&mut v, m);
    print!("After three_reverse_rotate(x, m, y)\t\t\t");
    print_range(&v);
    fmgp::three_reverse_rotate(&mut v, 3);
    print!("After three_reverse_rotate(x, x+3, y)\t\t\t");
    print_range(&v);
    m1 = fmgp::rotate_bidirectional(&mut v, m);
    print!("After rotate(x, m, y, bidirectional_iterator_tag()):\t");
    print_range(&v);
    fmgp::rotate_bidirectional(&mut v, m1);
    print!("After rotate(x, m1, y, bidirectional_iterator_tag()):\t");
    print_range(&v);
    fmgp::reverse_n(v.iter_mut(), 7);
    print!("After reverse_n(x, y, 7):\t\t\t\t");
    print_range(&v);
    fmgp::reverse_n(v.iter_mut(), 7);
    print!("After reverse_n(x, y, 7):\t\t\t\t");
    print_range(&v);
    fmgp::reverse_recursive(&mut v, 0, 7);
    print!("After reverse_recursive(x, 7):\t\t\t\t");
    print_range(&v);
    fmgp::reverse_recursive(&mut v, 0, 7);
    print!("After reverse_recursive(x, 7):\t\t\t\t");
    print_range(&v);
    let mut buffer = vec![0; 3];
    fmgp::reverse_n_adaptive(&mut v, 0, 7, &mut buffer);
    print!("After reverse_n_adaptive(x, 7, begin(buffer), 3):\t");
    print_range(&v);
    fmgp::reverse_n_adaptive(&mut v, 0, 7, &mut buffer);
    print!("After reverse_n_adaptive(x, 7, begin(buffer), 3):\t");
    print_range(&v);
}
