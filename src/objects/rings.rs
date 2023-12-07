//! Rings for Brogue Seed Scanner.

/// Describes a Brogue Ring.
#[derive(Clone, Debug)]
pub struct Ring {
    kind: RingKind,
    enchantment: i8,     // Not an Option as all rings have an enchantment
}

impl Ring {
    pub fn new(kind: RingKind, enchantment: i8) -> Self { 
        Self { kind, enchantment } 
    }
}

impl std::fmt::Display for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self.enchantment > 0 {
            true => "+",
            false => ""
        };
        write!(f, "A {}{} ring of {}", sign, self.enchantment, self.kind)
    }
}

/// Kinds for the Ring Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum RingKind {
    Awareness,
    Clairvoyance,
    Light,
    Reaping,
    Regeneration,
    Stealth,
    Transference,
    Wisdom,  
}

impl RingKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in RING_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in RING_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }       
}

impl std::fmt::Display for RingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            RingKind::Awareness => "awarness",
            RingKind::Clairvoyance => "clairvoyance",
            RingKind::Light => "light",
            RingKind::Reaping => "reaping",
            RingKind::Regeneration => "regeneration",
            RingKind::Stealth => "stealth",
            RingKind::Transference => "transference",
            RingKind::Wisdom => "wisdom",
        };
        write!(f, "{}", result)
    }
}

const RING_KINDS: [(&str, RingKind); 8] = [
    ("awareness", RingKind::Awareness),
    ("clairvoyance", RingKind::Clairvoyance),
    ("light", RingKind::Light),
    ("reaping", RingKind::Reaping),
    ("regeneration", RingKind::Regeneration),
    ("stealth", RingKind::Stealth),
    ("transference", RingKind::Transference),
    ("wisdom", RingKind::Wisdom),
];
