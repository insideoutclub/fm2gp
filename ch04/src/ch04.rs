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
// ch04.rs -- Functions from Chapter 4 of fM2GP.
// -------------------------------------------------------------------

extern crate std;

type LineSegment = u32;
type Integer = u32;

// Section 4.2

fn _odd(n: u32) -> bool {
    n & 0x1 == 1
}

fn half(n: u32) -> u32 {
    n >> 1
}

pub fn gcm0(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    while a != b {
        if b < a {
            a -= b
        } else {
            b -= a
        }
    }
    a
}

pub fn gcm1(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    while a != b {
        while b < a {
            a -= b
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn segment_remainder(mut a: LineSegment, b: LineSegment) -> LineSegment {
    while b < a {
        a -= b
    }
    a
}

pub fn gcm(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    while a != b {
        a = segment_remainder(a, b);
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn fast_segment_remainder(mut a: LineSegment, b: LineSegment) -> LineSegment {
    if a <= b {
        return a;
    }
    if a - b <= b {
        return a - b;
    }
    a = fast_segment_remainder(a, b + b);
    if a <= b {
        return a;
    }
    a - b
}

pub fn fast_segment_gcm(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    while a != b {
        a = fast_segment_remainder(a, b);
        std::mem::swap(&mut a, &mut b);
    }
    a
}

// Section 4.5

pub fn _fast_segment_remainder1(mut a: LineSegment, b: LineSegment) -> LineSegment {
    // precondition: b != 0
    if a < b {
        return a;
    }
    if a - b < b {
        return a - b;
    }
    a = _fast_segment_remainder1(a, b + b);
    if a < b {
        return a;
    }
    a - b
}

fn largest_doubling(a: LineSegment, mut b: LineSegment) -> LineSegment {
    // precondition: b != 0
    while a - b >= b {
        b += b;
    }
    b
}

pub fn remainder(mut a: LineSegment, b: LineSegment) -> LineSegment {
    // precondition: b != 0
    if a < b {
        return a;
    }
    let mut c = largest_doubling(a, b);
    a -= c;
    while c != b {
        c = half(c);
        if c <= a {
            a -= c;
        }
    }
    a
}

pub fn quotient(mut a: LineSegment, b: LineSegment) -> Integer {
    // Precondition: b > 0
    if a < b {
        return 0;
    }
    let mut c = largest_doubling(a, b);
    let mut n = 1;
    a -= c;
    while c != b {
        c = half(c);
        n += n;
        if c <= a {
            a -= c;
            n += 1;
        }
    }
    n
}

pub fn quotient_remainder(mut a: LineSegment, b: LineSegment) -> (Integer, LineSegment) {
    // Precondition: b > 0
    if a < b {
        return (0, a);
    }
    let mut c = largest_doubling(a, b);
    let mut n = 1;
    a -= c;
    while c != b {
        c = half(c);
        n += n;
        if c <= a {
            a -= c;
            n += 1;
        }
    }
    (n, a)
}

pub fn remainder_fibonacci(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    // Precondition: b > 0
    if a < b {
        return a;
    }
    let mut c = b;
    loop {
        let tmp = c;
        c += b;
        b = tmp;
        if a < c {
            break;
        }
    }
    loop {
        if a >= b {
            a -= b;
        }
        let tmp = c - b;
        c = b;
        b = tmp;
        if b >= c {
            break;
        }
    }
    a
}

pub fn gcm_remainder(mut a: LineSegment, mut b: LineSegment) -> LineSegment {
    while b != 0 {
        a = remainder(a, b);
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn gcd(mut a: Integer, mut b: Integer) -> Integer {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}
