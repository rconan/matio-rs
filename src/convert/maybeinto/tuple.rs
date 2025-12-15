use crate::{
    Mat, Result,
    cell::{Cell, LastCell},
};

use super::MayBeInto;

impl<'a, T1> MayBeInto<(T1,)> for Mat<'a>
where
    Mat<'a>: MayBeInto<T1>,
{
    fn maybe_into(self) -> Result<(T1,)> {
        let c = <Mat<'a> as MayBeInto<LastCell<T1>>>::maybe_into(self)?;
        let item = c.item();
        Ok((item,))
    }
}
impl<'a, T1, T2> MayBeInto<(T1, T2)> for Mat<'a>
where
    Mat<'a>: MayBeInto<T1> + MayBeInto<T2>,
{
    fn maybe_into(self) -> Result<(T1, T2)> {
        let c = <Mat<'a> as MayBeInto<Cell<T1, LastCell<T2>>>>::maybe_into(self)?;
        let (item1, c) = c.split();
        let item2 = c.item();
        Ok((item1, item2))
    }
}
impl<'a, T1, T2, T3> MayBeInto<(T1, T2, T3)> for Mat<'a>
where
    Mat<'a>: MayBeInto<T1> + MayBeInto<T2> + MayBeInto<T3>,
{
    fn maybe_into(self) -> Result<(T1, T2, T3)> {
        let c = <Mat<'a> as MayBeInto<Cell<T1, Cell<T2, LastCell<T3>>>>>::maybe_into(self)?;
        let (item1, c) = c.split();
        let (item2, c) = c.split();
        let item3 = c.item();
        Ok((item1, item2, item3))
    }
}
impl<'a, T1, T2, T3, T4> MayBeInto<(T1, T2, T3, T4)> for Mat<'a>
where
    Mat<'a>: MayBeInto<T1> + MayBeInto<T2> + MayBeInto<T3> + MayBeInto<T4>,
{
    fn maybe_into(self) -> Result<(T1, T2, T3, T4)> {
        let c =
            <Mat<'a> as MayBeInto<Cell<T1, Cell<T2, Cell<T3, LastCell<T4>>>>>>::maybe_into(self)?;
        let (item1, c) = c.split();
        let (item2, c) = c.split();
        let (item3, c) = c.split();
        let item4 = c.item();
        Ok((item1, item2, item3, item4))
    }
}
impl<'a, T1, T2, T3, T4, T5> MayBeInto<(T1, T2, T3, T4, T5)> for Mat<'a>
where
    Mat<'a>: MayBeInto<T1> + MayBeInto<T2> + MayBeInto<T3> + MayBeInto<T4> + MayBeInto<T5>,
{
    fn maybe_into(self) -> Result<(T1, T2, T3, T4, T5)> {
        let c =
            <Mat<'a> as MayBeInto<Cell<T1, Cell<T2, Cell<T3, Cell<T4,LastCell<T5>>>>>>>::maybe_into(self)?;
        let (item1, c) = c.split();
        let (item2, c) = c.split();
        let (item3, c) = c.split();
        let (item4, c) = c.split();
        let item5 = c.item();
        Ok((item1, item2, item3, item4, item5))
    }
}
