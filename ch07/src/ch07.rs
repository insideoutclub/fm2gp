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

pub trait NoncommutativeAdditiveMonoid
where
    Self: NoncommutativeAdditiveSemigroup,
    Self: num_traits::Zero,
{
}

impl<T> NoncommutativeAdditiveMonoid for T
where
    T: NoncommutativeAdditiveSemigroup,
    T: num_traits::Zero,
{
}

pub trait NoncommutativeAdditiveGroup
where
    Self: NoncommutativeAdditiveMonoid,
    Self: std::ops::Neg<Output = Self>,
{
}

impl<T> NoncommutativeAdditiveGroup for T
where
    T: NoncommutativeAdditiveMonoid,
    T: std::ops::Neg<Output = T>,
{
}

pub trait MultiplicativeSemigroup
where
    Self: Regular,
    Self: std::ops::Mul<Output = Self>,
{
}

impl<T> MultiplicativeSemigroup for T
where
    T: Regular,
    T: std::ops::Mul<Output = T>,
{
}

pub trait MultiplicativeMonoid
where
    Self: MultiplicativeSemigroup,
    Self: num_traits::One,
{
}

impl<T> MultiplicativeMonoid for T
where
    T: MultiplicativeSemigroup,
    T: num_traits::One,
{
}

pub trait Regular
where
    Self: Clone,
    Self: std::cmp::PartialEq,
{
}

impl<T> Regular for T
where
    T: Clone,
    T: std::cmp::PartialEq,
{
}

pub trait SemigroupOperation<A> {
    fn call(&self, A, A) -> A;
}

pub trait MonoidOperation<A>
where
    Self: SemigroupOperation<A>,
{
    fn identity_element(&self) -> A;
}

pub trait GroupOperation<A>
where
    Self: MonoidOperation<A>,
{
    fn inverse_operation(&self, A) -> A;
}

pub trait AdditiveGroup
where
    Self: NoncommutativeAdditiveGroup,
{
}

impl<T> AdditiveGroup for T
where
    T: NoncommutativeAdditiveGroup,
{
}

pub trait MultiplicativeGroup
where
    Self: MultiplicativeMonoid,
    Self: std::ops::Div<Output = Self>,
{
}

impl<T> MultiplicativeGroup for T
where
    T: MultiplicativeMonoid,
    T: std::ops::Div<Output = T>,
{
}

pub trait Integer
where
    Self: num_integer::Integer,
    Self: num_traits::Signed,
    Self: std::ops::Shr<Self, Output = Self>,
{
}

impl<T> Integer for T
where
    T: num_integer::Integer,
    T: num_traits::Signed,
    T: std::ops::Shr<T, Output = T>,
{
}

// Section 7.1

fn odd<N>(n: &N) -> bool
where
    N: Integer,
{
    n.is_odd()
}

fn half<N>(n: N) -> N
where
    N: Integer,
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
        n = half(n);
        a += a;
    }
}

pub fn multiply_accumulate0<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    A: Clone,
    A: std::ops::Add<Output = A>,
    N: Integer,
{
    loop {
        if odd(&n) {
            r = r + a.clone();
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = a.clone() + a;
    }
}

// Section 7.3

pub trait NoncommutativeAdditiveSemigroup
where
    Self: Regular,
    Self: std::ops::Add<Output = Self>,
{
}

impl<T> NoncommutativeAdditiveSemigroup for T
where
    T: Regular,
    T: std::ops::Add<Output = Self>,
{
}

pub fn multiply_accumulate<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    A: NoncommutativeAdditiveSemigroup,
    N: Integer,
{
    loop {
        if odd(&n) {
            r = r + a.clone();
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = a.clone() + a;
    }
}


pub fn multiply_accumulate_semigroup<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    A: NoncommutativeAdditiveSemigroup,
    N: Integer,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = r + a.clone();
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = a.clone() + a;
    }
}

pub fn multiply_semigroup<A, N>(mut n: N, mut a: A) -> A
where
    A: NoncommutativeAdditiveSemigroup,
    N: Integer,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = a.clone() + a;
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    multiply_accumulate_semigroup(a.clone(), half(n - num_traits::one()), a.clone() + a)
}


// Section 7.4

pub fn multiply_monoid<A, N>(n: N, a: A) -> A
where
    A: NoncommutativeAdditiveMonoid,
    N: Integer,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return num_traits::zero();
    }
    multiply_semigroup(n, a)
}

