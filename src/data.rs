use crate::logic::fractal::Behavior;

// HACK
type BitVec = Vec<bool>;

pub struct FragmentData {
    names: Vec<String>,
    behaviors: Vec<Behavior>,
}

pub struct Planet {
    // icon: Icon,
    name: String,
    desc: String,
    fragments: FragmentData,
    biomes: Vec<Biome>,
}

impl Planet {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        let xyyy = [
            ("", vec![]),
            ("X", vec![]),
            ("Y", vec![]),
            ("Flip-Flop", flip_self_and_below_self()),
            ("Spinner", hexagon()),
            ("Rotor", rotate()),
            ("Grower", grow()),
            ("Sucker", suck()),
            ("Wire", wire()),
        ];
        let frag_count = xyyy.len();
        let mut names = Vec::with_capacity(frag_count);
        let mut behaviors = Vec::with_capacity(frag_count);
        for (name, behavior) in xyyy {
            names.push(name.to_string());
            behaviors.push(behavior);
        }
        Self {
            name: "XYYY".into(),
            desc: "The first planet.".into(),
            fragments: FragmentData { names, behaviors },
            biomes: vec![Biome {
                name: "Landing Zone",
                desc: "Contains every fragment.",
                fragments: vec![true; frag_count],
            }],
        }
    }
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
