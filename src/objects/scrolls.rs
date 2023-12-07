//! Scrolls for Brogue Seed Scanner.

/// Describes a Brogue Scroll.
#[derive(Clone, Debug)]
pub struct Scroll {
    kind: ScrollKind,
}

impl Scroll {
    pub fn new(kind: ScrollKind) -> Self { 
        Self { kind } 
    }
}

impl std::fmt::Display for Scroll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A scroll of {}", self.kind)
    }
}

/// Kinds for the Scroll Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ScrollKind {
    AggravateMonsters,
    Discord,        
    Enchanting,
    Identify,
    MagicMapping,  
    Negation,  
    ProtectArmor,
    ProtectWeapon,
    Recharging,
    RemoveCurse,
    Sanctuary,
    Shattering,
    SummonMonsters,
    Teleportation,
}

impl ScrollKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in SCROLL_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in SCROLL_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }  
    /// Returns `true` if the scroll is malevolent.
    pub fn is_malevolent(&self) -> bool {
        use ScrollKind::*;

        match self {
            AggravateMonsters => true,
            SummonMonsters =>  true,
            _ => false,
        }
    }       
}

impl std::fmt::Display for ScrollKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScrollKind::*;

        let result = match self {
            AggravateMonsters => "aggravate monsters",
            Discord => "discord",
            Enchanting => "enchanting",
            Identify => "identify",
            MagicMapping => "magic mapping",
            Negation => "negation",
            ProtectArmor => "protect armor",
            ProtectWeapon => "protect weapon",
            Recharging => "recharging",
            RemoveCurse => "remove curse",
            Sanctuary => "sanctuary",
            Shattering => "shattering",
            SummonMonsters => "summon monsters",
            Teleportation => "teleportation",
        };
        write!(f, "{}", result)
    }
}

const SCROLL_KINDS: [(&str, ScrollKind); 14] = [
    ("aggravate monsters", ScrollKind::AggravateMonsters),
    ("discord", ScrollKind::Discord),
    ("enchanting", ScrollKind::Enchanting),
    ("identify", ScrollKind::Identify),
    ("magic mapping", ScrollKind::MagicMapping),
    ("negation", ScrollKind::Negation),
    ("protect armor", ScrollKind::ProtectArmor),
    ("protect weapon", ScrollKind::ProtectWeapon),
    ("recharging", ScrollKind::Recharging),
    ("remove curse", ScrollKind::RemoveCurse),
    ("sanctuary", ScrollKind::Sanctuary),
    ("shattering", ScrollKind::Shattering),
    ("summon monsters", ScrollKind::SummonMonsters),
    ("teleportation", ScrollKind::Teleportation),
];
