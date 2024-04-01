use nougat::{gat, Gat};

use crate::{
    adapters, fn_traits,
    hkt::{extend_lifetime, HKT},
};

pub type LendedItem<'lt, I> = Gat!(<I as LendingIter>::Item<'lt>);

#[gat]
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
        I: for<'b> LendingIter<Item<'b> = LendedItem<'b, Self>>,
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
        F: for<'a> fn_traits::Mapper<'a, LendedItem<'a, Self>>,
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
        for<'a> F: 'a + FnMut(&LendedItem<'a, Self>) -> bool,
    {
        adapters::Filter { iter: self, filter }
    }

    fn filter_map<F>(self, filter: F) -> adapters::FilterMap<Self, F>
    where
        Self: Sized,
        F: for<'a> fn_traits::OptionMapper<'a, LendedItem<'a, Self>>,
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
        P: for<'a> FnMut(&LendedItem<'a, Self>) -> bool,
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
        P: for<'a> FnMut(&LendedItem<'a, Self>) -> bool,
    {
        adapters::TakeWhile {
            iter: self,
            pred: predicate,
        }
    }

    fn map_while<B, P>(self, predicate: P) -> adapters::MapWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> fn_traits::OptionMapper<'a, LendedItem<'a, Self>>,
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
        F: for<'a> fn_traits::Scanner<'a, S, LendedItem<'a, Self>>,
    {
        adapters::Scan {
            iter: self,
            state: initial_state,
            scan: f,
        }
    }

    fn flat_map<J, F>(self, fun: F) -> adapters::FlatMap<Self, F, J>
    where
        Self: Sized,
        J: LendingIter,
        F: for<'a> FnMut(Self::Item<'a>) -> J,
    {
        adapters::FlatMap {
            iter: self,
            fun,
            curr: None,
        }
    }

    fn flatten<J>(self) -> adapters::Flatten<Self, J>
    where
        J: LendingIter,
        Self: Sized + for<'a> LendingIter<Item<'a> = J>,
    {
        adapters::Flatten {
            iter: self,
            curr: None,
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

    fn inspect<F>(self, fun: F) -> adapters::Inspect<Self, F>
    where
        F: for<'a> FnMut(&LendedItem<'a, Self>),
        Self: Sized,
    {
        adapters::Inspect { iter: self, fun }
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn try_fold(&mut self) {
        todo!()
    }

    fn try_for_each(&mut self) {
        todo!()
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: for<'a> FnMut(B, LendedItem<'a, Self>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: for<'a> FnMut(LendedItem<'a, Self>) -> bool,
    {
        while let Some(b) = self.next().map(&mut f) {
            if !b {
                return false;
            }
        }
        true
    }

    fn any<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: for<'a> FnMut(LendedItem<'a, Self>) -> bool,
    {
        while let Some(b) = self.next().map(&mut f) {
            if b {
                return true;
            }
        }
        false
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item<'_>>
    where
        Self: Sized,
        P: for<'a> FnMut(&LendedItem<'a, Self>) -> bool,
    {
        // SAFETY:
        // v is either dropped or returned by the end of the loop (polonius loop pattern)
        while let Some(v) =
            unsafe { extend_lifetime::<HKT!(Option<LendedItem<'_, Self>>)>(self.next()) }
        {
            if predicate(&v) {
                return Some(v);
            }
        }
        None
    }

    fn find_map<'a, B, P>(&'a mut self, mut f: P) -> Option<B>
    where
        Self: Sized,
        P: FnMut(LendedItem<'a, Self>) -> Option<B> + 'a,
    {
        // SAFETY:
        // v is either dropped or returned by the end of the loop (polonius loop pattern)
        while let Some(item) =
            unsafe { extend_lifetime::<HKT!(Option<LendedItem<'_, Self>>)>(self.next()) }
        {
            if let Some(v) = f(item) {
                return Some(v);
            }
        }
        None
    }

    fn position<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: for<'a> FnMut(LendedItem<'a, Self>) -> bool,
    {
        let mut i = 0usize;
        while let Some(b) = self.next().map(&mut predicate) {
            if b {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn copied(self)
    where
        Self: Sized,
    {
        todo!()
    }

    fn cloned(self)
    where
        Self: Sized,
    {
        todo!()
    }
}
