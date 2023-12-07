//! Weapons and weapon runics for Brogue Seed Scanner.

use crate::objects::MonsterClass;

/// Describes a Brogue Weapon.
#[derive(Clone, Debug)]
pub struct Weapon {
    kind: WeaponKind,
    enchantment: i8,
    runic: Option<WeaponRunic>,
}

impl Weapon {
    pub fn new(kind: WeaponKind, enchantment: i8, runic: Option<WeaponRunic>) -> Self { 
        Self { kind, enchantment, runic } 
    }
}

impl std::fmt::Display for Weapon {
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

/// Kinds for the Weapon Category.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum WeaponKind {
    Axe,
    Broadsword,
    Dagger,
    Dart,
    Flail,
    IncendiaryDart,
    Javelin,
    Mace,
    Rapier,
    Spear,
    Sword,
    WarAxe,
    WarHammer,
    WarPike,    
    Whip, 
}

impl WeaponKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in WEAPON_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in WEAPON_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }   
}

impl std::fmt::Display for WeaponKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            WeaponKind::Axe => "axe",
            WeaponKind::Broadsword => "broadsword",
            WeaponKind::Dagger => "dagger",
            WeaponKind::Dart => "dart",
            WeaponKind::Flail => "flail",
            WeaponKind::IncendiaryDart => "incendiary dart",
            WeaponKind::Javelin => "javelins",
            WeaponKind::Mace => "mace",
            WeaponKind::Rapier => "rapier",
            WeaponKind::Spear => "spear",
            WeaponKind::Sword => "sword",
            WeaponKind::WarAxe => "war axe",
            WeaponKind::WarHammer => "war hammer",
            WeaponKind::WarPike => "war pike",
            WeaponKind::Whip => "whip",
        };
        write!(f, "{}", result)
    }
}


// Runics for Weapons.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum WeaponRunic {
    // --- Positive --- //
    Confusion,
    Force,
    Multiplicity,       // Shared with Armor
    Paralysis,
    Quietus,
    Slaying(MonsterClass),
    Slowing,
    Speed,
    // --- Negative --- //
    Mercy,
    Plenty,
}

impl WeaponRunic {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in WEAPON_RUNICS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in WEAPON_RUNICS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }   
}

impl std::fmt::Display for WeaponRunic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponRunic::Confusion => write!(f, "confusion"),
            WeaponRunic::Force => write!(f, "force"),
            WeaponRunic::Multiplicity => write!(f, "multiplicity"),
            WeaponRunic::Paralysis => write!(f, "paralysis"),
            WeaponRunic::Quietus => write!(f, "quietus"),
            WeaponRunic::Slowing => write!(f, "slowing"),
            WeaponRunic::Speed => write!(f, "speed"),
            WeaponRunic::Mercy => write!(f, "mercy"),
            WeaponRunic::Plenty => write!(f, "plenty"),
            WeaponRunic::Slaying(mclass) => write!(f, "{} slaying", mclass),            
        }
    }
}

const WEAPON_KINDS: [(&str, WeaponKind); 15] = [
    // Sword types
    ("broadsword", WeaponKind::Broadsword),
    ("dagger", WeaponKind::Dagger),
    ("sword", WeaponKind::Sword),
    // Mace types
    ("mace", WeaponKind::Mace),
    ("war hammer", WeaponKind::WarHammer),
    // Spear types
    ("spear", WeaponKind::Spear),
    ("war pike", WeaponKind::WarPike),
    // Axe types            
    ("war axe", WeaponKind::WarAxe),
    ("axe", WeaponKind::Axe),
    // Rapier types                   
    ("rapier", WeaponKind::Rapier),
    // Whip types
    ("whip", WeaponKind::Whip),
    // Flail types
    ("flail", WeaponKind::Flail),
    // Thrown types
    ("incendiary dart", WeaponKind::IncendiaryDart),
    ("dart", WeaponKind::Dart),          
    ("javelin", WeaponKind::Javelin),          
];

const WEAPON_RUNICS: [(&str, WeaponRunic); 24] = [
    // --- Positive --- //
    ("confusion", WeaponRunic::Confusion),
    ("force", WeaponRunic::Force),
    ("multiplicity", WeaponRunic::Multiplicity),
    ("paralysis", WeaponRunic::Paralysis),
    ("quietus", WeaponRunic::Quietus),
    ("slowing", WeaponRunic::Slowing),
    ("speed", WeaponRunic::Speed),                  
    // --- Negative --- //
    ("mercy", WeaponRunic::Mercy),            
    ("plenty", WeaponRunic::Plenty),            
    // --- Slaying --- //
    ("airborne slaying", WeaponRunic::Slaying(MonsterClass::Airborne)),            
    ("abomination slaying", WeaponRunic::Slaying(MonsterClass::Abomination)),
    ("animal slaying", WeaponRunic::Slaying(MonsterClass::Animal)),
    ("dar slaying", WeaponRunic::Slaying(MonsterClass::Dar)),       
    ("dragon slaying", WeaponRunic::Slaying(MonsterClass::Dragon)),
    ("fireborne slaying", WeaponRunic::Slaying(MonsterClass::Fireborne)),
    ("goblin slaying", WeaponRunic::Slaying(MonsterClass::Goblin)),       
    ("infernal slaying", WeaponRunic::Slaying(MonsterClass::Infernal)),
    ("jelly slaying", WeaponRunic::Slaying(MonsterClass::Jelly)),      
    ("mage slaying", WeaponRunic::Slaying(MonsterClass::Mage)),
    ("ogre slaying", WeaponRunic::Slaying(MonsterClass::Ogre)),
    ("troll slaying", WeaponRunic::Slaying(MonsterClass::Troll)),
    ("turret slaying", WeaponRunic::Slaying(MonsterClass::Turret)),
    ("undead slaying", WeaponRunic::Slaying(MonsterClass::Undead)),
    ("waterborne slaying", WeaponRunic::Slaying(MonsterClass::Waterborne)),    
];
