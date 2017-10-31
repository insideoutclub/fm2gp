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
// ch10.rs -- Functions from Chapter 10 of fM2GP.
// -------------------------------------------------------------------

extern crate std;

#[derive(PartialEq, Debug, Clone)]
enum State {
    First,
    Last,
}

pub struct MyIterator<I>
where
    I: Iterator,
{
    state: State,
    value: Option<I::Item>,
    x: I,
}

impl<I> PartialEq for MyIterator<I>
where
    I: Iterator,
{
    fn eq(&self, _: &Self) -> bool {
        self.state == State::Last
    }
}

impl<I> InputIterator for MyIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type ValueType = I::Item;
    type DifferenceType = isize;
    fn successor(&mut self) {
        self.value = self.x.next();
        if self.value.is_none() {
            self.state = State::Last;
        }
    }
    fn source(&self) -> Self::ValueType {
        self.value.clone().unwrap()
    }
}

/*
#[derive(Clone, PartialEq)]
pub struct Wrapper<I>
where
    I: ExactSizeIterator,
    I: PartialEq,
{
    x: I,
}

impl<I> PartialEq for Wrapper<I>
where
    I: ExactSizeIterator,
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x
    }
}

impl<I> InputIterator for Wrapper<I>
where
    I: ExactSizeIterator,
{
    type ValueType = I::Item;
    type DifferenceType = isize;
    fn successor(&mut self) {
        self.value = self.x.next();
        if self.value.is_none() {
            self.state = State::Last;
        }
    }
    fn source(&self) -> Self::ValueType {
        self.value.clone().unwrap()
    }
}

impl<I> Clone for Wrapper<I>
where
    I: ExactSizeIterator,
{
    fn clone(&self) -> Self
    {}
}

impl<I> Iterator for Wrapper<I>
where
    I: ExactSizeIterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next()
    }
}

impl<I> ExactSizeIterator for Wrapper<I>
where
    I: ExactSizeIterator
{
}
*/

impl<I> Clone for MyIterator<I>
where
    I: Iterator,
    I::Item: Clone,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            value: self.value.clone(),
            x: self.x.clone(),
        }
    }
}

pub fn begin<I>(mut x: I) -> MyIterator<I>
where
    I: Iterator,
{
    let value = x.next();
    MyIterator {
        state: if value.is_some() {
            State::First
        } else {
            State::Last
        },
        value,
        x,
    }
}

pub fn end<I>(x: I) -> MyIterator<I>
where
    I: Iterator,
{
    MyIterator {
        state: State::Last,
        value: None,
        x,
    }
}

pub trait InputIterator
where
    Self: std::cmp::PartialEq,
{
    type ValueType;
    type DifferenceType;
    fn successor(&mut self);
    fn source(&self) -> Self::ValueType;
}

pub trait ForwardIterator
where
    Self: InputIterator,
    Self: Clone,
{
}

impl<T> ForwardIterator for T
where
    T: InputIterator,
    T: Clone,
{
}

pub trait RandomAccessIterator
where
    Self: ForwardIterator,
{
}

pub trait Predicate<T>
where
    Self: FnMut(&T) -> bool,
{
}

impl<T, U> Predicate<U> for T
where
    T: FnMut(&U) -> bool,
{
}

pub mod fmgp {

    // Section 10.5

    type DifferenceType = usize;

    pub fn distance_input<I>(mut f: I, l: &I) -> DifferenceType
    where
        I: ::InputIterator,
    {
        // precondition: valid_range(f, l)
        let mut n = 0;
        while &f != l {
            f.successor();
            n += 1;
        }
        n
    }

    /*
    pub fn distance_random_access<I>(f: I) -> DifferenceType
    where
        I: ::RandomAccessIterator,
    {
        f.len()
    }
    */

    // Section 10.6

    pub fn advance_input<I>(x: &mut I, mut n: DifferenceType)
    where
        I: ::InputIterator,
    {
        while n != 0 {
            n -= 1;
            x.successor();
        }
    }

    /*
    pub fn advance_random_access<T>(x: &[T], n: DifferenceType) -> &[T] {
        &x[n..]
    }
    */

    // Section 10.7

    pub fn find_if<I, P>(mut f: I, l: &I, mut p: P) -> I
    where
        I: ::InputIterator,
        P: ::Predicate<I::ValueType>,
    {
        while &f != l && !p(&f.source()) {
            f.successor();
        }
        f
    }

    pub fn find_if_n<I, P>(mut f: I, mut n: DifferenceType, mut p: P) -> (I, DifferenceType)
    where
        I: ::InputIterator,
        P: ::Predicate<I::ValueType>,
    {
        while n != 0 && !p(&f.source()) {
            f.successor();
            n -= 1;
        }
        (f, n)
    }

    // Section 10.8

    pub fn partition_point_n<I, P>(mut f: I, mut n: DifferenceType, mut p: P) -> I
    where
        I: ::ForwardIterator,
        P: ::Predicate<I::ValueType>,
    {
        while n != 0 {
            let mut middle = f.clone();
            let half = n >> 1;
            advance_input(&mut middle, half);
            if !p(&middle.source()) {
                n = half;
            } else {
                middle.successor();
                f = middle;
                n -= half + 1;
            }
        }
        f
    }

    pub fn partition_point<I, P>(f: I, l: &I, p: P) -> I
    where
        I: ::ForwardIterator,
        P: ::Predicate<I::ValueType>,
    {
        partition_point_n(f.clone(), distance_input(f, l), p)
    }

    pub fn lower_bound<I>(f: I, l: &I, a: &I::ValueType) -> I
    where
        I: ::ForwardIterator,
        I::ValueType: ::std::cmp::PartialOrd,
    {
        partition_point(f, l, |x: &I::ValueType| x < a)
    }

    pub fn upper_bound<I>(f: I, l: &I, a: &I::ValueType) -> I
    where
        I: ::ForwardIterator,
        I::ValueType: ::std::cmp::PartialOrd,
    {
        partition_point(f, l, |x: &I::ValueType| x <= a)
    }
} // namespace fmgp
