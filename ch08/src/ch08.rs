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
// ch08.rs -- Functions from Chapter 8 of fM2GP.
// -------------------------------------------------------------------

extern crate num_traits;
extern crate std;

pub trait Regular
where
    Self: Clone,
    Self: std::cmp::PartialEq,
    Self: std::cmp::PartialOrd,
{
}

pub trait AdditiveSemigroup
where
    Self: Regular,
    Self: std::ops::Add<Output = Self>,
{
}

pub trait AdditiveMonoid
where
    Self: AdditiveSemigroup,
    Self: num_traits::Zero,
{
}

pub trait MultiplicativeSemigroup
where
    Self: Regular,
    Self: std::ops::Mul<Output = Self>,
{
}

pub trait MultiplicativeMonoid
where
    Self: MultiplicativeSemigroup,
    Self: num_traits::One,
{
}

pub trait Semiring
where
    Self: AdditiveMonoid,
    Self: MultiplicativeMonoid,
{
}

macro_rules! arithmetic_types {
    ($($t:ty)*) => ($(
        impl Regular for $t {}
        impl AdditiveSemigroup for $t {}
        impl AdditiveMonoid for $t {}
        impl MultiplicativeSemigroup for $t {}
        impl MultiplicativeMonoid for $t {}
        impl Semiring for $t {}
    )*)
}

arithmetic_types!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64);

// Section 8.1

pub fn polynomial_value<I, R: Semiring>(first: I, x: &R) -> R
where
    I: IntoIterator<Item = R>,
{
    let mut iter = first.into_iter();
    match iter.next() {
        None => num_traits::zero(),
        Some(mut sum) => {
            for first in iter {
                sum = sum * x.clone();
                sum = sum + first;
            }
            sum
        }
    }
}
