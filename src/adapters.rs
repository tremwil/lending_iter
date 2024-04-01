use nougat::{apply, gat, Gat};

#[gat(Item)]
use crate::lending_iter::LendingIter;
use crate::{
    fn_traits::{Mapper, OptionMapper, Scanner},
    hkt::{extend_lifetime, HKT},
    lending_iter::LendedItem,
};

pub struct StepBy<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) step: usize,
    pub(crate) first: bool,
}

#[gat]
impl<I: LendingIter> LendingIter for StepBy<I> {
    type Item<'a> = LendedItem<'a, I>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.first {
            true => {
                self.first = false;
                self.iter.next()
            }
            false => self.iter.nth(self.step - 1),
        }
    }
}

#[apply(Gat!)]
pub struct Chain<I, J>
where
    I: LendingIter,
    J: for<'a> LendingIter<Item<'a> = LendedItem<'a, I>>,
{
    pub(crate) a: Fuse<I>,
    pub(crate) b: J,
}

#[gat]
impl<I, J> LendingIter for Chain<I, J>
where
    I: LendingIter,
    J: for<'b> LendingIter<Item<'b> = LendedItem<'b, I>>,
{
    type Item<'a> = LendedItem<'a, I>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.a.next().or_else(|| self.b.next())
    }
}

pub struct Zip<I, J>
where
    I: LendingIter,
    J: LendingIter,
{
    pub(crate) a: I,
    pub(crate) b: J,
}

#[gat]
impl<I, J> LendingIter for Zip<I, J>
where
    I: LendingIter,
    J: LendingIter,
{
    type Item<'a> = (LendedItem<'a, I>, LendedItem<'a, J>)
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.a.next().and_then(|ai| self.b.next().map(|bi| (ai, bi)))
    }
}

pub struct Map<I, F>
where
    I: LendingIter,
    F: for<'a> Mapper<'a, LendedItem<'a, I>>,
{
    pub(crate) iter: I,
    pub(crate) fun: F,
}

#[gat]
impl<I, F> LendingIter for Map<I, F>
where
    I: LendingIter,
    F: for<'b> Mapper<'b, LendedItem<'b, I>>,
{
    type Item<'a> = <F as Mapper<'a, LendedItem<'a, I>>>::Output
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|i| self.fun.call(i))
    }
}

pub struct Filter<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&LendedItem<'a, I>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) filter: F,
}

#[gat]
impl<I, F> LendingIter for Filter<I, F>
where
    I: LendingIter,
    F: for<'b> FnMut(&LendedItem<'b, I>) -> bool,
{
    type Item<'a> = LendedItem<'a, I>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().filter(&mut self.filter)
    }
}

pub struct FilterMap<I, F>
where
    I: LendingIter,
    F: for<'a> OptionMapper<'a, LendedItem<'a, I>>,
{
    pub(crate) iter: I,
    pub(crate) filter: F,
}

#[gat]
impl<I, F> LendingIter for FilterMap<I, F>
where
    I: LendingIter,
    F: for<'b> OptionMapper<'b, LendedItem<'b, I>>,
{
    type Item<'a> = <F as OptionMapper<'a, LendedItem<'a, I>>>::Output
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        loop {
            // SAFETY:
            //  The Item<'a> we are matching on is either dropped or returned by
            //  the end of this match block, so it never escapes its actual lifetime
            match unsafe { extend_lifetime::<HKT!(Option<LendedItem<'_, I>>)>(self.iter.next()) } {
                None => return None,
                Some(item) => match self.filter.call(item) {
                    None => continue,
                    some => return some,
                },
            }
        }
    }
}

pub struct Enumerate<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) count: usize,
}

#[gat]
impl<I: LendingIter> LendingIter for Enumerate<I> {
    type Item<'a> = (usize, LendedItem<'a, I>)
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next().map(|i| (self.count, i));
        self.count += 1;
        item
    }
}

pub struct SkipWhile<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&LendedItem<'a, I>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
    pub(crate) done: bool,
}

#[gat]
impl<I, F> LendingIter for SkipWhile<I, F>
where
    I: LendingIter,
    F: for<'b> FnMut(&LendedItem<'b, I>) -> bool,
{
    type Item<'a> = LendedItem<'a, I>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.done {
            return self.iter.next();
        }
        loop {
            // SAFETY:
            // There are never two mutable references to the item here. Either
            // we return it, or continue thus dropping it
            let item =
                unsafe { extend_lifetime::<HKT!(Option<LendedItem<'_, I>>)>(self.iter.next()) };
            if item.as_ref().map(&mut self.pred) != Some(true) {
                self.done = true;
                return item;
            }
        }
    }
}

pub struct TakeWhile<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&LendedItem<'a, I>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
}

