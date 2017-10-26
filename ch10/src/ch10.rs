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
// ch10.rs -- Functions from Chapter 10 of fM2GP.
// -------------------------------------------------------------------

pub mod fmgp {

    // Section 10.5

    type DifferenceType = usize;

    pub fn distance_input<I>(f: I) -> DifferenceType
    where
        I: IntoIterator,
    {
        let mut n = 0;
        for _ in f {
            n += 1;
        }
        n
    }

    pub fn distance_random_access<T>(f: &[T]) -> DifferenceType {
        f.len()
    }

    // Section 10.6

    pub fn advance_input<I>(x: &mut I, mut n: DifferenceType)
    where
        I: Iterator,
    {
        while n != 0 {
            n -= 1;
            x.next();
        }
    }

    pub fn advance_random_access<T>(x: &[T], n: DifferenceType) -> &[T] {
        &x[n..]
    }

    // Section 10.7

    pub fn find_if<I, P>(f: &mut I, p: P) -> Option<I::Item>
    where
        I: Iterator,
        P: Fn(&I::Item) -> bool,
    {
        loop {
            match f.next() {
                Some(x) => if p(&x) {
                    return Some(x);
                },
                None => return None,
            }
        }
    }

    pub fn find_if_n<I, P>(
        f: &mut I,
        mut n: DifferenceType,
        p: P,
    ) -> (Option<I::Item>, DifferenceType)
    where
        I: Iterator,
        P: Fn(&I::Item) -> bool,
    {
        while n != 0 {
            match f.next() {
                Some(x) => if p(&x) {
                    return (Some(x), n);
                },
                None => break,
            }
            n -= 1;
        }
        (None, n)
    }

    // Section 10.8

    pub fn partition_point_n<I, P>(mut f: I, mut n: DifferenceType, p: P) -> I
    where
        I: Iterator,
        I: Clone,
        P: Fn(&I::Item) -> bool,
    {
        while n != 0 {
            let mut middle = f.clone();
            let half = n >> 1;
            advance_input(&mut middle, half);
            match middle.next() {
                Some(x) => if !p(&x) {
                    n = half;
                } else {
                    f = middle;
                    n -= half + 1;
                },
                None => break,
            }
        }
        f
    }

    pub fn partition_point<I, P>(f: I, p: P) -> I
    where
        I: Iterator,
        I: Clone,
        P: Fn(&I::Item) -> bool,
    {
        let n = distance_input(f.clone());
        partition_point_n(f, n, p)
    }

    pub fn lower_bound<I>(f: I, a: &I::Item) -> I
    where
        I: Iterator,
        I: Clone,
        I::Item: ::std::cmp::PartialOrd,
    {
        partition_point(f, |x| x < a)
    }

    pub fn upper_bound<I>(f: I, a: &I::Item) -> I
    where
        I: Iterator,
        I: Clone,
        I::Item: ::std::cmp::PartialOrd,
    {
        partition_point(f, |x| x <= a)
    }
} // namespace fmgp
