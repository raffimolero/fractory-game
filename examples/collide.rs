#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Moves {
    motions: Vec<Move>,
}

impl Moves {
    fn fix(&mut self) {}

    fn to_actions(self) -> Vec<Option<Item>> {
        
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Things {
    data: Vec<Item>,
}

impl Things {
    fn do_moves(&mut self, mut moves: Moves) {
        moves.fix();
        moves.to_actions();
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
