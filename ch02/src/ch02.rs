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
// ch02.rs -- Functions from Chapter 2 of fM2GP.
// -------------------------------------------------------------------

// Section 2.1

fn odd(n: i32) -> bool {
    n & 0x1 == 1
}
fn half(n: i32) -> i32 {
    n >> 1
}

pub fn multiply0(n: i32, a: i32) -> i32 {
    if n == 1 {
        return a;
    }
    multiply0(n - 1, a) + a
}

pub fn multiply1(n: i32, a: i32) -> i32 {
    if n == 1 {
        return a;
    }
    let mut result = multiply1(half(n), a + a);
    if odd(n) {
        result += a
    }
    result
}

pub fn multiply_by_15(a: i32) -> i32 {
    let b = (a + a) + a;
    let c = b + b;
    (c + c) + b
}

// Section 2.2

pub fn mult_acc0(r: i32, n: i32, a: i32) -> i32 {
    if n == 1 {
        return r + a;
    }
    if odd(n) {
        mult_acc0(r + a, half(n), a + a)
    } else {
        mult_acc0(r, half(n), a + a)
    }
}

pub fn mult_acc1(mut r: i32, n: i32, a: i32) -> i32 {
    if n == 1 {
        return r + a;
    }
    if odd(n) {
        r += a
    }
    mult_acc1(r, half(n), a + a)
}

pub fn mult_acc2(mut r: i32, n: i32, a: i32) -> i32 {
    if odd(n) {
        r += a;
        if n == 1 {
            return r;
        }
    }
    mult_acc2(r, half(n), a + a)
}

pub fn mult_acc3(mut r: i32, mut n: i32, mut a: i32) -> i32 {
    if odd(n) {
        r += a;
        if n == 1 {
            return r;
        }
    }
    n = half(n);
    a += a;
    mult_acc3(r, n, a)
}

pub fn mult_acc4(mut r: i32, mut n: i32, mut a: i32) -> i32 {
    loop {
        if odd(n) {
            r += a;
            if n == 1 {
                return r;
            }
        }
        n = half(n);
        a += a;
    }
}

pub fn multiply2(n: i32, a: i32) -> i32 {
    if n == 1 {
        return a;
    }
    mult_acc4(a, n - 1, a)
}

pub fn multiply3(mut n: i32, mut a: i32) -> i32 {
    while !odd(n) {
        a += a;
        n = half(n);
    }
    if n == 1 {
        return a;
    }
    mult_acc4(a, n - 1, a)
}

pub fn multiply4(mut n: i32, mut a: i32) -> i32 {
    while !odd(n) {
        a += a;
        n = half(n);
    }
    if n == 1 {
        return a;
    }
    // even(n - 1) => n - 1 != 1
    mult_acc4(a, half(n - 1), a + a)
}
