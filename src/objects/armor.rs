//! Armor and armor runics for Brogue Seed Scanner.

use super::MonsterClass;

/// Describes a piece of Brogue Armor.
#[derive(Clone, Debug)]
pub struct Armor {
    kind: ArmorKind,
    enchantment: i8,
    runic: Option<ArmorRunic>,
}

impl Armor {
    pub fn new(kind: ArmorKind, enchantment: i8, runic: Option<ArmorRunic>) -> Self {
         Self { kind, enchantment, runic } 
    }
}

impl std::fmt::Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ench = self.enchantment;
        let sign = match self.enchantment >= 0 {
            true => "+",
            false => ""
        };       
        match self.runic {
            Some(runic) =>write!(f, "A {}{} {} of {}", sign, ench, self.kind, runic),
            None => write!(f, "A {}{} {}", sign, ench, self.kind),
        }          
    }
}

/// Kinds for the Armor Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ArmorKind {
    BandedMail, 
    ChainMail,
    LeatherArmor,          
    PlateMail,          
    ScaleMail,              
    SplintMail,   
}

impl ArmorKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in ARMOR_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in ARMOR_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }       
}

impl std::fmt::Display for ArmorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ArmorKind::BandedMail => "banded mail",
            ArmorKind::ChainMail => "chain mail",
            ArmorKind::LeatherArmor => "leather armor",
            ArmorKind::PlateMail => "plate mail",
            ArmorKind::ScaleMail => "scale mail",
            ArmorKind::SplintMail => "splint mail",
        };
        write!(f, "{}", result)
    }
}


// Runics for Armor.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ArmorRunic {
    // --- Positive --- //
    Absorption,
    Dampening,
    Immunity(MonsterClass),
    Multiplicity,
    Mutuality,
    Reflection,
    Reprisal,
    Respiration,
    // --- Negative --- //
    Burden,
    Immolation,
    Vulnerability,    
}

impl ArmorRunic {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in ARMOR_RUNICS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in ARMOR_RUNICS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }         
}

impl std::fmt::Display for ArmorRunic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmorRunic::Absorption => write!(f, "absorption"),
            ArmorRunic::Dampening => write!(f, "dampening"),
            ArmorRunic::Multiplicity => write!(f, "multiplicity"),
            ArmorRunic::Mutuality => write!(f, "mutuality"),
            ArmorRunic::Reflection => write!(f, "reflection"),
            ArmorRunic::Reprisal => write!(f, "reprisal"),
            ArmorRunic::Respiration => write!(f, "respiration"),
            ArmorRunic::Burden => write!(f, "burden"),
            ArmorRunic::Immolation => write!(f, "immolation"),
            ArmorRunic::Vulnerability => write!(f, "vulnerability"),
            ArmorRunic::Immunity(mclass) => write!(f, "{} immunity", mclass),            
        }
    }
}

const ARMOR_KINDS: [(&str, ArmorKind); 6] = [
    ("banded mail", ArmorKind::BandedMail),
    ("chain mail", ArmorKind::ChainMail),
    ("leather armor", ArmorKind::LeatherArmor),
    ("plate armor", ArmorKind::PlateMail),
    ("scale mail", ArmorKind::ScaleMail),
    ("splint mail", ArmorKind::SplintMail),            
];

const ARMOR_RUNICS: [(&str, ArmorRunic); 25] = [
    // --- Positive --- //
    ("absorption", ArmorRunic::Absorption),
    ("dampening", ArmorRunic::Dampening),
    ("multiplicity", ArmorRunic::Multiplicity),
    ("mutuality", ArmorRunic::Mutuality),
    ("reflection", ArmorRunic::Reflection),
    ("reprisal", ArmorRunic::Reprisal),
    ("respiration", ArmorRunic::Respiration),                     
    // --- Negative --- //
    ("burden", ArmorRunic::Burden),            
    ("immolation", ArmorRunic::Immolation),            
    ("vulnerability", ArmorRunic::Vulnerability),            
    // --- Immunity --- //
    ("airborne immunity", ArmorRunic::Immunity(MonsterClass::Airborne)),            
    ("abomination immunity", ArmorRunic::Immunity(MonsterClass::Abomination)),
    ("animal immunity", ArmorRunic::Immunity(MonsterClass::Animal)),
    ("dar immunity", ArmorRunic::Immunity(MonsterClass::Dar)),       
    ("dragon immunity", ArmorRunic::Immunity(MonsterClass::Dragon)),
    ("fireborne immunity", ArmorRunic::Immunity(MonsterClass::Fireborne)),
    ("goblin immunity", ArmorRunic::Immunity(MonsterClass::Goblin)),       
    ("infernal immunity", ArmorRunic::Immunity(MonsterClass::Infernal)),
    ("jelly immunity", ArmorRunic::Immunity(MonsterClass::Jelly)),      
    ("mage immunity", ArmorRunic::Immunity(MonsterClass::Mage)),
    ("ogre immunity", ArmorRunic::Immunity(MonsterClass::Ogre)),
    ("troll immunity", ArmorRunic::Immunity(MonsterClass::Troll)),
    ("turret immunity", ArmorRunic::Immunity(MonsterClass::Turret)),
    ("undead immunity", ArmorRunic::Immunity(MonsterClass::Undead)),
    ("waterborne immunity", ArmorRunic::Immunity(MonsterClass::Waterborne)),    
];
