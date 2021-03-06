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

pub mod fmgp {

    // Section 10.5

    extern crate num_integer;
    extern crate num_traits;
    extern crate std;

    pub trait Integer
    where
        Self: num_integer::Integer,
        Self: std::ops::Shr<Self, Output = Self>,
        Self: Clone,
    {
}

    impl<T> Integer for T
    where
        T: num_integer::Integer,
        T: std::ops::Shr<T, Output = T>,
        T: Clone,
    {
    }

    pub trait InputIterator
    where
        Self: PartialEq,
        Self: std::ops::Deref,
    {
        type DifferenceType: Integer;
        fn successor(&mut self);
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

    impl<I> InputIterator for IteratorAdapter<I>
    where
        I: std::iter::Iterator,
    {
        type DifferenceType = usize;
        fn successor(&mut self) {
            let next_is_none = {
                let &mut (ref mut value, ref mut iterator) =
                    self.value_and_iterator.as_mut().unwrap();
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

    pub fn begin<I>(into_iter: I) -> IteratorAdapter<I::IntoIter>
    where
        I: std::iter::IntoIterator,
    {
        let mut iterator = into_iter.into_iter();
        IteratorAdapter {
            value_and_iterator: iterator.next().and_then(|value| Some((value, iterator))),
        }
    }

    pub fn end<I>(_: I) -> IteratorAdapter<I::IntoIter>
    where
        I: std::iter::IntoIterator,
    {
        IteratorAdapter {
            value_and_iterator: None,
        }
    }


    #[derive(Clone)]
    pub struct SliceAdapter<'a, T>
    where
        T: 'a,
    {
        index: usize,
        slice: Option<&'a [T]>,
    }

    impl<'a, T> PartialEq for SliceAdapter<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            self.index == other.index
        }
    }

    pub fn begin_random_access<'a, T>(slice: &'a [T]) -> SliceAdapter<'a, T> {
        SliceAdapter {
            index: 0,
            slice: Some(slice),
        }
    }

    pub fn end_random_access<'a, T>(slice: &'a [T]) -> SliceAdapter<'a, T> {
        SliceAdapter {
            index: slice.len(),
            slice: None,
        }
    }

    impl<'a, T> InputIterator for SliceAdapter<'a, T> {
        type DifferenceType = usize;
        fn successor(&mut self) {
            self.index += 1;
        }
    }

    impl<'a, T> std::ops::Deref for SliceAdapter<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.slice.unwrap()[self.index]
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

    pub fn distance_input<I>(mut f: I, l: &I) -> I::DifferenceType
    where
        I: InputIterator,
    {
        // precondition: valid_range(f, l)
        let mut n = num_traits::zero();
        while &f != l {
            f.successor();
            n = n + num_traits::one();
        }
        n
    }

    pub fn distance_random_access<I>(f: I, l: I) -> I::DifferenceType
    where
        I: RandomAccessIterator,
        I::DifferenceType: From<usize>,
    {
        // precondition: valid_range(f, l)
        I::DifferenceType::from(l - f)
    }

    // Section 10.6

    pub fn advance_input<I>(x: &mut I, mut n: I::DifferenceType)
    where
        I: InputIterator,
    {
        while n != num_traits::zero() {
            n = n - num_traits::one();
            x.successor();
        }
    }

    pub fn advance_random_access<I>(x: &mut I, n: usize)
    where
        I: RandomAccessIterator,
    {
        *x += n
    }

    // Section 10.7

    pub fn find_if<I, P>(mut f: I, l: &I, mut p: P) -> I
    where
        I: InputIterator,
        P: Predicate<I::Target>,
        I::Target: Sized,
    {
        while &f != l && !p(&*f) {
            f.successor();
        }
        f
    }

    pub fn find_if_n<I, P>(mut f: I, mut n: I::DifferenceType, mut p: P) -> (I, I::DifferenceType)
    where
        I: InputIterator,
        P: Predicate<I::Target>,
        I::Target: Sized,
    {
        while n != num_traits::zero() && !p(&*f) {
            f.successor();
            n = n - num_traits::one();
        }
        (f, n)
    }

    // Section 10.8

    pub fn partition_point_n<I, P>(mut f: I, mut n: I::DifferenceType, mut p: P) -> I
    where
        I: ForwardIterator,
        P: Predicate<I::Target>,
        I::Target: Sized,
    {
        while n != num_traits::zero() {
            let mut middle = f.clone();
            let half = n.clone() >> num_traits::one();
            advance_input(&mut middle, half.clone());
            if !p(&*middle) {
                n = half;
            } else {
                middle.successor();
                f = middle;
                n = n - (half + num_traits::one());
            }
        }
        f
    }

    pub fn partition_point<I, P>(f: I, l: &I, p: P) -> I
    where
        I: ForwardIterator,
        P: Predicate<I::Target>,
        I::Target: Sized,
    {
        partition_point_n(f.clone(), distance_input(f, l), p)
    }

    pub fn lower_bound<I>(f: I, l: &I, a: &I::Target) -> I
    where
        I: ForwardIterator,
        I::Target: ::std::cmp::PartialOrd,
        I::Target: Sized,
    {
        partition_point(f, l, |x: &I::Target| x < a)
    }

    pub fn upper_bound<I>(f: I, l: &I, a: &I::Target) -> I
    where
        I: ForwardIterator,
        I::Target: ::std::cmp::PartialOrd,
        I::Target: Sized,
    {
        partition_point(f, l, |x: &I::Target| x <= a)
    }
} // namespace fmgp
