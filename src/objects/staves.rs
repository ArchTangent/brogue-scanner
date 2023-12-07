//! Staves for Brogue Seed Scanner.

/// Describes a Brogue Staff.
#[derive(Clone, Debug)]
pub struct Staff {
    kind: StaffKind,
    enchantment: i8,    // Not an Option as all staves have an enchantment
}

impl Staff {
    pub fn new(kind: StaffKind, enchantment: i8) -> Self {   
        Self { kind, enchantment } 
    }
}

impl std::fmt::Display for Staff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A staff of {} [{}/{}] ", self.kind, self.enchantment, self.enchantment)
    }
}

/// Kinds for the Staff Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum StaffKind {
    Blinking,
    Conjuration,
    Discord,
    Entrancement,
    Firebolt,
    Haste,
    Healing,
    Lightning,
    Obstruction,
    Poison,
    Protection,
    Tunneling,
}

impl StaffKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in STAFF_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in STAFF_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }  
    /// Returns `true` if the staff is malevolent.
    pub fn is_malevolent(&self) -> bool {
        use StaffKind::*;

        match self {
            Haste => true,
            Healing => true,
            Protection => true,
            _ => false,
        }
    }    
}

impl std::fmt::Display for StaffKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StaffKind::*;

        let result = match self {
            Blinking => "blinking",
            Conjuration => "conjuration",
            Discord => "discord",
            Entrancement => "entrancement",
            Firebolt => "firebolt",
            Haste => "haste",
            Healing => "healing",
            Lightning => "lightning",
            Obstruction => "obstruction",
            Poison => "poison",
            Protection => "protection",
            Tunneling => "tunneling",
        };
        write!(f, "{}", result)
    }
}

const STAFF_KINDS: [(&str, StaffKind); 12] = [
    ("blinking", StaffKind::Blinking),
    ("conjuration", StaffKind::Conjuration),
    ("discord", StaffKind::Discord),
    ("entrancement", StaffKind::Entrancement),
    ("firebolt", StaffKind::Firebolt),
    ("haste", StaffKind::Haste),
    ("healing", StaffKind::Healing),
    ("lightning", StaffKind::Lightning),
    ("obstruction", StaffKind::Obstruction),
    ("poison", StaffKind::Poison),
    ("protection", StaffKind::Protection),
    ("tunneling", StaffKind::Tunneling),
];
