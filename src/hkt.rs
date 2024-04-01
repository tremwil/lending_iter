use core::marker::PhantomData;

use crate::fn_traits::Mapper;
pub use crate::HKT;

/// Trait representing higher kinded type (HKT) parametrized by a single lifetime.
/// Implemented by [`HKT!`] macro types, e.g.
/// ```rust
///  HKT!(for<'a> T<'a>)
/// ````
///
/// Although this should not be used directly, concrete implementors of [`HKT`] are precisely
/// ```rust
/// PhantomData<for<'a> fn(&'a ()) -> T<'a>>
/// ```
/// so that [`FnOnce<(&'a (),)>::Output`] yields the concrete type for lifetime `'a`.
pub trait HKT {
    type With<'a>
    where
        Self: 'a;
}
impl<T> HKT for PhantomData<T>
where
    T: for<'a> Mapper<'a, &'a ()>,
{
    type With<'a> = <T as Mapper<'a, &'a ()>>::Output
        where
            Self: 'a;
}
impl<T> HKT for &T {
    type With<'a> = &'a T where Self: 'a;
}

impl<T> HKT for &mut T {
    type With<'a> = &'a mut T
    where
        Self: 'a;
}

#[macro_export]
macro_rules! HKT {
    (for<$lt:lifetime> $t:ty) => {
        ::core::marker::PhantomData<for<$lt> fn(&$lt ()) -> $t>
    };
    ($t:ty) => {
        ::core::marker::PhantomData<fn(&()) -> $t>
    };
}

/// Wrapper around [`core::mem::transmute`] that may only be used to extend lifetimes.
/// Preferable to transmuting directly.
pub unsafe fn extend_lifetime<'a, 'b, T: HKT>(v: T::With<'b>) -> T::With<'a> {
    core::mem::transmute(v)
}
