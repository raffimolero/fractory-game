use std::ops::Deref;

trait Expander<T, const N: usize> {
    // i would use associated types for this,
    // but "generic parameters (actually associated consts in this case) can't be used in const operations."
    fn expand(&self, item: &T) -> [T; N];
}
impl<T, const N: usize, E: Expander<T, N>, R: Deref<Target = E>> Expander<T, N> for R {
    fn expand(&self, item: &T) -> [T; N] {
        self.deref().expand(item)
    }
}

pub enum Node<T, const N: usize> {
    Leaf(T),
    Branch(Box<[Self; N]>),
}
impl<const N: usize, T> Node<T, N> {
    fn expand(&mut self, expander: impl Expander<T, N>) -> &mut [Self; N] {
        use Node::*;
        if let Leaf(item) = self {
            *self = Branch(Box::new(expander.expand(item).map(Leaf)));
        }

        if let Branch(children) = self {
            children
        } else {
            unreachable!()
        }
    }

    fn expand_get_mut(
        &mut self,
        expander: impl Expander<T, N>,
        index: impl Into<usize>,
    ) -> &mut Self {
        &mut self.expand(expander)[index.into()]
    }

    fn deep_expand_get_mut<Iter>(
        &mut self,
        expander: impl Expander<T, N>,
        indices: Iter,
    ) -> &mut Self
    where
        Iter: IntoIterator,
        Iter::Item: Into<usize>,
    {
        indices
            .into_iter()
            .fold(self, |node, index| node.expand_get_mut(&expander, index))
    }
}
