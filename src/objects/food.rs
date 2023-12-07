//! Food for Brogue Seed Scanner.

/// Describes a Food item in Brogue.
#[derive(Clone, Debug)]
pub struct Food {
    kind: FoodKind,
}

impl Food {
    pub fn new(kind: FoodKind) -> Self { 
        Self { kind } 
    }
}

impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A {}", self.kind)
    }
}

/// Kinds for the Food Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum FoodKind {
    Mango,
    RationOfFood, 
}

impl FoodKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in FOOD_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in FOOD_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }  
}

impl std::fmt::Display for FoodKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            FoodKind::RationOfFood => "ration of food",
            FoodKind::Mango => "mango",
        };
        write!(f, "{}", result)
    }
}

const FOOD_KINDS: [(&str, FoodKind); 2] = [
    ("mango", FoodKind::Mango),
    ("ration of food", FoodKind::RationOfFood),
];
