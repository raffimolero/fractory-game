use super::{
    orientation::Transform,
    path::{TileOffset, TilePos},
    tile::Tile,
};

/// a complete action that can be done to the tree,
/// where T is a position that is either relative (TileOffset)
/// or absolute (TilePos)
pub struct TargetedAction<T> {
    target: T,
    act: TileAction<T>,
}

/// action to do at an exact node
pub enum TileAction<T> {
    /// moves this fragment to another tile
    Move(T, Transform),

    /// stores this fragment in the player's inventory
    Store,

    // /// places a fragment onto the fractal from the player's inventory
    // Place(FragmentId),
    /// activates this tile in the next tick
    Activate,
}

enum Node<T> {
    Leaf(T),
    Quad([usize; 4]),
}

/// is able to collect any number of absolute targeted actions,
/// resolve their dependencies,
/// remove contradictions,
/// and be converted into a batch.
pub struct ActionCollector {
    // note: each node can have an infinite and recursive number of dependents and dependencies.
    // library: Vec<Node<Option<???>>>,
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

/// a batch of actions that can be applied to a fractal to advance the game state,
/// and possibly to the UI to facilitate movement.
pub struct ActionBatch {
    library: Vec<Node<Option<TargetedAction<TilePos>>>>,
}

impl ActionBatch {
    pub fn new() -> Self {
        Self { library: vec![] }
    }

    /// makes a position
    fn make_source(&mut self, target: TilePos, destination: Option<TilePos>) {
        todo!("")
    }

    /// attempts to add an action to the batch.
    /// fails silently.
    fn add_action(&mut self, act: TargetedAction<TilePos>) {
        let TargetedAction { mut target, act } = act;
        for sub in target {}
    }
}

#[test]
fn test_thing() {
    assert_eq!(
        std::mem::size_of::<TargetedAction<TilePos>>(),
        std::mem::size_of::<Option<TargetedAction<TilePos>>>()
    );
}
