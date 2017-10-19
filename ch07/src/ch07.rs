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

impl NoncommutativeAdditiveMonoid for i32 {}

pub trait NoncommutativeAdditiveGroup
where
    Self: NoncommutativeAdditiveMonoid,
    Self: std::ops::Neg<Output = Self>,
{
}

impl NoncommutativeAdditiveGroup for i32 {}

pub trait MultiplicativeSemigroup
where
    Self: Clone,
    Self: std::ops::Mul<Output = Self>,
{
}

impl MultiplicativeSemigroup for i32 {}
impl MultiplicativeSemigroup for f64 {}

pub trait MultiplicativeMonoid
where
    Self: MultiplicativeSemigroup,
    Self: num_traits::One,
{
}

impl MultiplicativeMonoid for i32 {}
impl MultiplicativeMonoid for f64 {}

pub trait SemigroupOperation<A> {
    fn apply(&self, &A, &A) -> A;
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

impl AdditiveGroup for i32 {}

pub trait MultiplicativeGroup
where
    Self: MultiplicativeMonoid,
    Self: std::ops::Div<Output = Self>,
{
}

impl MultiplicativeGroup for f64 {}

pub trait Integer
where
    Self: num_integer::Integer,
    Self: num_traits::Signed,
{
}

impl Integer for i32 {}

// Section 7.1

fn odd<N: Integer>(n: &N) -> bool {
    n.is_odd()
}

fn half<N: Integer>(n: N) -> N {
    n / (num_traits::one::<N>() + num_traits::one())
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
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = &a + &a;
    }
}

// Section 7.3

pub trait NoncommutativeAdditiveSemigroup
where
    Self: Clone,
    Self: std::ops::Add<Output = Self>,
{
}

impl NoncommutativeAdditiveSemigroup for i32 {}


pub fn multiply_accumulate<A: NoncommutativeAdditiveSemigroup, N: Integer>(
    mut r: A,
    mut n: N,
    mut a: A,
) -> A {
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


pub fn multiply_accumulate_semigroup<A: NoncommutativeAdditiveSemigroup, N: Integer>(
    mut r: A,
    mut n: N,
    mut a: A,
) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
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

pub fn multiply_semigroup<A: NoncommutativeAdditiveSemigroup, N: Integer>(mut n: N, mut a: A) -> A {
    // precondition(n > 0);
    while !odd(&n) {
        a = a.clone() + a;
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    let twice_a = a.clone() + a.clone();
    multiply_accumulate_semigroup(a, half(n - num_traits::one()), twice_a)
}


// Section 7.4

pub fn multiply_monoid<A: NoncommutativeAdditiveMonoid, N: Integer>(n: N, a: A) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
        return num_traits::zero();
    }
    multiply_semigroup(n, a)
}

pub fn multiply_group<A: NoncommutativeAdditiveGroup, N: Integer>(mut n: N, mut a: A) -> A {
    if n.is_negative() {
        n = -n;
        a = -a;
    }
    multiply_monoid(n, a)
}

// Section 7.5

pub fn power_accumulate_semigroup<A: MultiplicativeSemigroup, N: Integer>(
    mut r: A,
    mut a: A,
    mut n: N,
) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
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

pub fn power_semigroup<A: MultiplicativeSemigroup, N: Integer>(mut a: A, mut n: N) -> A {
    // precondition(n > 0);
    while !odd(&n) {
        a = a.clone() * a;
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    let a_squared = a.clone() * a.clone();
    power_accumulate_semigroup(a, a_squared, half(n - num_traits::one()))
}

pub fn power_monoid<A: MultiplicativeMonoid, N: Integer>(a: A, n: N) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
        return num_traits::one();
    };
    power_semigroup(a, n)
}

fn multiplicative_inverse<A: MultiplicativeGroup>(a: A) -> A {
    num_traits::one::<A>() / a
}

pub fn power_group<A: MultiplicativeGroup, N: Integer>(mut a: A, mut n: N) -> A {
    if n.is_negative() {
        n = -n;
        a = multiplicative_inverse(a);
    }
    power_monoid(a, n)
}

// Section 7.6

pub struct Plus();

impl<A> SemigroupOperation<A> for Plus
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    fn apply(&self, x: &A, y: &A) -> A {
        x + y
    }
}

pub struct _Multiplies();

impl<A> SemigroupOperation<A> for _Multiplies
where
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    fn apply(&self, x: &A, y: &A) -> A {
        x * y
    }
}

pub fn power_accumulate_semigroup_with_op<A, N: Integer, Op: SemigroupOperation<A>>(
    mut r: A,
    mut a: A,
    mut n: N,
    op: &Op,
) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
        return r;
    }
    loop {
        if odd(&n) {
            r = op.apply(&r, &a);
            if n == num_traits::one() {
                return r;
            }
        }
        n = half(n);
        a = op.apply(&a, &a);
    }
}

pub fn power_semigroup_with_op<A, N: Integer, Op: SemigroupOperation<A>>(
    mut a: A,
    mut n: N,
    op: &Op,
) -> A {
    // precondition(n > 0);
    while !odd(&n) {
        a = op.apply(&a, &a);
        n = half(n);
    }
    if n == num_traits::one() {
        return a;
    }
    let twice_a = op.apply(&a, &a);
    power_accumulate_semigroup_with_op(a, twice_a, half(n - num_traits::one()), op)
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

pub fn power_monoid_with_op<A, N: Integer, Op: MonoidOperation<A>>(a: A, n: N, op: &Op) -> A {
    // precondition(n >= 0);
    if n.is_zero() {
        return op.identity_element();
    }
    power_semigroup_with_op(a, n, op)
}

impl<T: AdditiveGroup> GroupOperation<T> for Plus
where
    Self: MonoidOperation<T>,
{
    fn inverse_operation(&self, x: T) -> T {
        -x
    }
}

fn reciprocal<T: MultiplicativeGroup>(x: T) -> T {
    num_traits::one::<T>() / x
}

impl<T: MultiplicativeGroup> GroupOperation<T> for _Multiplies
where
    Self: MonoidOperation<T>,
{
    fn inverse_operation(&self, x: T) -> T {
        reciprocal(x)
    }
}

pub fn power_group_with_op<A, N: Integer, Op: GroupOperation<A>>(mut a: A, mut n: N, op: &Op) -> A {
    if n.is_negative() {
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
    for _i in 1..n {
        v = (v.1, v.0 + v.1);
    }
    v.1
}
