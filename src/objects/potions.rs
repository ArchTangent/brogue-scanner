//! Potions for Brogue Seed Scanner.

/// Describes a Brogue Potion.
#[derive(Clone, Debug)]
pub struct Potion {
    kind: PotionKind,
}

impl Potion {
    pub fn new(kind: PotionKind) -> Self { 
        Self { kind } 
    }
}

impl std::fmt::Display for Potion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A potion of {}", self.kind)
    }
}

/// Kinds for the Potion Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PotionKind {
    CausticGas,    
    Confusion,
    CreepingDeath,       
    Darkness,
    Descent,
    DetectMagic,      
    FireImmunity,
    Hallucination,
    Incineration,
    Invisibility,
    Levitation,
    Life,
    Paralysis,
    Speed,
    Strength,
    Telepathy,
}

impl PotionKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in POTION_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in POTION_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }   
    /// Returns `true` if the potion is malevolent.
    pub fn is_malevolent(&self) -> bool {
        use PotionKind::*;

        match self {
            CausticGas => true,
            Confusion =>  true,
            CreepingDeath => true,
            Darkness => true,
            Descent => true,
            Hallucination => true,
            Incineration => true,
            Paralysis => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for PotionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PotionKind::*;

        let result = match self {
            CausticGas => "caustic gas",
            Confusion => "confusion",
            CreepingDeath => "creeping death",
            Darkness => "darkness",
            Descent => "descent",
            DetectMagic => "detect magic",
            FireImmunity => "fire immunity",
            Hallucination => "hallucination",
            Incineration => "incineration",
            Invisibility => "invisibility",
            Levitation => "levitation",
            Life => "life",
            Paralysis => "paralysis",
            Speed => "speed",
            Strength => "strength",
            Telepathy => "telepathy",
        };
        write!(f, "{}", result)
    }
}

const POTION_KINDS: [(&str, PotionKind); 16] = [
    ("caustic gas", PotionKind::CausticGas),
    ("confusion", PotionKind::Confusion),
    ("creeping death", PotionKind::CreepingDeath),
    ("darkness", PotionKind::Darkness),
    ("descent", PotionKind::Descent),
    ("detect magic", PotionKind::DetectMagic),
    ("fire immunity", PotionKind::FireImmunity),
    ("hallucination", PotionKind::Hallucination),
    ("incineration", PotionKind::Incineration),
    ("invisibility", PotionKind::Invisibility),
    ("levitation", PotionKind::Levitation),
    ("life", PotionKind::Life),
    ("paralysis", PotionKind::Paralysis),
    ("speed", PotionKind::Speed),
    ("strength", PotionKind::Strength),
    ("telepathy", PotionKind::Telepathy),
];
