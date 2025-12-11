use std::{collections::VecDeque, fmt::{Debug, Display}};

use super::CellBounds;
use crate::{Mat, MatioError, MayBeFrom, MayBeInto};

pub struct LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    pub(super) item: T,
}

impl<T: Debug> Debug for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LastCell")
            .field("item", &self.item)
            .finish()
    }
}
impl<T: Display> Display for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    LastCell<T>: CellBounds,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{[{}]}}", self.item)
    }
}

impl<'a, T> TryFrom<LastCell<T>> for VecDeque<Mat<'a>>
where
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
{
    type Error = MatioError;

    fn try_from(last_cell: LastCell<T>) -> std::result::Result<Self, Self::Error> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), last_cell.item);
        mat.map(|mat| vec![mat].into())
    }
}
