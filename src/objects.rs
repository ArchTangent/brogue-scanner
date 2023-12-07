//! Game objects by category for Brogue Seed Scanner.

mod altars;
mod armor;
mod charms;
mod food;
mod gold;
mod keys;
mod monsters;
mod potions;
mod rings;
mod scrolls;
mod staves;
mod wands;
mod weapons;

use crate::bitflags::BitFlags16;
pub use altars::{Altar, AltarKind};
pub use armor::{Armor, ArmorKind, ArmorRunic};
pub use charms::{Charm, CharmKind};
pub use food::{Food, FoodKind};
pub use gold::{Gold, GoldKind};
pub use keys::{Key, KeyKind};
pub use monsters::{Ally, AllyStatus, MonsterClass, MonsterKind, Mutation};
pub use potions::{Potion, PotionKind};
pub use rings::{Ring, RingKind};
pub use scrolls::{Scroll, ScrollKind};
pub use staves::{Staff, StaffKind};
pub use wands::{Wand, WandKind};
pub use weapons::{Weapon, WeaponKind, WeaponRunic};

/// All in-game object categories, under the "category" .csv header.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Category {
    Ally = 1,
    Altar,      // Commutation, Resurrection
    Armor,
    Charm,
    Food,
    Gold,
    Key,
    Potion,
    Ring,
    Scroll,
    Staff,
    Wand,
    Weapon,
    /// Any object that isn't Altar, Ally, Food, Gold, or Key
    Item,
    /// Any object that can be equipped (Weapon/Armor/Ring)
    Equipment,
}

impl Category {
    /// Attempts to parse from a string.
    pub fn parse(value: &str) -> Option<Self> {
        const CATEGORIES: [(&str, Category); 15] = [
            ("potion", Category::Potion),
            ("scroll", Category::Scroll),
            ("weapon", Category::Weapon),
            ("armor", Category::Armor),
            ("ring", Category::Ring),
            ("staff", Category::Staff),
            ("wand", Category::Wand),
            ("charm", Category::Charm),
            ("food", Category::Food),
            ("gold", Category::Gold),
            ("key", Category::Key),
            ("ally", Category::Ally),
            ("altar", Category::Altar),
            ("item", Category::Item),
            ("equipment", Category::Equipment),
        ];

        for (name, kind) in CATEGORIES.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }
    /// Converts a `Category` into a u16 `BitFlags` representation.
    pub fn to_flags(&self) -> BitFlags16 {
        let mut flags = BitFlags16::new();

        match self {
            Category::Item => {
                let armor = BitFlags16::from_index(Self::Armor as usize); 
                let charm = BitFlags16::from_index(Self::Charm as usize); 
                let potion = BitFlags16::from_index(Self::Potion as usize); 
                let ring = BitFlags16::from_index(Self::Ring as usize); 
                let scroll = BitFlags16::from_index(Self::Scroll as usize); 
                let staff = BitFlags16::from_index(Self::Staff as usize); 
                let wand = BitFlags16::from_index(Self::Wand as usize); 
                let weapon = BitFlags16::from_index(Self::Weapon as usize); 

                flags.insert(armor);
                flags.insert(charm);
                flags.insert(potion);
                flags.insert(ring);
                flags.insert(scroll);
                flags.insert(staff);
                flags.insert(wand);
                flags.insert(weapon);
            }
            Category::Equipment => {
                let armor = BitFlags16::from_index(Self::Armor as usize); 
                let ring = BitFlags16::from_index(Self::Ring as usize); 
                let weapon = BitFlags16::from_index(Self::Weapon as usize); 

                flags.insert(armor);
                flags.insert(ring);
                flags.insert(weapon);
            }
            _ => {
                let val = BitFlags16::from_index(*self as usize);
                flags.insert(val);
            }
        }

        flags
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Category::Ally => { "ally" }
            Category::Altar => { "altar" }
            Category::Armor => { "armor" }
            Category::Charm => { "charm" }
            Category::Food => { "food" }
            Category::Gold => { "gold" }
            Category::Key => { "key" }
            Category::Potion => { "potion" }
            Category::Ring => { "ring" }
            Category::Scroll => { "scroll" }
            Category::Staff => { "staff" }
            Category::Wand => { "wand" }
            Category::Weapon => { "weapon" }
            Category::Item => { "item" }
            Category::Equipment => { "equipment" }
        };
        write!(f, "{}", val)
    }
}

