//! Gold for Brogue Seed Scanner.

/// Describes a Gold item in Brogue.
#[derive(Clone, Debug)]
pub struct Gold {
    count: u32,
    kind: GoldKind,
}

impl Gold {
    pub fn new(kind: GoldKind, count: u32) -> Self { 
        Self { count, kind } 
    }
}

impl std::fmt::Display for Gold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.count, self.kind)
    }
}

/// Kinds for the Gold Category.
#[derive(Clone, Copy, Debug)]
pub struct GoldKind {
    piles: u16,
}

impl GoldKind {
    /// Attempts to parse from a string.
    pub fn parse(value: &str) -> Option<Self> {
        // Handle multiple (most common) or single piles of gold
        if value.len() >= 13 {
            if let Ok(piles) = value.split_at(13).1.split_at(2).0.trim().parse::<u16>() {
                return Some(GoldKind { piles });
            }
        } else if value == "gold pieces" {
            return Some(GoldKind { piles: 1 });
        }
        
        // if let Ok(piles) = value.split_at(13).1.split_at(2).0.trim().parse::<u16>() {
        //     return Some(GoldKind { piles });
        // }

        None
    }
}

impl std::fmt::Display for GoldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gold pieces ({} piles)", self.piles)
    }
}
