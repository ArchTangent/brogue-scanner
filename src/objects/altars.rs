//! Altars for Brogue Seed Scanner.

/// Describes a Brogue Altar.
#[derive(Clone, Debug)]
pub struct Altar {
    kind: AltarKind,
}

impl Altar {
    pub fn new(kind: AltarKind) -> Self { 
        Self { kind } 
    }
}

impl std::fmt::Display for Altar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A {}", self.kind)
    }
}

/// Kinds for the Charm Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum AltarKind {
    CommutationAltar,   
    ResurrectionAltar,
}

impl AltarKind {
    /// Attempts to parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in ALTAR_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }       
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in ALTAR_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }   
}

impl std::fmt::Display for AltarKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            AltarKind::CommutationAltar => "commutation altar",
            AltarKind::ResurrectionAltar => "resurrection altar",
        };
        write!(f, "{}", result)
    }
}

const ALTAR_KINDS: [(&str, AltarKind); 2] = [
    ("commutation altar", AltarKind::CommutationAltar),
    ("resurrection altar", AltarKind::ResurrectionAltar),
];
