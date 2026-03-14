pub trait Ends<T> {
    fn split_ends(&self) -> Option<(&T, &[T], &T)>;

    fn map_ends<U>(&self, f_head: impl FnMut(&T) -> U, f_mid: impl FnMut(&T) -> U, f_last: impl FnMut(&T) -> U) -> Option<impl Iterator<Item = U>> {
        let (head, mid, last) = self.split_ends()?;

        let mut vec: Vec::<U> = vec![];

        vec.push(f_head(head));

        for mid_elem in mid {
            vec.push(f_mid(mid_elem));
        }

        vec.push(f_last(last));

        Some(vec.into_iter())
    }

}

impl<T> Ends<T> for &[T] {
    fn split_ends(&self) -> Option<(&T, &[T], &T)> {
       match self {
           [head, mid@.., last] => Some((head, mid, last)),
           _ => None
       }
    }
}

