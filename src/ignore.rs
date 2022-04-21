use std::convert::Infallible;
use std::marker::PhantomData;

use crate::de::{Map, Seq, Visitor, VisitorError};
use crate::error::Result;
use alloc::boxed::Box;

impl<E> dyn Visitor<Error = E> {
    pub fn ignore() -> &'static mut Ignore<E> {
        static mut IGNORE: Ignore<Infallible> = Ignore(PhantomData);
        unsafe { std::mem::transmute(&mut IGNORE) }
        //
        // The following may be needed if stacked borrows gets more selective
        // about the above in the future:
        //
        //     unsafe { &mut *ptr::addr_of_mut!(IGNORE) }
        //
        // Conceptually we have an array of type [Ignore; ∞] in a static, which
        // is zero sized, and each caller of `fn ignore` gets a unique one of
        // them, as if by `&mut *ptr::addr_of_mut!(IGNORE[i++])` for some
        // appropriately synchronized i.
    }
}

pub(crate) struct Ignore<E>(PhantomData<E>);

impl<E> Ignore<E> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl VisitorError for Infallible {
    fn unexpected() -> Self {
        unreachable!()
    }
}

impl<E: VisitorError + 'static> Visitor for Ignore<E> {
    type Error = E;
    fn raise(&mut self, _err: Self::Error) {}

    fn null(&mut self) {}

    fn boolean(&mut self, _b: bool) {}

    fn string(&mut self, _s: &str) {}

    fn negative(&mut self, _n: i64) {}

    fn nonnegative(&mut self, _n: u64) {}

    fn float(&mut self, _n: f64) {}

    fn seq(&mut self) -> Option<Box<dyn Seq<Self::Error> + '_>> {
        Some(Box::new(Ignore::new()))
    }

    fn map(&mut self) -> Option<Box<dyn Map<Self::Error> + '_>> {
        Some(Box::new(Ignore::new()))
    }
}

impl<E: VisitorError + 'static> Seq<E> for Ignore<E> {
    fn element(&mut self) -> Result<&mut dyn Visitor<Error = E>> {
        Ok(<dyn Visitor<Error = E>>::ignore())
    }

    fn finish(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<E: VisitorError + 'static> Map<E> for Ignore<E> {
    fn key(&mut self, _k: &str) -> Result<&mut dyn Visitor<Error = E>> {
        Ok(<dyn Visitor<Error = E>>::ignore())
    }

    fn finish(&mut self) -> Result<()> {
        Ok(())
    }
}
