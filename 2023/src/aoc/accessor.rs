use std::convert::TryInto;

pub trait At< T > {
    fn at< I >(&self, index: I) -> &T
    where
        I: TryInto< isize > + TryInto< usize > + Copy;
}

impl< T > At< T > for Vec< T > {
    fn at< I >(&self, index: I) -> &T
    where
        I: TryInto< isize > + TryInto< usize > + Copy
    {
        if (TryInto::< isize >::try_into(index).unwrap_or(0) < 0)
            || (TryInto::< usize >::try_into(index).unwrap_or(self.len()) >= self.len())
        {
            panic!("Out of bounds!");
        }
        return &self[TryInto::< usize >::try_into(index).unwrap_or(0)];
    }
}
