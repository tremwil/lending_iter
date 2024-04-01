pub trait Mapper<'a, A: 'a> {
    type Output;

    fn call(&mut self, item: A) -> Self::Output;
}
impl<'a, A: 'a, R, F: FnMut(A) -> R> Mapper<'a, A> for F {
    type Output = R;

    fn call(&mut self, item: A) -> Self::Output {
        (self)(item)
    }
}

pub trait OptionMapper<'a, A: 'a> {
    type Output;

    fn call(&mut self, item: A) -> Option<Self::Output>;
}
impl<'a, A: 'a, R, F: FnMut(A) -> Option<R>> OptionMapper<'a, A> for F {
    type Output = R;

    fn call(&mut self, item: A) -> Option<Self::Output> {
        (self)(item)
    }
}

pub trait Scanner<'a, S: 'a, A: 'a> {
    type Output;

    fn call(&mut self, state: &'a mut S, item: A) -> Option<Self::Output>;
}
impl<'a, S: 'a, A: 'a, R, F: FnMut(&'a mut S, A) -> Option<R>> Scanner<'a, S, A> for F {
    type Output = R;

    fn call(&mut self, state: &'a mut S, item: A) -> Option<Self::Output> {
        (self)(state, item)
    }
}
