use core::marker::PhantomData;

use crate::hkt::HKT;
use crate::lending_iter::LendingIter;

pub struct Empty<T: HKT> {
    phantom: PhantomData<T>,
}

impl<T> LendingIter for Empty<T>
where
    T: HKT,
{
    type Item<'a> = T::With<'a>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        None
    }
}

pub const fn empty<T: HKT>() -> Empty<T> {
    Empty {
        phantom: PhantomData,
    }
}
