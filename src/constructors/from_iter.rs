use core::iter::{IntoIterator, Iterator};

use nougat::gat;

#[gat(Item)]
use crate::lending_iter::LendingIter;

pub struct LendingWrapper<I: Iterator> {
    iter: I,
}

#[gat]
impl<I: Iterator> LendingIter for LendingWrapper<I> {
    type Item<'a> = I::Item
        where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next()
    }
}

pub trait IntoLending {
    type LendingT: LendingIter;

    fn lending(self) -> Self::LendingT;
}

impl<I: Iterator> IntoLending for I {
    type LendingT = LendingWrapper<I>;

    fn lending(self) -> Self::LendingT {
        LendingWrapper { iter: self }
    }
}
