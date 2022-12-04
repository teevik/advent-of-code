use std::array;

use anyhow::{Result, Context};

pub trait IteratorHelpers: Iterator {
    fn collect_array<const N: usize>(mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        array::try_from_fn(|_| self.next())
    }

    fn collect_array_ok<const N: usize, A>(mut self) -> Result<[A; N]>
    where
        Self: Iterator<Item = Result<A>>,
        Self: Sized,
    {
        array::try_from_fn(|_| self.next().context("Not enough elements").flatten())
    }
}

impl<T: ?Sized> IteratorHelpers for T where T: Iterator {}
