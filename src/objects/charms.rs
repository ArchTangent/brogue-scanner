//! Charms for Brogue Seed Scanner.

/// Describes a Brogue Charm.
#[derive(Clone, Debug)]
pub struct Charm {
    kind: CharmKind,
    enchantment: i8,       // Not an Option as all charms have an enchantment
}

impl Charm {
    pub fn new(kind: CharmKind, enchantment: i8) -> Self {
        Self { kind, enchantment } 
    }
}

impl std::fmt::Display for Charm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self.enchantment >= 0 {
            true => "+",
            false => ""
        }; 
        write!(f, "A {}{} {} charm", sign, self.enchantment, self.kind)
    }
}

/// Kinds for the Charm Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum CharmKind {
    FireImmunity,   
    Guardian,
    Haste,
    Health,
    Invisibility,   
    Levitation,     
    Negation,       
    Protection,     
    Recharging,     
    Shattering,     
    Telepathy,      
    Teleportation,  
}

impl std::fmt::Display for CharmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            CharmKind::FireImmunity => "fire immunity",
            CharmKind::Guardian => "guardian",
            CharmKind::Haste => "haste",
            CharmKind::Health => "health",
            CharmKind::Invisibility => "invisibility",
            CharmKind::Levitation => "levitation",
            CharmKind::Negation => "negation",
            CharmKind::Protection => "protection",
            CharmKind::Recharging => "recharging",
            CharmKind::Shattering => "shattering",
            CharmKind::Telepathy => "telepathy",
            CharmKind::Teleportation => "teleportation",
        };
        write!(f, "{}", result)
    }
}

impl CharmKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in CHARM_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in CHARM_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }     
}

const CHARM_KINDS: [(&str, CharmKind); 12] = [
    ("fire immunity", CharmKind::FireImmunity),
    ("guardian", CharmKind::Guardian),
    ("haste", CharmKind::Haste),
    ("health", CharmKind::Health),
    ("invisibility", CharmKind::Invisibility),
    ("levitation", CharmKind::Levitation),
    ("negation", CharmKind::Negation),
    ("protection", CharmKind::Protection),
    ("recharging", CharmKind::Recharging),
    ("shattering", CharmKind::Shattering),
    ("telepathy", CharmKind::Telepathy),
    ("teleportation", CharmKind::Teleportation),
];