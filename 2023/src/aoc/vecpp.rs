use std::convert::TryInto;

pub trait Vecpp< T > {
    fn at< I >(&self, index: I) -> &T
    where
        I: TryInto< isize > + TryInto< usize > + Copy;

    fn in_bounds< I >(&self, index: I) -> bool
    where
        I: TryInto< isize > + TryInto< usize > + Copy;

    fn find(&self, item: &T) -> Option< usize >
    where
        T: Eq;
}

impl< T > Vecpp< T > for Vec< T > {
    fn at< I >(&self, index: I) -> &T
    where
        I: TryInto< isize > + TryInto< usize > + Copy
    {
        if !(self.in_bounds(index))
        {
            panic!("Out of bounds!");
        }
        return &self[TryInto::< usize >::try_into(index).unwrap_or(0)];
    }

    fn in_bounds< I >(&self, index: I) -> bool
    where
        I: TryInto< isize > + TryInto< usize > + Copy
    {
        return (TryInto::< isize >::try_into(index).unwrap_or(-1) >= 0)
            && (TryInto::< usize >::try_into(index).unwrap_or(self.len()) < self.len())
    }

    fn find(&self, member: &T) -> Option< usize >
    where T: Eq
    {
        self.iter().position(|x| x == member)
    }
}