/// Any in-game item or monster, distinguished by Category and Kind.
#[derive(Clone, Debug)]
pub enum Object {
    Ally(Ally),
    Altar(Altar),
    Armor(Armor),
    Charm(Charm),
    Food(Food),
    Gold(Gold),
    Key(Key),
    Ring(Ring),
    Potion(Potion),
    Scroll(Scroll),
    Staff(Staff),
    Wand(Wand),
    Weapon(Weapon),
}

impl Object {
    /// Makes a new `Object` from monster data.
    pub fn new_ally(kind: MonsterKind, status: AllyStatus, mutation: Option<Mutation>) -> Self {
        Object::Ally(Ally::new(kind, status, mutation))
    }
    /// Makes a new `Object` from altar data.
    pub fn new_altar(kind: AltarKind) -> Self {
        Object::Altar(Altar::new(kind))
    }    
    /// Makes a new `Object` from armor data.
    pub fn new_armor(kind: ArmorKind, enchantment: i8, runic: Option<ArmorRunic>) -> Self {
        Object::Armor(Armor::new(kind, enchantment, runic))
    }
    /// Makes a new `Object` from charm data.
    pub fn new_charm(kind: CharmKind, enchantment: i8) -> Self {
        Object::Charm(Charm::new(kind, enchantment))
    }
    /// Makes a new `Object` from food data.
    pub fn new_food(kind: FoodKind) -> Self {
        Object::Food(Food::new(kind))
    }
    /// Makes a new `Object` from gold data.
    pub fn new_gold(kind: GoldKind, count: u32) -> Self {
        Object::Gold(Gold::new(kind, count))
    }
    /// Makes a new `Object` from key data.
    pub fn new_key(kind: KeyKind, opens: Option<u8>) -> Self {
        Object::Key(Key::new(kind, opens))
    }     
    /// Makes a new `Object` from potion data.
    pub fn new_potion(kind: PotionKind) -> Self {
        Object::Potion(Potion::new(kind))
    } 
    /// Makes a new `Object` from ring data.
    pub fn new_ring(kind: RingKind, enchantment: i8) -> Self {
        Object::Ring(Ring::new(kind, enchantment))
    }       
    /// Makes a new `Object` from scroll data.
    pub fn new_scroll(kind: ScrollKind) -> Self {
        Object::Scroll(Scroll::new(kind))
    }        
    /// Makes a new `Object` from staff data.
    pub fn new_staff(kind: StaffKind, enchantment: i8) -> Self {
        Object::Staff(Staff::new(kind, enchantment))
    }    
    /// Makes a new `Object` from wand data.
    pub fn new_wand(kind: WandKind, enchantment: i8) -> Self {
        Object::Wand(Wand::new(kind, enchantment))
    }
    /// Makes a new `Object` from weapon data.
    pub fn new_weapon(kind: WeaponKind, enchantment: i8, runic: Option<WeaponRunic>) -> Self {
        Object::Weapon(Weapon::new(kind, enchantment, runic))
    }   
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Object::Ally(o) => format!("{}", o),
            Object::Altar(o) => format!("{}", o),
            Object::Armor(o) => format!("{}", o),
            Object::Charm(o) => format!("{}", o),
            Object::Food(o) => format!("{}", o),
            Object::Gold(o) => format!("{}", o),
            Object::Key(o) => format!("{}", o),
            Object::Ring(o) => format!("{}", o),
            Object::Potion(o) => format!("{}", o),
            Object::Scroll(o) => format!("{}", o),
            Object::Staff(o) => format!("{}", o),
            Object::Wand(o) => format!("{}", o),
            Object::Weapon(o) => format!("{}", o),
        };
        write!(f, "{}", result)
    }
}

/// Magic type (Benevolent, Malevolent) for Potions, Scrolls, Staves, and Wands.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MagicType {
    Benevolent,
    Malevolent,
}

impl std::fmt::Display for MagicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicType::Benevolent => write!(f, "benevolent"),
            MagicType::Malevolent => write!(f, "malevolent"),
        }
    }
}
