use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Index, IndexMut},
};

use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(usize);

impl Item {
    fn is_hole(self) -> bool {
        self.0 == 0
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hole() {
            write!(f, ".")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data<T>(Vec<T>);

impl<T> Data<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Idx(usize);

impl<T> Index<Idx> for Data<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<T> IndexMut<Idx> for Data<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

#[derive(Clone)]
struct Items(Data<Item>);

impl Items {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<Idx> for Items {
    type Output = Item;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Idx> for Items {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    src: Idx,
    dst: Idx,
}

#[derive(Debug, Clone)]
struct Moves(Vec<Move>);

impl Moves {
    fn new<const N: usize>(stuff: [[usize; 2]; N]) -> Self {
        Self(
            stuff
                .map(|[src, dst]| Move {
                    src: Idx(src),
                    dst: Idx(dst),
                })
                .to_vec(),
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn clean(mut self, items: &Items) -> VerifiedMoves {
        // first solve empties and dupes and forks,
        // then solve merges,
        // then solve dead ends and backtrack.
        self.clean_sources(items);
        self.clean_merges(items.len());
        self.clean_dead_ends(items);
        VerifiedMoves(self)
    }

    fn clean_sources(&mut self, items: &Items) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Status {
            Free,         // the slot is free
            Taken(usize), // the slot is taken by a move at this index
            Bad,          // the slot is taken by multiple moves
        }
        use Status::*;

        let moves = &mut self.0;
        // tells the status of each slot
        let mut slots = Data(vec![Free; items.len()]);

        // the index of the move being checked at the moment
        let mut cur = 0;
        while cur < moves.len() {
            let Move { src, dst } = moves[cur];

            // check if source is empty
            if items[src].is_hole() {
                moves.swap_remove(cur);
                continue;
            }

            let slot_state = &mut slots[src];
            match *slot_state {
                Status::Free => *slot_state = Status::Taken(cur),
                //                 [  dedup the move list  ]
                Status::Taken(old) if moves[old].dst != dst => {
                    *slot_state = Status::Bad;
                    moves.swap_remove(cur);
                    cur -= 1;

                    let Move { src, dst: _ } = moves[cur];

                    if let Status::Taken(addr) = &mut slots[src] {
                        *addr = old;
                    }

                    moves.swap(old, cur);
                    moves.swap_remove(cur);
                    cur -= 1;
                }
                _ => {
                    moves.swap_remove(cur);
                    cur -= 1;
                }
            }

            cur += 1;
        }
    }

    fn clean_merges(&mut self, thing_count: usize) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Status {
            // the slot is free
            Free,
            // the slot is taken by a move at this index
            Taken(usize),
            // the slot is taken by multiple moves
            Bad,
        }
        use Status::*;

        let moves = &mut self.0;
        // tells the status of each slot
        let mut slots = Data(vec![Free; thing_count]);

        // the index of the move being checked at the moment
        let mut cur = 0;
        while let Some(mv) = moves.get(cur).copied() {
            let slot_state = &mut slots[mv.dst];
            match *slot_state {
                Status::Free => *slot_state = Status::Taken(cur),
                // dedup is already done by this point
                Status::Taken(old) /*if moves[old].src != src*/ => {
                    *slot_state = Status::Bad;
                    moves.swap_remove(cur);
                    cur -= 1;

                    // get last valid move
                    let mv = moves[cur];

                    // swap remove
                    moves[old] = mv;
                    if let Status::Taken(addr) = &mut slots[mv.dst] {
                        // retarget
                        *addr = old;
                    }

                    moves.swap_remove(cur);
                    cur -= 1;
                }
                Status::Bad => {
                    moves.swap_remove(cur);
                    cur -= 1;
                }
            }

            cur += 1;
        }
    }

    fn clean_dead_ends(&mut self, items: &Items) {
        let mut slots = vec![None; items.len()];
        let mut src_set = HashSet::new();
        for (i, mv) in self.0.iter().enumerate() {
            slots[mv.dst.0] = Some(i);
            src_set.insert(mv.src.0);
        }

        let mut cur = 0;
        'a: while cur < self.len() {
            let Move { src: _, dst } = self.0[cur];
            if !src_set.contains(&dst.0) && !items[dst].is_hole() {
                let mut backtrack_idx = cur;

                loop {
                    let popped = self.0.swap_remove(backtrack_idx);
                    if let Some(moved) = self.0.get(backtrack_idx) {
                        assert_eq!(slots[moved.dst.0], Some(self.len()));
                        slots[moved.dst.0] = Some(backtrack_idx);
                    }
                    match slots[popped.src.0] {
                        Some(idx) => backtrack_idx = idx,
                        None => continue 'a,
                    };
                }
            }
            cur += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct VerifiedMoves(Moves);

impl VerifiedMoves {
    fn to_actions(self, items: &Items) -> Actions {
        let mut slots = Data(vec![None; items.len()]);
        for Move { src, dst: _ } in &self.0 .0 {
            assert!(slots[*src].is_none());
            slots[*src] = Some(Item(0));
        }
        for Move { src, dst } in self.0 .0 {
            assert!(matches!(slots[dst], None | Some(Item(0))));
            slots[dst] = Some(items[src]);
        }
        Actions(slots)
    }
}

#[derive(Debug, Clone)]
struct Actions(Data<Option<Item>>);

impl Actions {
    fn perform(&self, items: &mut Items) {
        for (item, action) in items.0 .0.iter_mut().zip(self.0 .0.iter().copied()) {
            if let Some(new) = action {
                *item = new;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Range, panic::catch_unwind};

    use super::*;

    fn random_items(item_count: Range<usize>) -> Items {
        let mut rng = thread_rng();

        let item_count = rng.gen_range(item_count);
        let mut random_item = |_| {
            if rng.gen() {
                0
            } else {
                rng.gen_range(1..10)
            }
        };

        let items = (0..item_count)
            .map(random_item)
            .map(Item)
            .collect::<Vec<Item>>();

        Items(Data(items))
    }

    fn random_moves(item_count: usize, move_count: Range<usize>) -> Moves {
        let mut rng = thread_rng();

        let move_ct = rng.gen_range(move_count);
        let mut random_num = || rng.gen_range(0..item_count);
        let moves = (0..move_ct)
            .map(|_| Move {
                src: Idx(random_num()),
                dst: Idx(random_num()),
            })
            .collect::<Vec<Move>>();

        Moves(moves)
    }

    // #[test]
    // fn test_dead_end() {
    //     let items = Items::new([1, 1, 1]);
    //     let moves = Moves::new([[0, 1], [1, 2]]);
    //     moves.clean(&items);
    // }

    // #[test]
    fn fuzz_fails() {
        let fails = [
            (vec![1, 1, 1, 1], vec![[0, 0], [0, 1], [0, 2]]),
            (vec![1, 1, 1, 1], vec![[0, 3], [1, 0], [2, 0]]),
            (vec![1, 1, 1, 1], vec![[0, 0], [1, 0], [2, 0]]),
        ];

        for (i, (it, mv)) in fails.into_iter().enumerate() {
            println!("{i}");
            let items = Items(Data(it.into_iter().map(Item).collect()));
            let mut moves = Moves(
                mv.into_iter()
                    .map(|[src, dst]| Move {
                        src: Idx(src),
                        dst: Idx(dst),
                    })
                    .collect(),
            );
            let moves = moves.clean(&items);
            println!("{moves:#?}");
        }
    }

    #[test]
    fn fuzz() {
        let mut fails = vec![];
        for i in 0..1 << 10 {
            let items = random_items(2..16);
            let moves = random_moves(items.len(), 2..16);
            let saved_items = items
                .0
                 .0
                .iter()
                .map(|Item(x)| (*x != 0) as u8)
                .collect::<Vec<u8>>();
            let saved_moves = moves
                .clone()
                .0
                .iter()
                .map(|Move { src, dst }| [src.0, dst.0])
                .collect::<Vec<[usize; 2]>>();

            if catch_unwind(|| moves.clean(&items).to_actions(&items)).is_err() {
                fails.push((saved_items, saved_moves));
            }
        }
        println!("let fails = [");
        for (it, mv) in fails {
            println!("  (");
            println!("    vec!{it:?},");
            println!("    vec!{mv:?},");
            println!("  ),");
        }
        println!("];");
    }
}

struct ItemData(Vec<Vec<Move>>);

// TODO: make this interactive so you can place stuff on a linear array
// they must have behavior
// basically fractory but without the fractals and it's just on a line

fn main() {
    let mut items = Items(Data([1, 2, 3, 0, 5, 6, 7, 8, 0].map(Item).to_vec()));

    let mut moves = [(0, 7), (7, 8), (5, 6), (6, 2), (1, 2), (2, 3), (3, 4)]
        .map(|(src, dst)| Move {
            src: Idx(src),
            dst: Idx(dst),
        })
        .to_vec();
    moves.shuffle(&mut thread_rng());
    let mut moves = Moves(moves);

    println!("items");
    for item in &items.0 .0 {
        println!("{item}");
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

    let actions = moves.to_actions(&items);
    println!("actions");
    for act in &actions.0 .0 {
        println!("{act:?}");
    }
    println!();

    let orig = items.clone();
    actions.perform(&mut items);
    println!("items");
    for (i, (orig, item)) in orig.0 .0.into_iter().zip(items.0 .0).enumerate() {
        println!("{i}: {} {}", orig, item);
    }
    println!();
}
