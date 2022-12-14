use anyhow::{Context, Result};
use itertools::Either;
use std::array;
use std::iter::Rev;

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

    fn rev_if(self, condition: bool) -> Either<Rev<Self>, Self>
    where
        Self: Sized,
        Self: DoubleEndedIterator,
    {
        if condition {
            Either::Left(self.rev())
        } else {
            Either::Right(self)
        }
    }
}

impl<T: ?Sized> IteratorHelpers for T where T: Iterator {}
