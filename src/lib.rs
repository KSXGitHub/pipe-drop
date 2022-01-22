#![no_std]
#![doc = include_str!("../doc/lib.md")]

use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[doc = include_str!("../doc/lib.md")]
pub trait PipeDrop: Sized {
    /// Apply `f` on `&self` and then drop `self`.
    fn pipe_ref_drop<F, Y>(self, f: F) -> Y
    where
        F: FnOnce(&Self) -> Y,
    {
        f(&self)
    }

    /// Apply `f` on `&mut self` and then drop `self`.
    fn pipe_mut_drop<F, Y>(mut self, f: F) -> Y
    where
        F: FnOnce(&mut Self) -> Y,
    {
        f(&mut self)
    }

    /// Apply `f` on `self.as_ref()` and then drop `self`.
    fn pipe_as_ref_drop<F, X, Y>(self, f: F) -> Y
    where
        Self: AsRef<X>,
        X: ?Sized,
        F: FnOnce(&X) -> Y,
    {
        f(self.as_ref())
    }

    /// Apply `f` on `self.as_mut()` and then drop `self`.
    fn pipe_as_mut_drop<F, X, Y>(mut self, f: F) -> Y
    where
        Self: AsMut<X>,
        X: ?Sized,
        F: FnOnce(&mut X) -> Y,
    {
        f(self.as_mut())
    }

    /// Apply `f` on `&self` and then drop `self`.
    fn pipe_deref_drop<F, X, Y>(self, f: F) -> Y
    where
        Self: Deref<Target = X>,
        X: ?Sized,
        F: FnOnce(&X) -> Y,
    {
        f(&self)
    }

    /// Apply `f` on `&mut self` and then drop `self`.
    fn pipe_deref_mut_drop<F, X, Y>(mut self, f: F) -> Y
    where
        Self: DerefMut<Target = X>,
        X: ?Sized,
        F: FnOnce(&X) -> Y,
    {
        f(&mut self)
    }

    /// Apply `f` on `self.borrow()` and then drop `self`.
    fn pipe_borrow_drop<F, X, Y>(self, f: F) -> Y
    where
        Self: Borrow<X>,
        X: ?Sized,
        F: FnOnce(&X) -> Y,
    {
        f(self.borrow())
    }

    /// Apply `f` on `self.borrow_mut()` and then drop `self`.
    fn pipe_borrow_mut_drop<F, X, Y>(mut self, f: F) -> Y
    where
        Self: BorrowMut<X>,
        X: ?Sized,
        F: FnOnce(&X) -> Y,
    {
        f(self.borrow_mut())
    }
}

impl<X> PipeDrop for X {}
