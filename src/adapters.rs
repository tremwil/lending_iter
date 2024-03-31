use crate::hkt::{extend_lifetime, OptionType, ResultType, HKT};
use crate::lending_iter::LendingIter;

pub struct StepBy<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) step: usize,
    pub(crate) first: bool,
}

impl<I: LendingIter> LendingIter for StepBy<I> {
    type Item<'a> = I::Item<'a>
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

pub struct Chain<I, J>
where
    I: LendingIter,
    J: for<'a> LendingIter<Item<'a> = I::Item<'a>>,
{
    pub(crate) a: Fuse<I>,
    pub(crate) b: J,
}

impl<I, J> LendingIter for Chain<I, J>
where
    I: LendingIter,
    for<'a> J: 'a + LendingIter<Item<'a> = I::Item<'a>>,
{
    type Item<'a> = I::Item<'a>
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

impl<I, J> LendingIter for Zip<I, J>
where
    I: LendingIter,
    J: LendingIter,
{
    type Item<'a> = (I::Item<'a>, J::Item<'a>)
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.a.next().and_then(|ai| self.b.next().map(|bi| (ai, bi)))
    }
}

pub struct Map<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(I::Item<'a>,)>,
{
    pub(crate) iter: I,
    pub(crate) fun: F,
}

impl<I, F> LendingIter for Map<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(I::Item<'a>,)>,
{
    type Item<'a> = <F as FnOnce<(I::Item<'a>,)>>::Output
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(&mut self.fun)
    }
}

pub struct Filter<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) filter: F,
}

impl<I, F> LendingIter for Filter<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().filter(&mut self.filter)
    }
}

pub struct FilterMap<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(I::Item<'a>,)>,
    for<'a> <F as FnOnce<(I::Item<'a>,)>>::Output: OptionType,
{
    pub(crate) iter: I,
    pub(crate) filter: F,
}

impl<I, F> LendingIter for FilterMap<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(I::Item<'a>,)>,
    for<'a> <F as FnOnce<(I::Item<'a>,)>>::Output: OptionType,
{
    type Item<'a> = <<F as FnOnce<(I::Item<'a>,)>>::Output as OptionType>::UnwrapT
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        loop {
            // SAFETY:
            //  The Item<'a> we are matching on is either dropped or returned by
            //  the end of this match block, so it never escapes its actual lifetime
            match unsafe { extend_lifetime::<HKT!(Option<I::Item<'_>>)>(self.iter.next()) } {
                None => return None,
                Some(item) => match (self.filter)(item).make_concrete() {
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

impl<I: LendingIter> LendingIter for Enumerate<I> {
    type Item<'a> = (usize, I::Item<'a>)
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
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
    pub(crate) done: bool,
}

impl<I, F> LendingIter for SkipWhile<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a>
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
            let item = unsafe { extend_lifetime::<HKT!(Option<I::Item<'_>>)>(self.iter.next()) };
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
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
}

impl<I, F> LendingIter for TakeWhile<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a>
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
    F: for<'a> FnMut<(I::Item<'a>,)>,
    for<'a> <F as FnOnce<(I::Item<'a>,)>>::Output: OptionType,
{
    pub(crate) iter: I,
    pub(crate) pred: F,
}

impl<I, F> LendingIter for MapWhile<I, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(I::Item<'a>,)>,
    for<'a> <F as FnOnce<(I::Item<'a>,)>>::Output: OptionType,
{
    type Item<'a> = <<F as FnOnce<(I::Item<'a>,)>>::Output as OptionType>::UnwrapT
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        self.iter.next().and_then(|i| (self.pred)(i).make_concrete())
    }
}

pub struct Skip<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) n: usize,
}

impl<I: LendingIter> LendingIter for Skip<I> {
    type Item<'a> = I::Item<'a>
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

impl<I: LendingIter> LendingIter for Take<I> {
    type Item<'a> = I::Item<'a>
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
    F: for<'a> FnMut<(&'a mut S, I::Item<'a>)>,
    for<'a> <F as FnOnce<(&'a mut S, I::Item<'a>)>>::Output: OptionType,
{
    pub(crate) iter: I,
    pub(crate) state: S,
    pub(crate) scan: F,
}

impl<I, S, F> LendingIter for Scan<I, S, F>
where
    I: LendingIter,
    F: for<'a> FnMut<(&'a mut S, I::Item<'a>)>,
    for<'a> <F as FnOnce<(&'a mut S, I::Item<'a>)>>::Output: OptionType,
{
    type Item<'a> = <<F as FnOnce<(&'a mut S, I::Item<'a>,)>>::Output as OptionType>::UnwrapT
        where
            Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        self.iter.next().and_then(|item| (self.scan)(&mut self.state, item).make_concrete())
    }
}

pub struct FlatMap<I, J, F>
where
    I: LendingIter,
    J: LendingIter,
    F: for<'a> FnMut(I::Item<'a>) -> J,
{
    iter: I,
    fun: F,
    curr: Option<J>,
}

impl<I, J, F> LendingIter for FlatMap<I, J, F>
where
    I: LendingIter,
    J: LendingIter,
    F: for<'a> FnMut(I::Item<'a>) -> J,
{
    type Item<'a> = J::Item<'a>
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY:
            // polonius loop case
            let item = unsafe {
                extend_lifetime::<HKT!(Option<J::Item<'_>>)>(
                    self.curr.as_mut().and_then(|i| i.next()),
                )
            };
            match item {
                None => match self.iter.next().map(&mut self.fun) {
                    None => return None,
                    Some(it) => {
                        self.curr = Some(it);
                        continue;
                    }
                },
                some => return some,
            }
        }
    }
}

pub struct Fuse<I: LendingIter> {
    pub(crate) iter: I,
    pub(crate) avail: bool,
}

impl<I: LendingIter> LendingIter for Fuse<I> {
    type Item<'a> = I::Item<'a>
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
