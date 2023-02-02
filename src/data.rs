use crate::logic::fractal::Behavior;

pub struct Planet {
    // icon: Icon,
    name: String,
    desc: String,
    fragments: Vec<Fragment>,
    biomes: Vec<Biome>,
}

pub struct Biome {
    // icon: Icon,
    name: String,
    desc: String,
    fragments: BitVec,
}

pub struct Fragment {
    // icon: Icon,
    name: String,
    desc: String,
    behavior: Behavior,
}