pub fn multiply_group<A, N>(mut n: N, mut a: A) -> A
where
    A: NoncommutativeAdditiveGroup,
    N: Integer,
{
    if n < num_traits::zero() {
        n = -n;
        a = -a;
    }
    multiply_monoid(n, a)
}

// Section 7.5

pub fn power_accumulate_semigroup<A, N>(mut r: A, mut a: A, mut n: N) -> A
where
    A: MultiplicativeSemigroup,
    N: Integer,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = r * a.clone();
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = a.clone() * a;
    }
}

pub fn power_semigroup<A, N>(mut a: A, mut n: N) -> A
where
    A: MultiplicativeSemigroup,
    N: Integer,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = a.clone() * a;
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    power_accumulate_semigroup(a.clone(), a.clone() * a, half(n - num_traits::one()))
}

pub fn power_monoid<A, N>(a: A, n: N) -> A
where
    A: MultiplicativeMonoid,
    N: Integer,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return num_traits::one();
    }
    power_semigroup(a, n)
}

fn multiplicative_inverse<A>(a: A) -> A
where
    A: MultiplicativeGroup,
{
    num_traits::one::<A>() / a
}

pub fn power_group<A, N>(mut a: A, mut n: N) -> A
where
    A: MultiplicativeGroup,
    N: Integer,
{
    if n < num_traits::zero() {
        n = -n;
        a = multiplicative_inverse(a);
    }
    power_monoid(a, n)
}

// Section 7.6

pub struct Plus();

impl<A> SemigroupOperation<A> for Plus
where
    A: std::ops::Add<Output = A>,
{
    fn call(&self, x: A, y: A) -> A {
        x + y
    }
}

pub struct _Multiplies();

impl<A> SemigroupOperation<A> for _Multiplies
where
    A: std::ops::Mul<Output = A>,
{
    fn call(&self, x: A, y: A) -> A {
        x * y
    }
}

pub fn power_accumulate_semigroup_with_op<A, N, Op>(mut r: A, mut a: A, mut n: N, op: &Op) -> A
where
    A: Regular,
    N: Integer,
    Op: SemigroupOperation<A>,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = op.call(r, a.clone());
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = op.call(a.clone(), a);
    }
}

pub fn power_semigroup_with_op<A, N, Op>(mut a: A, mut n: N, op: &Op) -> A
where
    A: Regular,
    N: Integer,
    Op: SemigroupOperation<A>,
{
    // precondition(n > 0);
    while !odd(&n) {
        a = op.call(a.clone(), a);
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    power_accumulate_semigroup_with_op(
        a.clone(),
        op.call(a.clone(), a),
        half(n - num_traits::one()),
        op,
    )
}

impl<T> MonoidOperation<T> for Plus
where
    Self: SemigroupOperation<T>,
    T: num_traits::Zero,
{
    fn identity_element(&self) -> T {
        num_traits::zero()
    }
}

impl<T> MonoidOperation<T> for _Multiplies
where
    Self: SemigroupOperation<T>,
    T: num_traits::One,
{
    fn identity_element(&self) -> T {
        num_traits::one()
    }
}

pub fn power_monoid_with_op<A, N, Op>(a: A, n: N, op: &Op) -> A
where
    A: Regular,
    N: Integer,
    Op: MonoidOperation<A>,
{
    // precondition(n >= 0);
    if n == num_traits::zero() {
        return op.identity_element();
    }
    power_semigroup_with_op(a, n, op)
}

impl<T> GroupOperation<T> for Plus
where
    Self: MonoidOperation<T>,
    T: AdditiveGroup,
{
    fn inverse_operation(&self, x: T) -> T {
        -x
    }
}

fn reciprocal<T>(x: T) -> T
where
    T: MultiplicativeGroup,
{
    num_traits::one::<T>() / x
}

impl<T> GroupOperation<T> for _Multiplies
where
    Self: MonoidOperation<T>,
    T: MultiplicativeGroup,
{
    fn inverse_operation(&self, x: T) -> T {
        reciprocal(x)
    }
}

pub fn power_group_with_op<A, N, Op>(mut a: A, mut n: N, op: &Op) -> A
where
    A: Regular,
    N: Integer,
    Op: GroupOperation<A>,
{
    if n < num_traits::zero() {
        n = -n;
        a = op.inverse_operation(a);
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
    for _ in 1..n {
        v = (v.1, v.0 + v.1);
    }
    v.1
}
