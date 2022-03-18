use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt::Debug;

/// Partial variant of `std::iter::FromIterator`.
///
/// This trait is implemented for types which can be constructed from an iterator of decoded SSZ
/// values, but which may refuse values once a length limit is reached.
pub trait TryFromIter<T>: Sized {
    type Error: Debug;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>;
}

// It would be nice to be able to do a blanket impl, e.g.
//
// `impl TryFromIter<T> for C where C: FromIterator<T>`
//
// However this runs into trait coherence issues due to the type parameter `T` on `TryFromIter`.
//
// E.g. If we added an impl downstream for `List<T, N>` then another crate downstream of that
// could legally add an impl of `FromIterator<Local> for List<Local, N>` which would create
// two conflicting implementations for `List<Local, N>`. Hence the `List<T, N>` impl is disallowed
// by the compiler in the presence of the blanket impl. That's obviously annoying, so we opt to
// abandon the blanket impl in favour of impls for selected types.
impl<T> TryFromIter<T> for Vec<T> {
    type Error = Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        Ok(Self::from_iter(iter))
    }
}

impl<T, const N: usize> TryFromIter<T> for SmallVec<[T; N]> {
    type Error = Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        Ok(Self::from_iter(iter))
    }
}

impl<K, V> TryFromIter<(K, V)> for BTreeMap<K, V>
where
    K: Ord,
{
    type Error = Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Ok(Self::from_iter(iter))
    }
}

/// Partial variant of `collect`.
pub trait TryCollect: Iterator {
    fn try_collect<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIter<Self::Item>;
}

impl<I> TryCollect for I
where
    I: Iterator,
{
    fn try_collect<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIter<Self::Item>,
    {
        C::try_from_iter(self)
    }
}
