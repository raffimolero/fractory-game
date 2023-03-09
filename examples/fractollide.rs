use std::{num::NonZeroUsize, ops::Index};

use fractory_game::logic::{
    path::{SubTile, TilePos},
    tile::Quad,
};

enum Occupation {
    Free,
    Partial,
    Full,
}

#[derive(Debug, Clone, Copy, Default)]
struct Node<T> {
    children: Quad<Option<NonZeroUsize>>,
    data: T,
}

/// a quadtree where each node holds a value and the root is always 0
struct QuadTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> QuadTree<T> {
    pub fn get_idx(&self, index: TilePos) -> Option<usize> {
        let mut cur = 0; // root
        for subtile in index {
            cur = self.nodes[cur].children[subtile]?.get();
        }
        Some(cur)
    }

    fn get(&self, mut index: TilePos) -> Option<&Node<T>> {
        let idx = self.get_idx(index)?;
        Some(&self.nodes[idx])
    }

    fn get_mut(&mut self, mut index: TilePos) -> Option<&mut Node<T>> {
        let idx = self.get_idx(index)?;
        Some(&mut self.nodes[idx])
    }

    pub fn register(&mut self, mut index: TilePos) -> usize
    where
        T: Default,
    {
        // let mut cur = 0; // root
        // for subtile in index {
        //     let idx = &mut cur.children[subtile];
        //     let idx = idx.unwrap_or_else(|| {
        //         *idx = Some(self.nodes.len());
        //         self.nodes.push(Node::default())
        //     });
        //     cur = idx.get();
        // }
        // cur
    }

    pub fn register_tmp0(&mut self, mut index: TilePos) -> usize
    where
        T: Default,
    {
        let mut cur = 0; // root
        for subtile in index {
            let idx = &mut cur.children[subtile];
            let idx = idx.unwrap_or_else(|| {
                *idx = Some(self.nodes.len());
                self.nodes.push(Node::default())
            });
            cur = idx.get();
        }
        cur
    }

    pub fn register_tmp1(&mut self, mut index: TilePos) -> usize
    where
        T: Default,
    {
        if self.nodes.is_empty() {
            self.nodes.push(Node::default());
        }
        let mut cur = self.nodes[0]; // root

        for subtile in index {
            let idx = &mut cur.children[subtile];
            let idx = idx.get_or_insert_with(|| {
                self.nodes.push(Node::default());
                (self.nodes.len() - 1).try_into().unwrap()
            });
            cur = self.nodes[idx.get()];
        }
        cur
    }
}

fn main() {
    println!("Hello, World!");
}
