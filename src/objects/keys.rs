//! Keys for Brogue Seed Scanner.

/// Describes a Brogue Key.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Key {
    kind: KeyKind,
    /// Vault number this key opens, if any
    opens: Option<u8>,
}

impl Key {
    pub fn new(kind: KeyKind, opens: Option<u8>) -> Self { 
        Self { kind, opens } 
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A {}", self.kind)
    }
}


/// Kinds for the Key Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum KeyKind {
    CageKey,    
    CrystalOrb,    
    DoorKey,    
}

impl KeyKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in KEY_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
}

impl std::fmt::Display for KeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            KeyKind::DoorKey => "door key",
            KeyKind::CageKey => "cage key",
            KeyKind::CrystalOrb => "crystal orb",
        };
        write!(f, "{}", result)
    }
}

const KEY_KINDS: [(&str, KeyKind); 3] = [
    ("door key", KeyKind::DoorKey),
    ("cage key", KeyKind::CageKey),
    ("crystal orb", KeyKind::CrystalOrb),
];
