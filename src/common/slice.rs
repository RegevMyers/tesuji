pub trait SplitEnds<T> {
    fn split_ends(&self) -> Option<(&T, &[T], &T)>;
}

impl<T> SplitEnds<T> for &[T] {
    fn split_ends(&self) -> Option<(&T, &[T], &T)> {
        let (head, tail) = self.split_first()?;
        let (last, mid) = tail.split_last()?;
        Some((head, mid, last))
    }
}
