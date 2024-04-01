use core::marker::PhantomData;

use nougat::gat;

use crate::hkt::HKT;
#[gat(Item)]
use crate::lending_iter::LendingIter;

pub struct Empty<T: HKT> {
    phantom: PhantomData<T>,
}

#[gat]
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
