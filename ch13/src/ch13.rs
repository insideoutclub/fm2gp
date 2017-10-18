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
// ch13.rs -- Functions from Chapter 13 of fM2GP.
// -------------------------------------------------------------------

extern crate num_integer;
extern crate num_traits;
use ch07::*;
use ch12::*;

// Section 13.2

fn divides<I>(i: &I, n: &I) -> bool
where
    for<'a, 'b> &'a I: ::std::ops::Rem<&'b I, Output = I>,
    I: num_traits::Zero,
{
    (n % i).is_zero()
}

fn smallest_divisor<I>(n: I) -> I
where
    I: num_integer::Integer,
    for<'a, 'b> &'a I: ::std::ops::Rem<&'b I, Output = I>,
    for<'a> &'a I: ::std::ops::Add<Output = I>,
{
    // precondition: n > 0
    let two = num_traits::one::<I>() + num_traits::one::<I>();
    if n.is_even() {
        return two;
    }
    let mut i = &two + &num_traits::one();
    while n >= i {
        if divides(&i, &n) {
            return i;
        }
        i = &i + &two;
    }
    n
}

pub fn is_prime<I>(n: &I) -> I
where
    I: ::std::cmp::PartialOrd,
    I: num_traits::One,
    I: num_integer::Integer,
    for<'a, 'b> &'a I: ::std::ops::Rem<&'b I, Output = I>,
    for<'a> &'a I: ::std::ops::Add<Output = I>,
    for<'a> &'a I: ::std::cmp::PartialEq,
    I: Clone,
{
    if *n > num_traits::one() && &smallest_divisor(n.clone()) == n {
        num_traits::one()
    } else {
        num_traits::zero()
    }
}

struct ModuloMultiply<I> {
    modulus: I,
}

impl<I> ModuloMultiply<I> {
    fn new(i: I) -> ModuloMultiply<I> {
        ModuloMultiply { modulus: i }
    }

    fn apply(&self, n: &I, m: &I) -> I
    where
        for<'a, 'b> &'a I: ::std::ops::Mul<&'b I, Output = I>,
        I: for<'a> ::std::ops::Rem<&'a I, Output = I>,
    {
        (n * m) % &self.modulus
    }
}

fn identity_element<I>(_: ModuloMultiply<I>) -> I
where
    I: num_traits::One,
{
    num_traits::one()
}

pub fn multiplicative_inverse_fermat<I>(a: I, p: I) -> I
where
    I: num_traits::One,
    I: ::std::ops::Sub<Output = I>,
    I: num_integer::Integer,
    I: ::std::ops::Shr<i32, Output = I>,
    for<'a, 'b> &'a I: ::std::ops::Mul<&'b I, Output = I>,
    I: Clone,
    I: for<'a> ::std::ops::Rem<&'a I, Output = I>,
{
    // precondition: p is prime & a > 0
    let multiplier = ModuloMultiply::new(p.clone());
    let two = num_traits::one::<I>() + num_traits::one();
    power_monoid_with_op(
        a,
        p - two,
        &|x, y| multiplier.apply(x, y),
        num_traits::one(),
    )
}

pub fn fermat_test<I>(n: I, witness: I) -> bool
where
    I: ::std::ops::Sub<Output = I>,
    I: num_traits::One,
    I: num_integer::Integer,
    I: ::std::ops::Shr<i32, Output = I>,
    for<'a, 'b> &'a I: ::std::ops::Mul<&'b I, Output = I>,
    for<'a> I: ::std::ops::Rem<&'a I, Output = I>,
    I: Clone,
{
    // precondition: 0 < witness < n
    let multiplier = ModuloMultiply::new(n.clone());
    let remainder = power_semigroup_with_op(
        witness,
        n - num_traits::one(),
        &|x, y| multiplier.apply(x, y),
    );
    remainder == num_traits::one()
}

// Section 13.3

pub fn miller_rabin_test<I>(n: &I, q: I, k: &I, witness: I) -> bool
where
    I: num_integer::Integer,
    I: ::std::ops::Shr<i32, Output = I>,
    for<'a, 'b> &'a I: ::std::ops::Mul<&'b I, Output = I>,
    for<'a> I: ::std::ops::Rem<&'a I, Output = I>,
    I: ::std::ops::AddAssign,
    I: Clone,
    for<'a> &'a I: ::std::ops::Sub<I, Output = I>,
{
    // precondition n > 1 && n - 1 = 2^kq && q is odd

    let mmult = ModuloMultiply::new(n.clone());
    let mut x = power_semigroup_with_op(witness, q, &|x, y| mmult.apply(x, y));
    if x == num_traits::one() || x == n - num_traits::one() {
        return true;
    }
    let mut index = num_traits::one::<I>();
    while index < *k {
        // invariant x = w^{2^{i-1}q}

        x = mmult.apply(&x, &x);
        if x == n - num_traits::one() {
            return true;
        }
        if x == num_traits::one() {
            return false;
        }
        index += num_traits::one();
    }
    false
}

// Section 13.4

pub fn multiplicative_inverse<I>(a: I, n: I) -> I
where
    I: ::std::cmp::PartialEq,
    I: num_traits::One,
    I: num_traits::Zero,
    for<'a, 'b> &'a I: ::std::ops::Div<&'b I, Output = I>,
    for<'a, 'b> &'a I: ::std::ops::Rem<&'b I, Output = I>,
    I: ::std::ops::Sub<Output = I>,
    I: for<'a> ::std::ops::Mul<&'a I, Output = I>,
    I: ::std::cmp::PartialOrd,
    I: Clone,
{
    let p = extended_gcd(a, n.clone());
    if p.1 != num_traits::one() {
        return num_traits::zero();
    }
    if p.0 < num_traits::zero() {
        return p.0 + n;
    }
    p.0
}
