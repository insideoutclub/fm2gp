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
use self::num_integer::Integer;
use num_traits::one;

// Section 7.1

fn odd<N: Integer>(n: &N) -> bool {
    n.is_odd()
}

fn half<N: Integer>(n: N) -> N {
    n / (one::<N>() + one())
}

pub fn mult_acc4(mut r: i32, mut n: i32, mut a: i32) -> i32 {
    loop {
        if odd(&n) {
            r += a;
            if n == 1 {
                return r;
            }
        }
        n = half(n);
        a += a;
    }
}

pub fn multiply_accumulate0<A, N: Integer>(mut r: A, mut n: N, mut a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == one() {
                return r;
            }
        }
        n = half(n);
        a = &a + &a;
    }
}

// Section 7.3


pub fn multiply_accumulate<A, N: Integer>(mut r: A, mut n: N, mut a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == one() {
                return r;
            }
        }
        n = half(n);
        a = &a + &a;
    }
}


pub fn multiply_accumulate_semigroup<A, N: Integer>(mut r: A, mut n: N, mut a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = &r + &a;
            if n == one() {
                return r;
            }
        }
        n = half(n);
        a = &a + &a;
    }
}

pub fn multiply_semigroup<A, N: Integer>(mut n: N, mut a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = &a + &a;
        n = half(n);
    }
    if n == one() {
        return a;
    }
    let twice_a = &a + &a;
    multiply_accumulate_semigroup(a, half(n - one()), twice_a)
}


// Section 7.4

pub fn multiply_monoid<A, N: Integer>(n: N, a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    A: num_traits::Zero,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return num_traits::zero();
    }
    multiply_semigroup(n, a)
}

pub fn multiply_group<A, N: Integer>(mut n: N, mut a: A) -> A
where
    N: num_traits::Signed,
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

pub fn power_accumulate_semigroup<A, N: Integer>(mut r: A, mut a: A, mut n: N) -> A
where
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = &r * &a;
            if n == one() {
                return r;
            }
        }
        n = half(n);
        a = &a * &a;
    }
}

pub fn power_semigroup<A, N: Integer>(mut a: A, mut n: N) -> A
where
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = &a * &a;
        n = half(n);
    }
    if n == one() {
        return a;
    }
    let a_squared = &a * &a;
    power_accumulate_semigroup(a, a_squared, half(n - one()))
}

pub fn power_monoid<A, N: Integer>(a: A, n: N) -> A
where
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
    A: num_traits::One,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return one();
    };
    power_semigroup(a, n)
}

fn multiplicative_inverse<A>(a: A) -> A
where
    A: num_traits::One,
    A: std::ops::Div<Output = A>,
{
    one::<A>() / a
}

pub fn power_group<A, N: Integer>(mut a: A, mut n: N) -> A
where
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

pub trait SemigroupOperation<A> {
    fn apply(&self, x: &A, y: &A) -> A;
}

pub fn power_accumulate_semigroup_with_op<A, N: Integer, Op>(
    mut r: A,
    mut a: A,
    mut n: N,
    op: &Op,
) -> A
where
    Op: SemigroupOperation<A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = op.apply(&r, &a);
            if n == one() {
                return r;
            }
        }
        n = half(n);
        a = op.apply(&a, &a);
    }
}

pub fn power_semigroup_with_op<A, N: Integer, Op>(mut a: A, mut n: N, op: &Op) -> A
where
    Op: SemigroupOperation<A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = op.apply(&a, &a);
        n = half(n);
    }
    if n == one() {
        return a;
    }
    let twice_a = op.apply(&a, &a);
    power_accumulate_semigroup_with_op(a, twice_a, half(n - one()), op)
}

pub trait MonoidOperation<A>: SemigroupOperation<A> {
    fn identity_element(&self) -> A;
}

pub fn power_monoid_with_op<A, N: Integer, Op>(a: A, n: N, op: &Op) -> A
where
    Op: MonoidOperation<A>,
{
    // precondition(n >= 0);
    if n.is_zero() {
        return op.identity_element();
    }
    power_semigroup_with_op(a, n, op)
}

pub trait GroupOperation<A>: MonoidOperation<A> {
    fn inverse_operation(&self, &A) -> A;
}

pub fn power_group_with_op<A, N: Integer, Op>(mut a: A, mut n: N, op: &Op) -> A
where
    N: num_traits::Signed,
    Op: GroupOperation<A>,
{
    if n.is_negative() {
        n = -n;
        a = op.inverse_operation(&a);
    }
    power_monoid_with_op(a, n, op)
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
