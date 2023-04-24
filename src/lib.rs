#![no_std]
//! Collection of iterator adapter creation functions that act like their so-named [`Iterator`] method counterparts,
//! but they take any instance of [`IntoIterator`] (which includes iterators and mutable references to them),
//! allowing you to choose whether to call [`IntoIterator::into_iter`] or [`Iterator::by_ref`] explicitly.
//!
//! # Examples
//!
//! ```
//! use iia::chain;
//! let mut range = 0..10;
//! let mut iter = chain([1, 2, 3], &mut range);
//! iter.nth(5);
//! assert_eq!(range, 3..10);
//! ```
//!
//! ```
//! use iia::rev;
//! for (i, j) in rev([1, 2, 3]).enumerate() {
//!     assert_eq!(i, 3 - j);
//! }
//! ```

use core::iter::{
    Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect,
    Map, MapWhile, Peekable, Rev, Scan, Skip, SkipWhile, StepBy, Take, TakeWhile, Zip,
};

/// [`IntoIterator`]-enabled version of [`Iterator::step_by`].
pub fn step_by<I: IntoIterator>(iter: I, step: usize) -> StepBy<I::IntoIter> {
    iter.into_iter().step_by(step)
}

/// [`IntoIterator`]-enabled version of [`Iterator::chain`].
pub fn chain<A: IntoIterator, B: IntoIterator<Item = A::Item>>(
    a: A,
    b: B,
) -> Chain<A::IntoIter, B::IntoIter> {
    a.into_iter().chain(b)
}

/// [`IntoIterator`]-enabled version of [`Iterator::zip`].
pub fn zip<A: IntoIterator, B: IntoIterator>(a: A, b: B) -> Zip<A::IntoIter, B::IntoIter> {
    a.into_iter().zip(b)
}

/// [`IntoIterator`]-enabled version of [`Iterator::map`].
pub fn map<I: IntoIterator, B, F: FnMut(I::Item) -> B>(iter: I, f: F) -> Map<I::IntoIter, F> {
    iter.into_iter().map(f)
}

/// [`IntoIterator`]-enabled version of [`Iterator::filter`].
pub fn filter<I: IntoIterator, P: FnMut(&I::Item) -> bool>(
    iter: I,
    predicate: P,
) -> Filter<I::IntoIter, P> {
    iter.into_iter().filter(predicate)
}

/// [`IntoIterator`]-enabled version of [`Iterator::filter_map`].
pub fn filter_map<I: IntoIterator, B, F: FnMut(I::Item) -> Option<B>>(
    iter: I,
    f: F,
) -> FilterMap<I::IntoIter, F> {
    iter.into_iter().filter_map(f)
}

/// [`IntoIterator`]-enabled version of [`Iterator::enumerate`].
pub fn enumerate<I: IntoIterator>(iter: I) -> Enumerate<I::IntoIter> {
    iter.into_iter().enumerate()
}

/// [`IntoIterator`]-enabled version of [`Iterator::peekable`].
pub fn peekable<I: IntoIterator>(iter: I) -> Peekable<I::IntoIter> {
    iter.into_iter().peekable()
}

/// [`IntoIterator`]-enabled version of [`Iterator::skip_while`].
pub fn skip_while<I: IntoIterator, P: FnMut(&I::Item) -> bool>(
    iter: I,
    predicate: P,
) -> SkipWhile<I::IntoIter, P> {
    iter.into_iter().skip_while(predicate)
}

/// [`IntoIterator`]-enabled version of [`Iterator::take_while`].
pub fn take_while<I: IntoIterator, P: FnMut(&I::Item) -> bool>(
    iter: I,
    predicate: P,
) -> TakeWhile<I::IntoIter, P> {
    iter.into_iter().take_while(predicate)
}

/// [`IntoIterator`]-enabled version of [`Iterator::map_while`].
pub fn map_while<I: IntoIterator, B, P: FnMut(I::Item) -> Option<B>>(
    iter: I,
    predicate: P,
) -> MapWhile<I::IntoIter, P> {
    iter.into_iter().map_while(predicate)
}

/// [`IntoIterator`]-enabled version of [`Iterator::skip`].
pub fn skip<I: IntoIterator>(iter: I, n: usize) -> Skip<I::IntoIter> {
    iter.into_iter().skip(n)
}

/// [`IntoIterator`]-enabled version of [`Iterator::take`].
pub fn take<I: IntoIterator>(iter: I, n: usize) -> Take<I::IntoIter> {
    iter.into_iter().take(n)
}

/// [`IntoIterator`]-enabled version of [`Iterator::scan`].
pub fn scan<I: IntoIterator, St, B, F: FnMut(&mut St, I::Item) -> Option<B>>(
    iter: I,
    initial_state: St,
    f: F,
) -> Scan<I::IntoIter, St, F> {
    iter.into_iter().scan(initial_state, f)
}

/// [`IntoIterator`]-enabled version of [`Iterator::flat_map`].
pub fn flat_map<I: IntoIterator, U: IntoIterator, F: FnMut(I::Item) -> U>(
    iter: I,
    f: F,
) -> FlatMap<I::IntoIter, U, F> {
    iter.into_iter().flat_map(f)
}

/// [`IntoIterator`]-enabled version of [`Iterator::flatten`].
pub fn flatten<I: IntoIterator>(iter: I) -> Flatten<I::IntoIter>
where
    I::Item: IntoIterator,
{
    iter.into_iter().flatten()
}

/// [`IntoIterator`]-enabled version of [`Iterator::fuse`].
pub fn fuse<I: IntoIterator>(iter: I) -> Fuse<I::IntoIter> {
    iter.into_iter().fuse()
}

/// [`IntoIterator`]-enabled version of [`Iterator::inspect`].
pub fn inspect<I: IntoIterator, F: FnMut(&I::Item)>(iter: I, f: F) -> Inspect<I::IntoIter, F> {
    iter.into_iter().inspect(f)
}

/// [`IntoIterator`]-enabled version of [`Iterator::rev`].
pub fn rev<I: IntoIterator>(iter: I) -> Rev<I::IntoIter>
where
    I::IntoIter: DoubleEndedIterator,
{
    iter.into_iter().rev()
}

/// [`IntoIterator`]-enabled version of [`Iterator::copied`].
pub fn copied<'a, T: 'a + Copy, I: IntoIterator<Item = &'a T>>(iter: I) -> Copied<I::IntoIter> {
    iter.into_iter().copied()
}

/// [`IntoIterator`]-enabled version of [`Iterator::cloned`].
pub fn cloned<'a, T: 'a + Clone, I: IntoIterator<Item = &'a T>>(iter: I) -> Cloned<I::IntoIter> {
    iter.into_iter().cloned()
}

/// [`IntoIterator`]-enabled version of [`Iterator::cycle`].
pub fn cycle<I: IntoIterator>(iter: I) -> Cycle<I::IntoIter>
where
    I::IntoIter: Clone,
{
    iter.into_iter().cycle()
}
