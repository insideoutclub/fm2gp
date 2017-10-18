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
// ch12.rs -- Functions from Chapter 12 of fM2GP.
// -------------------------------------------------------------------

extern crate num_integer;
extern crate num_traits;

// Section 12.1

fn even<N>(n: &N) -> bool
where
    N: num_integer::Integer,
{
    n.is_even()
}

pub fn stein_gcd<N>(mut m: N, mut n: N) -> N
where
    N: ::std::cmp::PartialOrd,
    N: num_traits::Zero,
    N: num_traits::Signed,
    N: num_integer::Integer,
    N: ::std::ops::ShrAssign,
    N: num_traits::One,
    N: for<'a> ::std::ops::Sub<&'a N, Output = N>,
    N: ::std::ops::Shl<i32, Output = N>,
{
    if m < num_traits::zero() {
        m = -m;
    }
    if n < num_traits::zero() {
        n = -n;
    }
    if m.is_zero() {
        return n;
    }
    if n.is_zero() {
        return m;
    }

    // m > 0 && n > 0

    let mut d_m = 0;
    while even(&m) {
        m >>= num_traits::one();
        d_m += 1;
    }

    let mut d_n = 0;
    while even(&n) {
        n >>= num_traits::one();
        d_n += 1;
    }

    // odd(m) && odd(n)

    while m != n {
        if n > m {
            ::std::mem::swap(&mut n, &mut m);
        }
        m = m - &n;
        loop {
            m >>= num_traits::one();
            if !even(&m) {
                break;
            }
        }
    }

    // m == n

    m << ::std::cmp::min(d_m, d_n)
}

// Section 12.4

fn _gcd<E>(mut a: E, mut b: E) -> E
where
    E: ::std::cmp::PartialEq,
    E: num_traits::Zero,
    E: for<'a> ::std::ops::Rem<&'a E, Output = E>,
{
    while b != num_traits::zero() {
        a = a % &b;
        ::std::mem::swap(&mut a, &mut b);
    }
    a
}

fn quotient_remainder<E>(a: &E, b: &E) -> (E, E)
where
    for<'a, 'b> &'a E: ::std::ops::Div<&'b E, Output = E>,
    for<'a, 'b> &'a E: ::std::ops::Rem<&'b E, Output = E>,
{
    (a / b, a % b)
}

pub fn extended_gcd<E>(mut a: E, mut b: E) -> (E, E)
where
    E: ::std::cmp::PartialEq,
    E: num_traits::One,
    E: num_traits::Zero,
    for<'a, 'b> &'a E: ::std::ops::Div<&'b E, Output = E>,
    for<'a, 'b> &'a E: ::std::ops::Rem<&'b E, Output = E>,
    E: ::std::ops::Sub<Output = E>,
    E: for<'b> ::std::ops::Mul<&'b E, Output = E>,
{
    let mut x0 = num_traits::one();
    let mut x1 = num_traits::zero();
    while b != num_traits::zero() {
        // compute new r and x
        let qr = quotient_remainder(&a, &b);
        let x2 = x0 - qr.0 * &x1;
        // shift r and x
        x0 = x1;
        x1 = x2;
        a = b;
        b = qr.1;
    }
    (x0, a)
}
