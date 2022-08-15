use std::{
    array,
    ops::{Deref, IndexMut},
};

pub enum Node<T, B> {
    Leaf(T),
    Branch(B),
}
impl<T, I, B: IndexMut<I, Output = T>> Node<T, B> {
    pub fn expand<'a>(&mut self, expander: &impl Expander<From = &'a T, To = B>) -> &mut B {
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

    // pub fn expand_get_mut(
    //     &mut self,
    //     expander: &impl Expander<T, N>,
    //     index: impl Into<usize>,
    // ) -> &mut Self {
    //     &mut self.expand(expander)[index.into()]
    // }

    // pub fn deep_expand_get_mut<Iter>(
    //     &mut self,
    //     expander: &impl Expander<T, N>,
    //     indices: Iter,
    // ) -> &mut Self
    // where
    //     Iter: IntoIterator,
    //     Iter::Item: Into<usize>,
    // {
    //     indices
    //         .into_iter()
    //         .fold(self, |node, index| node.expand_get_mut(expander, index))
    // }
}

pub trait Expander {
    type From;
    type To;
    fn expand(&self, item: Self::From) -> Self::To;
}
// impl<T, const N: usize, E: Expander<T, N>> Expander<Option<T>, N> for E {
//     fn expand(&self, item: &Option<T>) -> [Option<T>; N] {
//         match item {
//             Some(inner) => self.expand(inner).map(Some),
//             None => array::from_fn(|_| None),
//         }
//     }
// }
