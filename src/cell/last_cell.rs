use std::fmt::{Debug, Display};

use super::CellBounds;

pub struct LastCell<T> {
    pub(super) item: T,
}

impl<T> LastCell<T> {
    pub fn item(self) -> T {
        self.item
    }
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

impl<T: PartialEq> PartialEq for LastCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: Clone> Clone for LastCell<T> {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
        }
    }
}
