use super::{
    orientation::Transform,
    path::{TileOffset, TilePos},
    tile::Tile,
};

/// behavior that a fragment can have
pub struct TileAct {
    from: TilePos,
    act: TreeAct,
}

/// action to do at an exact node
pub enum TreeAct {
    /// moves this fragment to another tile
    Move(TileOffset, Transform),

    /// stores this fragment in the player's inventory
    Store,

    /// activates this tile in the next tick
    Activate,
}

enum Node<T> {
    Root(T),
    Quad([usize; 4]),
}

/*
TODO: figure out how Actions should work

notes:
    - cache tilepos per node in fractal?
    - store activations in btreeset, sorted by depth and executed from deepest to top?

required:
    - moving tiles from one place to another with a transformation
    - put all actions down into the table, maybe build dependencies alongside it
    - after all actions are registered, find collisions
        - 2 moves from the same occupied tile
        - 2 moves to the same tile
        - moving to an occupied tile
    - cascade collisions by backtracking dependencies, everything else should be able to move

*/

pub struct Actions {
    library: Vec<Node<Option<TileAct>>>,
    root: usize,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            library: vec![],
            root: 0,
        }
    }
}
