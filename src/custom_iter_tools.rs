use crate::custom_iter_tools::adaptors::zip_latest::{ZipLatest, zip_latest};

mod adaptors;
mod methods;

pub trait CustomIterTools: Iterator {
    #[inline]
    fn zip_latest<Item, J, JItem>(self, other: J) -> ZipLatest<Self, J::IntoIter>
    where
        Self: Sized,
        Self: Iterator<Item = Item>,
        Item: Copy,
        J: IntoIterator<Item = JItem>,
        JItem: Copy,
    {
        zip_latest(self, other.into_iter())
    }
}

impl<T> CustomIterTools for T where T: Iterator + ?Sized {}
