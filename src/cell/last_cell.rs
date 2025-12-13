use std::fmt::{Debug, Display};

use super::CellBounds;
use crate::{Mat, MayBeFrom, MayBeInto};

pub struct LastCell<T> {
    pub(super) item: T,
}

impl<T: Debug> Debug for LastCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LastCell")
            .field(&format!("item #{}", Self::INDEX), &self.item)
            .finish()
    }
}
impl<T: Display> Display for LastCell<T>
where
    LastCell<T>: CellBounds,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{[{}]}}", self.item)
    }
}

impl<T> LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    pub fn i(&self) -> &T {
        &self.item
    }
}
