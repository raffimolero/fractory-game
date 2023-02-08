use std::{iter::repeat, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(usize);

impl Item {
    fn is_empty(self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Idx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    from: Idx,
    to: Idx,
}

#[derive(Debug, Clone)]
struct Moves {
    motions: Vec<Move>,
}

impl Moves {
    fn fix(&mut self, things: &Things) {
        #[derive(Debug, Clone, Copy)]
        enum FromStatus {
            Free,
            Taken { motion_idx: usize, to: Idx },
            Conflicted,
        }

        let mut from_trackers = vec![FromStatus::Free; things.data.len()];
        let mut to_trackers = vec![vec![]; things.data.len()];

        let mut i = 0;
        while i < self.motions.len() {
            let Move { from, to } = self.motions[i];
            if things[from].is_empty() {
                self.motions.swap_remove(i);
                continue;
            }

            let from_tracker = &mut from_trackers[from.0];
            match from_tracker {
                FromStatus::Free => *from_tracker = FromStatus::Taken(to),
                FromStatus::Taken {
                    motion_idx,
                    to: arrow_to,
                } => {
                    from_trackers[from.0] = FromStatus::Conflicted;
                    // TODO: figure out how to delete previous "from" item without messing up existing indices
                    self.motions.swap(i, motion_idx);
                    self.motions.swap_remove(i);
                    continue;
                }
                FromStatus::Conflicted => {
                    self.motions.swap_remove(i);
                    continue;
                }
            }

            to_ct[to.0] += 1;

            i += 1;
        }
    }
}

struct Actions {
    actions: Vec<(Idx, Item)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Things {
    data: Vec<Item>,
}

impl Things {
    fn do_moves(&mut self, mut moves: Moves) {
        moves.fix(self);
        moves.to_actions();
    }
}

impl Index<Idx> for Things {
    type Output = Item;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index.0]
    }
}

fn main() {
    let mut items = vec![Item(10), Item(20), Item(30), Item(40), Item(50)];

    let motions = [(0, 2)].map(|(from, to)| Move { from, to });

    println!("items");
    for item in &items {
        println!("{item:?}");
    }
    println!();

    println!("motions");
    for motion in motions {
        println!("{motion:?}");
    }
    println!();

    println!("items");
    for item in &items {
        println!("{item:?}");
    }
    println!();
}
