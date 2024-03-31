use core::iter::{IntoIterator, Iterator};

use crate::lending_iter::LendingIter;

pub struct FromIter<I: Iterator> {
    iter: I,
}

impl<I: Iterator> LendingIter for FromIter<I> {
    type Item<'a> = I::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next()
    }
}

/// Transform an iterator into a lending iterator that yields the same items.
pub fn from_iter<I: Iterator>(iter: I) -> FromIter<I> {
    FromIter { iter }
}

/// Transform an iterator into a lending iterator that yields the same items.
pub trait AsLendingIter {
    type AsLendingIter: LendingIter;

    fn as_lending(self) -> Self::AsLendingIter;
}

impl<I: Iterator> AsLendingIter for I {
    type AsLendingIter = FromIter<I>;

    fn as_lending(self) -> Self::AsLendingIter {
        from_iter(self)
    }
}

/// Transform an collection into a lending iterator that yields the same items.
pub trait IntoLendingIter {
    type IntoLendingIter: LendingIter;

    fn into_lending(self) -> Self::IntoLendingIter;
}

impl<T: IntoIterator> IntoLendingIter for T {
    type IntoLendingIter = FromIter<T::IntoIter>;

    fn into_lending(self) -> Self::IntoLendingIter {
        from_iter(self.into_iter())
    }
}

pub struct FromIterRef<I: Iterator> {
    iter: I,
    item: Option<I::Item>,
}

impl<I: Iterator> LendingIter for FromIterRef<I> {
    type Item<'a> = &'a I::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.item = self.iter.next();
        self.item.as_ref()
    }
}

/// Transform an iterator into a lending iterator that yields immutable references to each item.
pub fn from_iter_ref<I: Iterator>(iter: I) -> FromIterRef<I> {
    FromIterRef { iter, item: None }
}

/// Transform an iterator into a lending iterator that yields immutable references to each item.
pub trait AsLendingIterRef {
    type AsLendingIterRef: LendingIter;

    fn as_lending_ref(self) -> Self::AsLendingIterRef;
}

impl<I: Iterator> AsLendingIterRef for I {
    type AsLendingIterRef = FromIterRef<I>;

    fn as_lending_ref(self) -> Self::AsLendingIterRef {
        from_iter_ref(self)
    }
}

/// Transform an collection into a lending iterator that yields immutable references to each item.
pub trait IntoLendingIterRef {
    type IntoLendingIterRef: LendingIter;

    fn into_lending_ref(self) -> Self::IntoLendingIterRef;
}

impl<T: IntoIterator> IntoLendingIterRef for T {
    type IntoLendingIterRef = FromIterRef<T::IntoIter>;

    fn into_lending_ref(self) -> Self::IntoLendingIterRef {
        from_iter_ref(self.into_iter())
    }
}

pub struct FromIterMut<I: Iterator> {
    iter: I,
    item: Option<I::Item>,
}

impl<I: Iterator> LendingIter for FromIterMut<I> {
    type Item<'a> = &'a mut I::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.item = self.iter.next();
        self.item.as_mut()
    }
}

/// Transform an iterator into a lending iterator that yields mutable references to each item.
pub fn from_iter_mut<I: Iterator>(iter: I) -> FromIterMut<I> {
    FromIterMut { iter, item: None }
}

/// Transform an iterator into a lending iterator that yields mutable references to each item.
pub trait AsLendingIterMut {
    type AsLendingIterMut: LendingIter;

    fn as_lending_mut(self) -> Self::AsLendingIterMut;
}

impl<I: Iterator> AsLendingIterMut for I {
    type AsLendingIterMut = FromIterMut<I>;

    fn as_lending_mut(self) -> Self::AsLendingIterMut {
        from_iter_mut(self)
    }
}

/// Transform an collection into a lending iterator that yields mmutable references to each item.
pub trait IntoLendingIterMut {
    type IntoLendingIterMut: LendingIter;

    fn into_lending_mut(self) -> Self::IntoLendingIterMut;
}

impl<T: IntoIterator> IntoLendingIterMut for T {
    type IntoLendingIterMut = FromIterMut<T::IntoIter>;

    fn into_lending_mut(self) -> Self::IntoLendingIterMut {
        from_iter_mut(self.into_iter())
    }
}
