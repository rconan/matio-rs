use crate::{
    Mat, Result,
    cell::{Cell, CellBounds, LastCell},
};

use super::MayBeFrom;

impl<'a, T1> MayBeFrom<(T1,)> for Mat<'a>
where
    Mat<'a>: MayBeFrom<LastCell<T1>>,
{
    fn maybe_from<S: Into<String>>(name: S, (item1,): (T1,)) -> Result<Self>
    where
        Self: Sized,
    {
        let c = Cell::new(item1);
        MayBeFrom::maybe_from(name, c)
    }
}

impl<'a, T1, T2> MayBeFrom<(T1, T2)> for Mat<'a>
where
    Mat<'a>: MayBeFrom<Cell<T1, LastCell<T2>>>,
{
    fn maybe_from<S: Into<String>>(name: S, (item1, item2): (T1, T2)) -> Result<Self>
    where
        Self: Sized,
    {
        let c = Cell::new(item1).push(item2);
        MayBeFrom::maybe_from(name, c)
    }
}

impl<'a, T1, T2, T3> MayBeFrom<(T1, T2, T3)> for Mat<'a>
where
    Mat<'a>: MayBeFrom<Cell<T1, Cell<T2, LastCell<T3>>>>,
{
    fn maybe_from<S: Into<String>>(name: S, (item1, item2, item3): (T1, T2, T3)) -> Result<Self>
    where
        Self: Sized,
    {
        let c = Cell::new(item1).push(item2).push(item3);
        MayBeFrom::maybe_from(name, c)
    }
}

impl<'a, T1, T2, T3, T4> MayBeFrom<(T1, T2, T3, T4)> for Mat<'a>
where
    Mat<'a>: MayBeFrom<Cell<T1, Cell<T2, Cell<T3, LastCell<T4>>>>>,
{
    fn maybe_from<S: Into<String>>(
        name: S,
        (item1, item2, item3, item4): (T1, T2, T3, T4),
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let c = Cell::new(item1).push(item2).push(item3).push(item4);
        MayBeFrom::maybe_from(name, c)
    }
}

impl<'a, T1, T2, T3, T4, T5> MayBeFrom<(T1, T2, T3, T4, T5)> for Mat<'a>
where
    Mat<'a>: MayBeFrom<Cell<T1, Cell<T2, Cell<T3, Cell<T4, LastCell<T5>>>>>>,
{
    fn maybe_from<S: Into<String>>(
        name: S,
        (item1, item2, item3, item4, item5): (T1, T2, T3, T4, T5),
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let c = Cell::new(item1)
            .push(item2)
            .push(item3)
            .push(item4)
            .push(item5);
        MayBeFrom::maybe_from(name, c)
    }
}
