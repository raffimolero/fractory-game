use super::{orientation::Transform, path::TileOffset};

pub struct Move {
    from: TileOffset,
    to: TileOffset,
    transform: Transform,
}

pub enum Action {
    Move(Move),
    Activate(TileOffset),
}

enum Node<T> {
    Root(T),
    Quad([usize; 4]),
}

/*
TODO: figure out how ActionTree should work

required:
    - moving tiles from one place to another with a transformation
    - put all actions down into the table, maybe build dependencies alongside it
    - after all actions are registered, find collisions
        - 2 moves from the same occupied tile
        - 2 moves to the same tile
        - moving to an occupied tile
    - cascade collisions by backtracking dependencies, everything else should be able to move

*/
pub struct ActionTree {
    library: Vec<Node<Option<Action>>>,
    root: usize,
}

impl ActionTree {
    pub fn new() -> Self {
        Self {
            library: vec![],
            root: 0,
        }
    }
}