#[gat]
impl<I, F> LendingIter for TakeWhile<I, F>
where
    I: LendingIter,
    F: for<'b> FnMut(&LendedItem<'b, I>) -> bool,
{
    type Item<'a> = LendedItem<'a, I>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next();
        match item.as_ref().map(&mut self.pred) {
            Some(true) => item,
            _ => None,
        }
    }
}

pub struct MapWhile<I, F>
where
    I: LendingIter,
    F: for<'a> OptionMapper<'a, LendedItem<'a, I>>,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
}

#[gat]
impl<I, F> LendingIter for MapWhile<I, F>
where
    I: LendingIter,
    F: for<'b> OptionMapper<'b, LendedItem<'b, I>>,
{
    type Item<'a> = <F as OptionMapper<'a, LendedItem<'a, I>>>::Output
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        self.iter.next().and_then(|i| self.pred.call(i))
    }
}

pub struct Skip<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) n: usize,
}

#[gat]
impl<I: LendingIter> LendingIter for Skip<I> {
    type Item<'a> = LendedItem<'a, I>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        while self.n > 0 {
            match self.iter.next() {
                Some(_) => self.n -= 1,
                None => return None,
            }
        }
        self.iter.next()
    }
}

pub struct Take<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) n: usize,
}

#[gat]
impl<I: LendingIter> LendingIter for Take<I> {
    type Item<'a> = LendedItem<'a, I>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.n {
            0 => None,
            _ => {
                self.n -= 1;
                self.iter.next()
            }
        }
    }
}

pub struct Scan<I, S, F>
where
    I: LendingIter,
    F: for<'a> Scanner<'a, S, LendedItem<'a, I>>,
{
    pub(crate) iter: I,
    pub(crate) state: S,
    pub(crate) scan: F,
}

#[gat]
impl<I, S, F> LendingIter for Scan<I, S, F>
where
    I: LendingIter,
    F: for<'b> Scanner<'b, S, LendedItem<'b, I>>,
{
    type Item<'a> = <F as Scanner<'a, S, LendedItem<'a, I>>>::Output
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        self.iter.next().and_then(|item| self.scan.call(&mut self.state, item))
    }
}

pub struct FlatMap<I, F, J>
where
    I: LendingIter,
    J: LendingIter,
    F: for<'b> FnMut(LendedItem<'b, I>) -> J,
{
    pub(crate) iter: I,
    pub(crate) fun: F,
    pub(crate) curr: Option<J>,
}

#[gat]
impl<I, F, J> LendingIter for FlatMap<I, F, J>
where
    I: LendingIter,
    J: LendingIter,
    F: for<'b> FnMut(LendedItem<'b, I>) -> J,
{
    type Item<'a> = LendedItem<'a, J>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY:
            // Item is either dropped or returned by the end of the loop,
            // so two concurrent mutable references are never created.
            let item = unsafe {
                extend_lifetime::<HKT!(Option<LendedItem<'_, J>>)>(
                    self.curr.as_mut().and_then(|it| it.next()),
                )
            };
            match item {
                Some(v) => return Some(v),
                None => match self.iter.next().map(&mut self.fun) {
                    Some(it) => self.curr = Some(it),
                    None => return None,
                },
            }
        }
    }
}

#[apply(Gat!)]
pub struct Flatten<I, J>
where
    J: LendingIter,
    I: for<'a> LendingIter<Item<'a> = J>,
{
    pub(crate) iter: I,
    pub(crate) curr: Option<J>,
}

#[gat]
impl<I, J> LendingIter for Flatten<I, J>
where
    J: LendingIter,
    I: for<'b> LendingIter<Item<'b> = J>,
{
    type Item<'a> = LendedItem<'a, J>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY:
            // Item is either dropped or returned by the end of the loop,
            // so two concurrent mutable references are never created.
            let item = unsafe {
                extend_lifetime::<HKT!(Option<LendedItem<'_, J>>)>(
                    self.curr.as_mut().and_then(|it| it.next()),
                )
            };
            match item {
                Some(v) => return Some(v),
                None => match self.iter.next() {
                    Some(it) => self.curr = Some(it),
                    None => return None,
                },
            }
        }
    }
}

pub struct Fuse<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) avail: bool,
}

#[gat]
impl<I: LendingIter> LendingIter for Fuse<I> {
    type Item<'a> = LendedItem<'a, I>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.avail.then(|| self.iter.next()).flatten() {
            Some(item) => Some(item),
            None => {
                self.avail = false;
                None
            }
        }
    }
}

pub struct Inspect<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&LendedItem<'a, I>),
{
    pub(crate) iter: I,
    pub(crate) fun: F,
}

#[gat]
impl<I, F> LendingIter for Inspect<I, F>
where
    I: LendingIter,
    F: for<'b> FnMut(&LendedItem<'b, I>),
{
    type Item<'a> = LendedItem<'a, I> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|i| {
            (self.fun)(&i);
            i
        })
    }
}
