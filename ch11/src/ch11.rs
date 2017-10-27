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
// ch11.rs -- Functions from Chapter 11 of fM2GP.
// -------------------------------------------------------------------

extern crate num_integer;
extern crate num_traits;
extern crate std;

pub trait InputIterator
where
    Self: Iterator,
{
}

impl<T> InputIterator for T
where
    T: Iterator,
{
}

pub trait ForwardIterator
where
    Self: InputIterator,
{
}

impl<T> ForwardIterator for T
where
    T: InputIterator,
{
}

pub trait BidirectionalIterator
where
    Self: DoubleEndedIterator,
{
}

impl<T> BidirectionalIterator for T
where
    T: DoubleEndedIterator,
{
}

pub trait RandomAccessIterator
where
    Self: BidirectionalIterator,
    Self: ExactSizeIterator,
{
}

impl<T> RandomAccessIterator for T
where
    T: BidirectionalIterator,
    T: ExactSizeIterator,
{
}

pub trait OutputIterator
where
    Self: Iterator,
{
}

impl<T> OutputIterator for T
where
    T: Iterator,
{
}

pub trait Integer
where
    Self: num_integer::Integer,
    Self: std::ops::Shr<Self, Output = Self>,
    Self: std::ops::ShrAssign,
    Self: std::ops::SubAssign,
{
}

impl<T> Integer for T
where
    T: num_integer::Integer,
    T: std::ops::Shr<T, Output = T>,
    T: std::ops::ShrAssign,
    T: std::ops::SubAssign,
{
}

pub trait EuclidianDomain
where
    Self: num_traits::Num,
    Self: Clone,
{
}

impl<T> EuclidianDomain for T
where
    T: num_traits::Num,
    Self: Clone,
{
}

pub trait Transformation
where
    Self: Fn(usize) -> usize,
{
}

impl<T> Transformation for T
where
    T: Fn(usize) -> usize,
{
}

type DifferenceType = usize;

fn odd<N>(n: &N) -> bool
where
    N: Integer,
{
    n.is_odd()
}

fn _half<N>(n: N) -> N
where
    N: Integer,
{
    n >> num_traits::one()
}

pub mod fmgp {

    // Section 11.1

    fn swap<T>(x: &mut T, y: &mut T) {
        ::std::mem::swap(x, y);
    }

    // Section 11.2

    // ValueType<I0> == ValueType<I1>
    pub fn swap_ranges<'a, I0, I1, T>(first0: I0, mut first1: I1) -> I1
    where
        I0: ::InputIterator<Item = &'a mut T>,
        I1: ::InputIterator<Item = I0::Item>,
        T: 'a,
    {
        for x in first0 {
            swap(x, first1.next().unwrap());
        }
        first1
    }

    pub fn swap_ranges_bounded<'a, I0, I1, T>(mut first0: I0, mut first1: I1) -> (I0, I1)
    where
        I0: ::InputIterator<Item = &'a mut T>,
        I1: ::InputIterator<Item = I0::Item>,
        T: 'a,
    {
        while let Some(x) = first0.next() {
            match first1.next() {
                Some(y) => swap(x, y),
                None => break,
            }
        }
        (first0, first1)
    }

    pub fn swap_ranges_n<'a, I0, I1, T, N>(mut first0: I0, mut first1: I1, mut n: N) -> (I0, I1)
    where
        I0: ::InputIterator<Item = &'a mut T>,
        I1: ::InputIterator<Item = I0::Item>,
        T: 'a,
        N: ::Integer,
    {
        while n != ::ch11::num_traits::zero() {
            swap(first0.next().unwrap(), first1.next().unwrap());
            n -= ::ch11::num_traits::one();
        }
        (first0, first1)
    }

    fn swap_ranges_slice<T>(
        slice: &mut [T],
        first0: usize,
        last0: usize,
        first1: usize,
        last1: usize,
    ) -> (usize, usize) {
        let min = ::std::cmp::min(last0 - first0, last1 - first1);
        let p = (first0 + min, first1 + min);
        let (left, right) = slice.split_at_mut(first1);
        swap_ranges(left[first0..p.0].iter_mut(), right[..min].iter_mut());
        p
    }

