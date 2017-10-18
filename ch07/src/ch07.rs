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
// ch07.rs -- Functions from Chapter 7 of fM2GP.
// -------------------------------------------------------------------

extern crate num_integer;
extern crate num_traits;
extern crate std;

// Section 7.1

fn odd<N>(n: &N) -> bool
where
    N: num_integer::Integer,
{
    n.is_odd()
}

fn half<N>(n: &N) -> N
where
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    N: num_traits::One,
{
    n >> num_traits::one()
}

pub fn mult_acc4(mut r: i32, mut n: i32, mut a: i32) -> i32 {
    loop {
        if odd(&n) {
            r += a;
            if n == 1 {
                return r;
            }
        }
        n = half(&n);
        a += a;
    }
}

pub fn multiply_accumulate0<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(&n);
        a = &a + &a;
    }
}

// Section 7.3


pub fn multiply_accumulate<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(&n);
        a = &a + &a;
    }
}


pub fn multiply_accumulate_semigroup<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(&n);
        a = &a + &a;
    }
}

pub fn multiply_semigroup<A, N>(mut n: N, mut a: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = &a + &a;
        n = half(&n);
    }
    if n == num_traits::one() {
        return a;
    }
    let twice_a = &a + &a;
    multiply_accumulate_semigroup(a, half(&(n - num_traits::one())), twice_a)
}


// Section 7.4

pub fn multiply_monoid<A, N>(n: N, a: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    A: num_traits::Zero,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return num_traits::zero();
    }
    multiply_semigroup(n, a)
}

pub fn multiply_group<A, N>(mut n: N, mut a: A) -> A
where
    N: num_integer::Integer,
    N: num_traits::Signed,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    A: num_traits::Zero,
    A: num_traits::Signed,
{
    if n.is_negative() {
        n = -n;
        a = -a;
    }
    multiply_monoid(n, a)
}

// Section 7.5

pub fn power_accumulate_semigroup<A, N>(mut r: A, mut a: A, mut n: N) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = &r * &a;
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(&n);
        a = &a * &a;
    }
}

pub fn power_semigroup<A, N>(mut a: A, mut n: N) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = &a * &a;
        n = half(&n);
    }
    if n == num_traits::one() {
        return a;
    }
    let a_squared = &a * &a;
    power_accumulate_semigroup(a, a_squared, half(&(n - num_traits::one())))
}

pub fn power_monoid<A, N>(a: A, n: N) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
    A: num_traits::One,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return num_traits::one();
    };
    power_semigroup(a, n)
}

fn multiplicative_inverse<A>(a: A) -> A
where
    A: num_traits::One,
    A: std::ops::Div<Output = A>,
{
    num_traits::one::<A>() / a
}

pub fn power_group<A, N>(mut a: A, mut n: N) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    N: num_traits::Signed,
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
    A: num_traits::One,
    A: std::ops::Div<Output = A>,
{
    if n.is_negative() {
        n = -n;
        a = multiplicative_inverse(a);
    }
    power_monoid(a, n)
}

// Section 7.6

pub fn power_accumulate_semigroup_with_op<A, N, Op>(mut r: A, mut a: A, mut n: N, op: &Op) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    Op: Fn(&A, &A) -> A,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = op(&r, &a);
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(&n);
        a = op(&a, &a);
    }
}

pub fn power_semigroup_with_op<A, N, Op>(mut a: A, mut n: N, op: &Op) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    Op: Fn(&A, &A) -> A,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = op(&a, &a);
        n = half(&n);
    }
    if n == num_traits::one() {
        return a;
    }
    let twice_a = op(&a, &a);
    power_accumulate_semigroup_with_op(a, twice_a, half(&(n - num_traits::one())), &op)
}

pub fn power_monoid_with_op<A, N, Op>(a: A, n: N, op: &Op, identity_element: A) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    Op: Fn(&A, &A) -> A,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return identity_element;
    }
    power_semigroup_with_op(a, n, op)
}

pub fn power_group_with_op<A, N, Op, InverseOp>(
    mut a: A,
    mut n: N,
    op: &Op,
    inverse_operation: &InverseOp,
    identity_element: A,
) -> A
where
    N: num_integer::Integer,
    for<'a> &'a N: std::ops::Shr<N, Output = N>,
    N: num_traits::Signed,
    Op: Fn(&A, &A) -> A,
    InverseOp: Fn(&A) -> A,
{
    if n.is_negative() {
        n = -n;
        a = inverse_operation(&a);
    }
    power_monoid_with_op(a, n, op, identity_element)
}


// Section 7.7

pub fn fib0(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib0(n - 1) + fib0(n - 2)
}

pub fn fibonacci_iterative(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut v = (0, 1);
    for _i in 1..n {
        v = (v.1, v.0 + v.1);
    }
    v.1
}
