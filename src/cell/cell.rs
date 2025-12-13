use std::fmt::{Debug, Display};

use super::{CellBounds, LastCell};

pub struct Cell<T, C>
where
    C: CellBounds,
{
    pub(super) item: T,
    pub(super) next_cell: C,
}

impl<T: Debug, C> Debug for Cell<T, C>
where
    C: CellBounds + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field(&format!("item #{}", Self::INDEX), &self.item)
            .field("next_cell", &self.next_cell)
            .finish()
    }
}

impl<T: Display, C> Display for Cell<T, C>
where
    C: CellBounds + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{[{}]}} ", self.item)?;
        self.next_cell.fmt(f)
    }
}

impl<T> Cell<T, LastCell<T>> {
    pub fn new(item: T) -> LastCell<T> {
        LastCell { item }
    }
}
