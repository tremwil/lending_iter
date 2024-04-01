use nougat::{apply, gat, Gat};

#[gat(Item)]
use crate::lending_iter::LendingIter;

pub trait LendingIterNext<'a, S: 'a> {
    type Output;

    fn next<'b>(&'b mut self, state: &'a mut S) -> Option<Self::Output>;
}

impl<'a, F, S: 'a, R> LendingIterNext<'a, S> for F
where
    F: FnMut(&'a mut S) -> Option<R>,
{
    type Output = R;

    fn next<'b>(&'b mut self, state: &'a mut S) -> Option<Self::Output> {
        (self)(state)
    }
}

#[apply(Gat)]
pub struct FromFn<S, F>
where
    F: for<'b> LendingIterNext<'b, S>,
{
    state: S,
    fun: F,
}

#[gat]
impl<S, F> LendingIter for FromFn<S, F>
where
    F: for<'b> LendingIterNext<'b, S>,
{
    type Item<'a> = <F as LendingIterNext<'a, S>>::Output
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.fun.next(&mut self.state)
    }
}

pub fn from_fn<S, F>(state: S, fun: F) -> FromFn<S, F>
where
    F: for<'b> LendingIterNext<'b, S>,
{
    FromFn { state, fun }
}
