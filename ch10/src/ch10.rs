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

pub trait Iterator {
    type DifferenceType;
    fn successor(&mut self);
}

pub trait InputIterator
where
    Self: PartialEq,
    Self: Iterator,
    Self: std::ops::Deref,
{
    type ValueType;
}

impl<I> InputIterator for I
where
    I: PartialEq,
    I: Iterator,
    I: std::ops::Deref,
    I::Target: Sized,
{
    type ValueType = I::Target;
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
    Self: std::ops::Sub<Output = usize>,
    Self: std::ops::AddAssign<usize>,
{
}

impl<T> RandomAccessIterator for T
where
    T: ForwardIterator,
    T: std::ops::Sub<Output = usize>,
    T: std::ops::AddAssign<usize>,
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

#[derive(Clone)]
pub struct IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    value_and_iterator: Option<(I::Item, I)>,
}

impl<I> PartialEq for IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    fn eq(&self, _: &Self) -> bool {
        self.value_and_iterator.is_none()
    }
}

impl<I> Iterator for IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    type DifferenceType = isize;
    fn successor(&mut self) {
        let next_is_none = {
            let &mut (ref mut value, ref mut iterator) = self.value_and_iterator.as_mut().unwrap();
            iterator.next().map_or(true, |x| {
                *value = x;
                false
            })
        };
        if next_is_none {
            self.value_and_iterator = None
        }
    }
}

impl<I> std::ops::Deref for IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    type Target = I::Item;
    fn deref(&self) -> &Self::Target {
        &self.value_and_iterator.as_ref().unwrap().0
    }
}

pub fn begin<I>(mut iterator: I) -> IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    IteratorAdapter {
        value_and_iterator: iterator.next().and_then(|value| Some((value, iterator))),
    }
}

pub fn end<I>(_: I) -> IteratorAdapter<I>
where
    I: std::iter::Iterator,
{
    IteratorAdapter {
        value_and_iterator: None,
    }
}


#[derive(Clone, PartialEq)]
pub struct SliceAdapter<'a, T>
where
    T: 'a,
{
    index: usize,
    slice: &'a [T],
}

impl<'a, T> SliceAdapter<'a, T> {
    pub fn new(index: usize, slice: &'a [T]) -> Self {
        Self { index, slice }
    }
}

impl<'a, T> Iterator for SliceAdapter<'a, T> {
    type DifferenceType = isize;
    fn successor(&mut self) {
        self.index += 1;
    }
}

impl<'a, T> std::ops::Deref for SliceAdapter<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.slice[self.index]
    }
}

impl<'a, T> std::ops::Sub for SliceAdapter<'a, T> {
    type Output = usize;
    fn sub(self, other: Self) -> Self::Output {
        self.index - other.index
    }
}

impl<'a, T> std::ops::AddAssign<usize> for SliceAdapter<'a, T> {
    fn add_assign(&mut self, n: usize) {
        self.index += n
    }
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

    pub fn distance_random_access<I>(f: I, l: I) -> DifferenceType
    where
        I: ::RandomAccessIterator,
    {
        // precondition: valid_range(f, l)
        l - f
    }

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

    pub fn advance_random_access<I>(x: &mut I, n: usize)
    where
        I: ::RandomAccessIterator,
    {
        *x += n
    }

    // Section 10.7

    pub fn find_if<I, P>(mut f: I, l: &I, mut p: P) -> I
    where
        I: ::InputIterator,
        P: ::Predicate<I::Target>,
        I::Target: Sized,
    {
        while &f != l && !p(&*f) {
            f.successor();
        }
        f
    }

    pub fn find_if_n<I, P>(mut f: I, mut n: DifferenceType, mut p: P) -> (I, DifferenceType)
    where
        I: ::InputIterator,
        P: ::Predicate<I::Target>,
        I::Target: Sized,
    {
        while n != 0 && !p(&*f) {
            f.successor();
            n -= 1;
        }
        (f, n)
    }

    // Section 10.8

    pub fn partition_point_n<I, P>(mut f: I, mut n: DifferenceType, mut p: P) -> I
    where
        I: ::ForwardIterator,
        P: ::Predicate<I::Target>,
        I::Target: Sized,
    {
        while n != 0 {
            let mut middle = f.clone();
            let half = n >> 1;
            advance_input(&mut middle, half);
            if !p(&*middle) {
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
        P: ::Predicate<I::Target>,
        I::Target: Sized,
    {
        partition_point_n(f.clone(), distance_input(f, l), p)
    }

    pub fn lower_bound<I>(f: I, l: &I, a: &I::Target) -> I
    where
        I: ::ForwardIterator,
        I::Target: ::std::cmp::PartialOrd,
        I::Target: Sized,
    {
        partition_point(f, l, |x: &I::Target| x < a)
    }

    pub fn upper_bound<I>(f: I, l: &I, a: &I::Target) -> I
    where
        I: ::ForwardIterator,
        I::Target: ::std::cmp::PartialOrd,
        I::Target: Sized,
    {
        partition_point(f, l, |x: &I::Target| x <= a)
    }
} // namespace fmgp