    // Section 11.3

    pub fn gries_mills_rotate<T>(slice: &mut [T], mut m: usize) {
        let (mut f, l) = (0, slice.len());
        // u = distance(f, m) && v = distance(m, l)
        if f == m || m == l {
            return;
        } // u == 0 || v == 0
        let mut p = swap_ranges_slice(slice, f, m, m, l);
        while p.0 != m || p.1 != l {
            if p.0 == m {
                // u < v
                f = m;
                m = p.1; // v = v - u
            } else {
                // v < u
                f = p.0; // u = u - v
            }
            p = swap_ranges_slice(slice, f, m, m, l);
        }
        // u == v
    }

    fn rotate_unguarded<T>(slice: &mut [T], mut f: usize, mut m: usize) {
        let l = slice.len();
        // assert(f != m && m != l)
        let mut p = swap_ranges_slice(slice, f, m, m, l);
        while p.0 != m || p.1 != l {
            f = p.0;
            if m == f {
                m = p.1;
            }
            p = swap_ranges_slice(slice, f, m, m, l);
        }
    }

    pub fn rotate_forward<T>(slice: &mut [T], mut m: usize) -> usize {
        let (mut f, l) = (0, slice.len());
        if f == m {
            return l;
        }
        if m == l {
            return f;
        }
        let mut p = swap_ranges_slice(slice, f, m, m, l);
        while p.0 != m || p.1 != l {
            if p.1 == l {
                rotate_unguarded(slice, p.0, m);
                return p.0;
            }
            f = m;
            m = p.1;
            p = swap_ranges_slice(slice, f, m, m, l);
        }
        m
    }

    // Section 11.4

    fn rotate_cycle_from<T, F>(slice: &mut [T], mut i: usize, from: F)
    where
        T: Clone,
        F: ::Transformation,
    {
        let tmp = slice[i].clone();
        let start = i;
        let mut j = from(i);
        while j != start {
            slice[i] = slice[j].clone();
            i = j;
            j = from(j);
        }
        slice[i] = tmp;
    }

    #[derive(Copy, Clone)]
    struct RotateTransform {
        plus: ::ch11::DifferenceType,
        minus: isize,
        m1: usize,
    }

    impl RotateTransform {
        fn new(f: usize, m: usize, l: usize) -> RotateTransform {
            RotateTransform {
                plus: m - f,
                minus: m as isize - l as isize,
                m1: f + (l - m),
            }
        }
        // m1 is dividing point between items moving forward and backward

        fn call(&self, i: usize) -> usize {
            (i as isize + if i < self.m1 {
                self.plus as isize
            } else {
                self.minus
            }) as usize
        }
    }


    fn gcd<E>(mut a: E, mut b: E) -> E
    where
        E: ::EuclidianDomain,
    {
        while b != ::ch11::num_traits::zero() {
            a = a % b.clone();
            swap(&mut a, &mut b);
        }
        a
    }

    pub fn rotate_random_access<T>(slice: &mut [T], m: usize) -> usize
    where
        T: Clone,
    {
        let (f, l) = (0, slice.len());
        if f == m {
            return l;
        }
        if m == l {
            return f;
        }
        let mut cycles = gcd(m - f, l - m);
        let rotator = RotateTransform::new(f, m, l);
        while cycles > 0 {
            cycles -= 1;
            rotate_cycle_from(slice, f + cycles, |x| rotator.call(x));
        }
        rotator.m1
    }

    // Section 11.5

    fn _reverse_bidirectional<I>(mut f: I)
    where
        I: ::BidirectionalIterator,
    {
        loop {
            let mut x = f.next();
            if x.is_none() {
                break;
            }
            let mut y = f.next_back();
            if y.is_none() {
                break;
            }
            swap(&mut x, &mut y);
        }
    }

