use crate::{
    adapters,
    constructors::from_fn::{self, FromFn},
    hkt::{extend_lifetime, OptionType, HKT},
};

pub trait LendingIter {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }

    fn nth<'a>(&'a mut self, n: usize) -> Option<Self::Item<'a>> {
        for _ in 0..n {
            self.next()?;
        }
        self.next()
    }

    fn step_by(self, step: usize) -> adapters::StepBy<Self>
    where
        Self: Sized,
    {
        adapters::StepBy {
            iter: self,
            step,
            first: true,
        }
    }

    fn chain<I>(self, other: I) -> adapters::Chain<Self, I>
    where
        Self: Sized,
        for<'a> I: 'a + LendingIter<Item<'a> = Self::Item<'a>>,
    {
        adapters::Chain {
            a: adapters::Fuse {
                iter: self,
                avail: true,
            },
            b: other,
        }
    }

    fn zip<I>(self, other: I) -> adapters::Zip<Self, I>
    where
        Self: Sized,
        I: LendingIter,
    {
        adapters::Zip { a: self, b: other }
    }

    fn map<F>(self, fun: F) -> adapters::Map<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut<(Self::Item<'a>,)>,
    {
        adapters::Map { iter: self, fun }
    }

    fn for_each<F>(mut self, mut fun: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Self::Item<'a>),
    {
        while let Some(v) = self.next() {
            fun(v);
        }
    }

    fn filter<F>(self, filter: F) -> adapters::Filter<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        adapters::Filter { iter: self, filter }
    }

    fn filter_map<F>(self, filter: F) -> adapters::FilterMap<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut<(Self::Item<'a>,)>,
        for<'a> <F as FnOnce<(Self::Item<'a>,)>>::Output: OptionType,
    {
        adapters::FilterMap { iter: self, filter }
    }

    fn enumerate(self) -> adapters::Enumerate<Self>
    where
        Self: Sized,
    {
        adapters::Enumerate {
            iter: self,
            count: 0,
        }
    }

    fn skip_while<P>(self, predicate: P) -> adapters::SkipWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        adapters::SkipWhile {
            iter: self,
            pred: predicate,
            done: false,
        }
    }

    fn take_while<P>(self, predicate: P) -> adapters::TakeWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        adapters::TakeWhile {
            iter: self,
            pred: predicate,
        }
    }

    fn map_while<B, P>(self, predicate: P) -> adapters::MapWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut<(Self::Item<'a>,)>,
        for<'a> <P as FnOnce<(Self::Item<'a>,)>>::Output: OptionType,
    {
        adapters::MapWhile {
            iter: self,
            pred: predicate,
        }
    }

    fn skip(self, n: usize) -> adapters::Skip<Self>
    where
        Self: Sized,
    {
        adapters::Skip { iter: self, n }
    }

    fn take(self, n: usize) -> adapters::Take<Self>
    where
        Self: Sized,
    {
        adapters::Take { iter: self, n }
    }

    fn scan<S, F>(self, initial_state: S, f: F) -> adapters::Scan<Self, S, F>
    where
        Self: Sized,
        F: for<'a> FnMut<(&'a mut S, Self::Item<'a>)>,
        for<'a> <F as FnOnce<(&'a mut S, Self::Item<'a>)>>::Output: OptionType,
    {
        adapters::Scan {
            iter: self,
            state: initial_state,
            scan: f,
        }
    }

    fn fuse(self) -> adapters::Fuse<Self>
    where
        Self: Sized,
    {
        adapters::Fuse {
            iter: self,
            avail: true,
        }
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}
