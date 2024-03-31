use crate::{hkt::OptionType, lending_iter::LendingIter};

pub struct FromFn<S, F> {
    state: S,
    fun: F,
}

impl<S, F> LendingIter for FromFn<S, F>
where
    for<'a> F: 'a + FnMut<(&'a mut S,)>,
    for<'a> <F as FnOnce<(&'a mut S,)>>::Output: OptionType,
{
    type Item<'a> = <<F as FnOnce<(&'a mut S,)>>::Output as OptionType>::UnwrapT
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        (self.fun)(&mut self.state).make_concrete()
    }
}

pub fn from_fn<S, F>(state: S, fun: F) -> FromFn<S, F>
where
    for<'a> F: 'a + FnMut<(&'a mut S,)>,
    for<'a> <F as FnOnce<(&'a mut S,)>>::Output: OptionType,
{
    FromFn { state, fun }
}
