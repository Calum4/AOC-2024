use std::cmp;
use std::iter::{Fuse, FusedIterator};

pub struct ZipLatest<T, U>
where
    T: Iterator,
    U: Iterator,
{
    a: Fuse<T>,
    a_latest: Option<T::Item>,
    b: Fuse<U>,
    b_latest: Option<U::Item>,
}

pub fn zip_latest<T, TItem, U, UItem>(a: T, b: U) -> ZipLatest<T, U>
where
    T: Iterator<Item = TItem>,
    TItem: Copy,
    U: Iterator<Item = UItem>,
    UItem: Copy,
{
    ZipLatest {
        a: a.fuse(),
        a_latest: None,
        b: b.fuse(),
        b_latest: None,
    }
}

impl<T, TItem, U, UItem> Iterator for ZipLatest<T, U>
where
    T: Iterator<Item = TItem>,
    TItem: Copy,
    U: Iterator<Item = UItem>,
    UItem: Copy,
{
    type Item = (T::Item, U::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (Some(a), None) => {
                if let Some(b_latest) = &self.b_latest {
                    self.a_latest = Some(a);

                    Some((a, *b_latest))
                } else {
                    None
                }
            }
            (None, Some(b)) => {
                if let Some(a_latest) = &self.a_latest {
                    self.b_latest = Some(b);

                    Some((*a_latest, b))
                } else {
                    None
                }
            }
            (Some(a), Some(b)) => {
                self.a_latest = Some(a);
                self.b_latest = Some(b);

                Some((a, b))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = cmp::max(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => Some(cmp::max(x, y)),
            _ => None,
        };

        (lower, upper)
    }
}

impl<T, TItem, U, UItem> ExactSizeIterator for ZipLatest<T, U>
where
    T: ExactSizeIterator<Item = TItem>,
    TItem: Copy,
    U: ExactSizeIterator<Item = UItem>,
    UItem: Copy,
{
}

impl<T, TItem, U, UItem> FusedIterator for ZipLatest<T, U>
where
    T: FusedIterator<Item = TItem>,
    TItem: Copy,
    U: FusedIterator<Item = UItem>,
    UItem: Copy,
{
}
