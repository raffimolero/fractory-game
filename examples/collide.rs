use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(usize);

impl Item {
    fn is_hole(self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Idx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    src: Idx,
    dst: Idx,
}

#[derive(Debug, Clone)]
struct Moves(Vec<Move>);

#[derive(Debug, Clone)]
struct VerifiedMoves(Moves);

impl Moves {
    fn clean(mut self, things: &Things) -> VerifiedMoves {
        // first solve empties and dupes and forks,
        // then solve merges,
        // then solve dead ends and backtrack.
        self.clean_sources(things);
        self.clean_merges(things.0.len());
        self.clean_dead_ends(things);
        VerifiedMoves(self)
    }

    fn clean_sources(&mut self, things: &Things) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Status {
            Free,         // the slot is free
            Taken(usize), // the slot is taken by a move at this index
            Bad,          // the slot is taken by multiple moves
        }
        use Status::*;

        // tells the status of each slot
        let mut slots = vec![Free; things.0.len()];

        // the index of the move being checked at the moment
        let mut cur = 0;
        while cur < self.0.len() {
            let Move { src, dst } = self.0[cur];

            // check if source is empty
            if things[src].is_hole() {
                self.0.swap_remove(cur);
                continue;
            }

            let slot_state = &mut slots[src.0];
            match *slot_state {
                Status::Free => *slot_state = Status::Taken(cur),
                // check if they're actually the same
                Status::Taken(old) if self.0[old].dst != dst => {
                    *slot_state = Status::Bad;
                    self.0.swap_remove(cur);
                    cur -= 1;

                    let Move { src, dst: _ } = self.0[cur];

                    assert_eq!(
                        slots[src.0],
                        if old == cur {
                            Status::Bad
                        } else {
                            Status::Taken(cur)
                        }
                    );

                    slots[src.0] = Status::Taken(old);

                    self.0.swap(old, cur);
                    self.0.swap_remove(cur);
                    continue;
                }
                _ => {
                    self.0.swap_remove(cur);
                    continue;
                }
            }

            cur += 1;
        }
    }

    fn clean_merges(&mut self, thing_count: usize) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Status {
            Free,         // the slot is free
            Taken(usize), // the slot is taken by a move at this index
            Bad,          // the slot is taken by multiple moves
        }
        use Status::*;

        // tells the status of each slot
        let mut slots = vec![Free; thing_count];

        // the index of the move being checked at the moment
        let mut cur = 0;
        while cur < self.0.len() {
            let Move { src: _, dst } = self.0[cur];

            let slot_state = &mut slots[dst.0];
            match *slot_state {
                Status::Free => *slot_state = Status::Taken(cur),
                Status::Taken(old) => {
                    *slot_state = Status::Bad;
                    self.0.swap_remove(cur);
                    cur -= 1;

                    let Move { src: _, dst } = self.0[cur];

                    assert_eq!(
                        slots[dst.0],
                        if old == cur {
                            Status::Bad
                        } else {
                            Status::Taken(cur)
                        }
                    );

                    slots[dst.0] = Status::Taken(old);

                    self.0.swap(old, cur);
                    self.0.swap_remove(cur);
                    continue;
                }
                Status::Bad => {
                    self.0.swap_remove(cur);
                    continue;
                }
            }

            cur += 1;
        }
    }

    fn clean_dead_ends(&mut self, things: &Things) {
        let mut slots = vec![None; things.0.len()];
        let mut src_set = HashSet::new();
        for (i, mv) in self.0.iter().enumerate() {
            slots[mv.dst.0] = Some(i);
            src_set.insert(mv.src.0);
        }

        let mut cur = 0;
        'a: while cur < self.0.len() {
            let Move { src: _, dst } = self.0[cur];
            if !src_set.contains(&dst.0) && !things[dst].is_hole() {
                let mut backtrack_idx = cur;

                loop {
                    self.0.swap_remove(backtrack_idx);
                    if backtrack_idx != self.0.len() {
                        let moved = self.0[backtrack_idx];
                        assert_eq!(slots[moved.dst.0], Some(self.0.len()));
                        slots[moved.dst.0] = Some(backtrack_idx);
                    }
                    if let Some(idx) = slots[self.0[backtrack_idx].src.0] {
                        backtrack_idx = idx;
                    } else {
                        continue 'a;
                    }
                }
            }
            cur += 1;
        }
    }
}

struct Actions {
    actions: Vec<(Idx, Item)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Things(Vec<Item>);

impl Things {
    fn do_moves(&mut self, moves: Moves) {
        moves.clean(self);
        todo!()
    }
}

impl Index<Idx> for Things {
    type Output = Item;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index.0]
    }
}

fn main() {
    let mut items = Things([1, 2, 3, 0, 5, 6, 7, 8, 0].map(Item).to_vec());

    let moves = [(0, 7), (7, 8), (5, 6), (6, 2), (1, 2), (2, 3), (3, 4)]
        .map(|(src, dst)| Move {
            src: Idx(src),
            dst: Idx(dst),
        })
        .to_vec();
    let mut moves = Moves(moves);

    println!("items");
    for item in &items.0 {
        println!("{item:?}");
    }
    println!();

    println!("moves");
    for mv in &moves.0 {
        println!("{mv:?}");
    }
    println!();

    let moves = moves.clean(&items);

    println!("moves");
    for mv in &moves.0 .0 {
        println!("{mv:?}");
    }
    println!();
}