    pub fn reverse_n<'a, T, I, N>(mut f: I, mut n: N)
    where
        I: ::BidirectionalIterator<Item = &'a mut T>,
        T: 'a,
        N: ::Integer,
    {
        n >>= ::ch11::num_traits::one();
        while n > ::ch11::num_traits::zero() {
            n -= ::ch11::num_traits::one();
            swap(f.next().unwrap(), f.next_back().unwrap());
        }
    }

    pub fn _reverse_random_access<'a, T, I>(f: I)
    where
        I: ::RandomAccessIterator<Item = &'a mut T>,
        T: 'a,
    {
        let n = f.len();
        reverse_n(f, n);
    }

    pub fn three_reverse_rotate<T>(slice: &mut [T], m: usize) {
        slice[..m].reverse();
        slice[m..].reverse();
        slice.reverse();
    }

    fn reverse_until<T>(slice: &mut [T], m: usize) -> (usize, usize) {
        let (mut f, mut l) = (0, slice.len());
        let (left, right) = slice.split_at_mut(m);
        while f != m && m != l {
            l -= 1;
            swap(&mut left[f], &mut right[l - m]);
            f += 1;
        }
        (f, l)
    }

    pub fn rotate_bidirectional<T>(slice: &mut [T], m: usize) -> usize {
        slice[..m].reverse();
        slice[m..].reverse();
        let p = reverse_until(slice, m);
        slice[p.0..p.1].reverse();
        if m == p.0 {
            return p.1;
        }
        p.0
    }

    pub fn reverse_recursive<T>(slice: &mut [T], mut f: usize, n: usize) -> usize {
        if n == 0 {
            return f;
        }
        if n == 1 {
            f += 1;
            return f;
        }
        let h = n >> 1;
        let mut m = reverse_recursive(slice, f, h);
        if ::ch11::odd(&n) {
            m += 1;
        }
        let last = reverse_recursive(slice, m, h);
        swap_ranges_slice(slice, f, m, m, m + h);
        last
    }

    fn _distance(f: usize, l: usize) -> usize {
        l - f
    }

    fn _reverse_forward<T>(slice: &mut [T], f: usize, l: usize) {
        reverse_recursive(slice, f, _distance(f, l));
    }

    // Section 11.6

    fn reverse_copy<'a, T, I, O>(mut l: I, mut result: O) -> O
    where
        T: Clone,
        T: 'a,
        I: ::BidirectionalIterator<Item = &'a T>,
        O: ::OutputIterator<Item = &'a mut T>,
    {
        while let Some(x) = l.next_back() {
            *result.next().unwrap() = x.clone();
        }
        result
    }

    fn copy_n<'a, T, I, N, O>(mut first: I, mut n: N, mut result: O) -> O
    where
        T: Clone,
        T: 'a,
        I: ::InputIterator<Item = &'a T>,
        N: ::Integer,
        O: ::OutputIterator<Item = &'a mut T>,
    {
        while n != ::ch11::num_traits::zero() {
            *result.next().unwrap() = first.next().unwrap().clone();
            n -= ::ch11::num_traits::one();
        }
        result
    }

    fn reverse_n_with_buffer<T, N>(slice: &mut [T], f: usize, n: N, buffer: &mut [T])
    where
        T: Clone,
        N: ::Integer,
    {
        copy_n(slice[f..].iter(), n, buffer.iter_mut());
        reverse_copy(buffer.iter(), slice[f..].iter_mut());
    }

    // Section 11.7

    fn advance(it: &mut usize, n: usize) {
        *it += n
    }

    pub fn reverse_n_adaptive<T>(slice: &mut [T], mut f: usize, n: usize, buffer: &mut [T]) -> usize
    where
        T: Clone,
    {
        if n == 0 {
            return f;
        }
        if n == 1 {
            f += 1;
            return f;
        }
        if n <= buffer.len() {
            reverse_n_with_buffer(slice, f, n, buffer);
            return n;
        }
        let h = n >> 1;
        let mut m = reverse_n_adaptive(slice, f, h, buffer);
        advance(&mut m, n & 1);
        let last = reverse_n_adaptive(slice, m, h, buffer);
        swap_ranges_slice(slice, f, m, m, m + h);
        last
    }

} // namespace
