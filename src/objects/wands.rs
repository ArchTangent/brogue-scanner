//! Wands for Brogue Seed Scanner.

/// Describes a Brogue Wand.
#[derive(Clone, Debug)]
pub struct Wand {
    kind: WandKind,
    enchantment: i8,   // Not an Option as all wands have an enchantment
}

impl Wand {
    pub fn new(kind: WandKind, enchantment: i8) -> Self { 
        Self { kind, enchantment } 
    }
}

impl std::fmt::Display for Wand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A wand of {} [{}]", self.kind, self.enchantment)
    }
}

/// Kinds for the Wand Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum WandKind {
    Beckoning,
    Domination,
    Empowerment,
    Invisibility,
    Negation,
    Plenty,
    Polymorphism,
    Slowness,
    Teleportation,
}

impl WandKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in WAND_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in WAND_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }   
    /// Returns `true` if the wand is malevolent.
    pub fn is_malevolent(&self) -> bool {
        use WandKind::*;

        match self {
            Empowerment => true,
            Invisibility => true,
            Plenty => true,
            _ => false,
        }
    }    
}

impl std::fmt::Display for WandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WandKind::*;

        let result = match self {
            Beckoning => "beckoning",
            Domination => "domination",
            Empowerment => "empowerment",
            Invisibility => "invisibility",
            Negation => "negation",
            Plenty => "plenty",
            Polymorphism => "polymorphism",
            Slowness => "slowness",
            Teleportation => "teleportation",
        };
        write!(f, "{}", result)
    }
}

const WAND_KINDS: [(&str, WandKind); 9] = [
    ("beckoning", WandKind::Beckoning),
    ("domination", WandKind::Domination),
    ("empowerment", WandKind::Empowerment),
    ("invisibility", WandKind::Invisibility),
    ("negation", WandKind::Negation),
    ("plenty", WandKind::Plenty),
    ("polymorphism", WandKind::Polymorphism),
    ("slowness", WandKind::Slowness),
    ("teleportation", WandKind::Teleportation),
];
