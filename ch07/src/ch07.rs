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

pub trait Regular
where
    Self: Sized,
{
}

impl<T> Regular for T
where
    T: Sized,
{
}

// Section 7.1

pub trait Integer
where
    Self: num_integer::Integer,
    Self: num_traits::Signed,
    Self: std::ops::Shr<Self, Output = Self>,
{
    fn odd(&self) -> bool {
        self.is_odd()
    }

    fn half(self) -> Self {
        self >> num_traits::one()
    }
}

impl<T> Integer for T
where
    T: num_integer::Integer,
    T: num_traits::Signed,
    T: std::ops::Shr<T, Output = T>,
{
}

pub fn mult_acc4(mut r: i32, mut n: i32, mut a: i32) -> i32 {
    loop {
        if n.odd() {
            r += a;
            if n == 1 {
                return r;
            }
        }
        n = n.half();
        a += a;
    }
}

pub fn multiply_accumulate0<A, N>(mut r: A, mut n: N, mut a: A) -> A
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
    N: Integer,
{
    loop {
        if n.odd() {
            r = &r + &a;
            if n == num_traits::one() {
                return r;
            }
        }
        n = n.half();
        a = &a + &a;
    }
}

// Section 7.3


pub trait NoncommutativeAdditiveSemigroup
where
    Self: Regular,
{
    fn multiply_accumulate<N>(mut self, mut r: Self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Add<&'b Self, Output = Self>,
    {
        loop {
            if n.odd() {
                r = &r + &self;
                if n == num_traits::one() {
                    return r;
                }
            }
            n = n.half();
            self = &self + &self;
        }
    }

    fn multiply_accumulate_semigroup<N>(mut self, mut r: Self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Add<&'b Self, Output = Self>,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return r;
        }
        loop {
            if n.odd() {
                r = &r + &self;
                if n == num_traits::one() {
                    return r;
                }
            }
            n = n.half();
            self = &self + &self;
        }
    }

    fn multiply_semigroup<N>(mut self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Add<&'b Self, Output = Self>,
    {
        // precondition(n > 0);
        while !n.odd() {
            self = &self + &self;
            n = n.half();
        }
        if n == num_traits::one() {
            return self;
        }
        (&self + &self).multiply_accumulate_semigroup(self, (n - num_traits::one()).half())
    }
}

impl<T> NoncommutativeAdditiveSemigroup for T
where
    T: Regular,
{
}


// Section 7.4

pub trait NoncommutativeAdditiveMonoid
where
    Self: NoncommutativeAdditiveSemigroup,
    Self: num_traits::Zero,
{
    fn multiply_monoid<N>(self, n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Add<&'b Self, Output = Self>,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return num_traits::zero();
        }
        self.multiply_semigroup(n)
    }
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
    fn multiply_group<N>(mut self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Add<&'b Self, Output = Self>,
    {
        if n < num_traits::zero() {
            n = -n;
            self = -self;
        }
        self.multiply_monoid(n)
    }
}

impl<T> NoncommutativeAdditiveGroup for T
where
    T: NoncommutativeAdditiveMonoid,
    T: std::ops::Neg<Output = T>,
{
}

// Section 7.5

pub trait MultiplicativeSemigroup
where
    Self: Regular,
{
    fn power_accumulate_semigroup<N>(mut self, mut r: Self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Mul<&'b Self, Output = Self>,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return r;
        }
        loop {
            if n.odd() {
                r = &r * &self;
                if n == num_traits::one() {
                    return r;
                }
            }
            n = n.half();
            self = &self * &self;
        }
    }

    fn power_semigroup<N>(mut self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Mul<&'b Self, Output = Self>,
    {
        // precondition(n > 0);
        while !n.odd() {
            self = &self * &self;
            n = n.half();
        }
        if n == num_traits::one() {
            return self;
        }
        (&self * &self).power_accumulate_semigroup(self, (n - num_traits::one()).half())
    }
}

impl<T> MultiplicativeSemigroup for T
where
    T: Regular,
    for<'a, 'b> &'a T: std::ops::Mul<&'b T, Output = T>,
{
}

pub trait MultiplicativeMonoid
where
    Self: MultiplicativeSemigroup,
    Self: num_traits::One,
{
    fn power_monoid<N>(self, n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Mul<&'b Self, Output = Self>,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return num_traits::one();
        }
        self.power_semigroup(n)
    }
}

impl<T> MultiplicativeMonoid for T
where
    T: MultiplicativeSemigroup,
    T: num_traits::One,
{
}

pub trait MultiplicativeGroup
where
    Self: MultiplicativeMonoid,
    Self: std::ops::Div<Output = Self>,
{
    fn multiplicative_inverse(self) -> Self {
        num_traits::one::<Self>() / self
    }

    fn power_group<N>(mut self, mut n: N) -> Self
    where
        N: Integer,
        for<'a, 'b> &'a Self: std::ops::Mul<&'b Self, Output = Self>,
    {
        if n < num_traits::zero() {
            n = -n;
            self = self.multiplicative_inverse();
        }
        self.power_monoid(n)
    }
}

impl<T> MultiplicativeGroup for T
where
    T: MultiplicativeMonoid,
    T: std::ops::Div<Output = T>,
{
}

// Section 7.6

pub trait SemigroupOperation<A> {
    fn call(&self, &A, &A) -> A;

    fn power_accumulate_semigroup<N>(&self, mut r: A, mut a: A, mut n: N) -> A
    where
        N: Integer,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return r;
        }
        loop {
            if n.odd() {
                r = self.call(&r, &a);
                if n == num_traits::one() {
                    return r;
                }
            }
            n = n.half();
            a = self.call(&a, &a);
        }
    }

    fn power_semigroup<N>(&self, mut a: A, mut n: N) -> A
    where
        N: Integer,
    {
        // precondition(n > 0);
        while !n.odd() {
            a = self.call(&a, &a);
            n = n.half();
        }
        if n == num_traits::one() {
            return a;
        }
        let a_squared = self.call(&a, &a);
        self.power_accumulate_semigroup(a, a_squared, (n - num_traits::one()).half())
    }
}

pub struct Plus();

impl<A> SemigroupOperation<A> for Plus
where
    for<'a, 'b> &'a A: std::ops::Add<&'b A, Output = A>,
{
    fn call(&self, x: &A, y: &A) -> A {
        x + y
    }
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

pub struct _Multiplies();

impl<A> SemigroupOperation<A> for _Multiplies
where
    for<'a, 'b> &'a A: std::ops::Mul<&'b A, Output = A>,
{
    fn call(&self, x: &A, y: &A) -> A {
        x * y
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

pub trait MonoidOperation<A>
where
    Self: SemigroupOperation<A>,
{
    fn identity_element(&self) -> A;

    fn power_monoid<N>(&self, a: A, n: N) -> A
    where
        N: Integer,
    {
        // precondition(n >= 0);
        if n == num_traits::zero() {
            return self.identity_element();
        }
        self.power_semigroup(a, n)
    }
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

pub trait GroupOperation<A>
where
    Self: MonoidOperation<A>,
{
    fn inverse_operation(&self, A) -> A;

    fn power_group<N>(&self, mut a: A, mut n: N) -> A
    where
        N: Integer,
    {
        if n < num_traits::zero() {
            n = -n;
            a = self.inverse_operation(a);
        }
        self.power_monoid(a, n)
    }
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
