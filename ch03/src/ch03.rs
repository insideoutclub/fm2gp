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
// ch03.rs -- Functions from Chapter 3 of fM2GP.
// -------------------------------------------------------------------

type Integer = usize;

// Section 3.3

pub fn mark_sieve(slice: &mut [bool], factor: Integer) {
    let (mut first, last) = (0, slice.len());
    // assert(first != last)
    slice[first] = false;
    while last - first > factor {
        first += factor;
        slice[first] = false;
    }
}

fn fill<'a, I, T>(xs: I, value: T)
where
    I: IntoIterator<Item = &'a mut T>,
    T: Copy,
    T: 'a,
{
    for x in xs {
        *x = value;
    }
}

pub fn sift0(first: &mut [bool]) {
    let n = first.len();
    fill(&mut first[..], true);
    let mut i = 0;
    let mut index_square = 3;
    while index_square < n {
        // invariant: index_square = 2i^2 + 6i + 3
        if first[i] {
            // if current candidate is prime
            mark_sieve(&mut first[index_square..], i + i + 3); // factor
        }
        i += 1;
        index_square = 2 * i * (i + 3) + 3;
    }
}

pub fn sift1(first: &mut [bool]) {
    let n = first.len();
    fill(&mut first[..], true);
    let mut i = 0;
    let mut index_square = 3;
    let mut factor = 3;
    while index_square < n {
        // invariant: index_square = 2i^2 + 6i + 3, factor = 2i + 3
        if first[i] {
            mark_sieve(&mut first[index_square..], factor);
        }
        i += 1;
        factor = i + i + 3;
        index_square = 2 * i * (i + 3) + 3;
    }
}

pub fn sift(first: &mut [bool]) {
    let n = first.len();
    fill(&mut first[..], true);
    let mut i = 0;
    let mut index_square = 3;
    let mut factor = 3;
    while index_square < n {
        // invariant: index_square = 2i^2 + 6i + 3, factor = 2i + 3
        if first[i] {
            mark_sieve(&mut first[index_square..], factor);
        }
        i += 1;
        index_square += factor;
        factor += 2;
        index_square += factor;
    }
}

// Section 3.5

type LineSegment = u32;

pub fn gcm(a: LineSegment, b: LineSegment) -> LineSegment {
    if a == b {
        return a;
    }
    if b < a {
        return gcm(a - b, b);
    }
    /* if (a < b) */
    gcm(a, b - a)
}
