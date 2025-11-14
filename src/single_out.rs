pub struct DiscontinuousSpan<'a, T> {
    before: &'a mut [T],
    after: &'a mut [T],
}

pub fn single_out<T>(v: &mut [T], index: usize) -> Option<(&mut T, DiscontinuousSpan<'_, T>)> {
    let (before, rest) = v.split_at_mut_checked(index)?;
    let (single, after) = rest.split_first_mut()?;
    Some((single, DiscontinuousSpan { before, after }))
}

impl<'b, T> IntoIterator for &'b mut DiscontinuousSpan<'_, T> {
    type Item = &'b mut T;
    type IntoIter = std::iter::Chain<std::slice::IterMut<'b, T>, std::slice::IterMut<'b, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.before.iter_mut().chain(self.after.iter_mut())
    }
}

impl<'b, T> IntoIterator for &'b DiscontinuousSpan<'_, T> {
    type Item = &'b T;
    type IntoIter = std::iter::Chain<std::slice::Iter<'b, T>, std::slice::Iter<'b, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.before.iter().chain(self.after.iter())
    }
}
