//! Monsters, allies, classes, and mutations for Brogue Seed Scanner.

/// Describes a Brogue Ally.
#[derive(Clone, Debug)]
pub struct Ally {
    kind: MonsterKind,
    status: AllyStatus,
    mutation: Option<Mutation>,
}

impl Ally {
    pub fn new(kind: MonsterKind, status: AllyStatus, mutation: Option<Mutation>) -> Self { 
        Self { kind, status, mutation } 
    }
}

impl std::fmt::Display for Ally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mutation {
            Some(mutation) => write!(f, "A {} {} <{}>", self.status, self.kind, mutation),
            None => write!(f, "A {} {}", self.status, self.kind),
        }        
    }
}

/// An ally's status, under the "ally_status" .csv header.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum AllyStatus {
    /// For legendary allies
    Allied,
    Caged,
    Shackled,
}

impl AllyStatus {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in ALLY_STATUS_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
}

impl std::fmt::Display for AllyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            AllyStatus::Allied => "legendary",
            AllyStatus::Caged => "caged",
            AllyStatus::Shackled => "shackled",
        };
        write!(f, "{}", result)
    }
}

/// Monster name, used under "kind" and "carried_by_monster_name" .csv headers.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MonsterKind {
    AcidMound,
    AcidicJelly,    
    ArrowTurret,    
    BlackJelly,   
    Bloat,
    BogMonster,     
    Centaur,
    Centipede,
    DarBattlemage,   
    DarBlademaster,    
    DarPriestess,    
    DartTurret,        
    Dragon,
    Eel,
    ExplosiveBloat,   
    FlameTurret,     
    Flamedancer,
    Fury,
    Goblin,
    GoblinConjurer,        
    GoblinMystic,        
    GoblinTotem,   
    GoblinWarlord,             
    Golem,
    GuardianSpirit,        
    Ifrit,
    Imp,
    Jackal,
    Kobold,
    Kraken,
    Lich,
    MangroveDryad,
    MirroredTotem,
    Monkey,
    Naga,
    Ogre,
    OgreShaman,
    OgreTotem,
    Phantom,
    Phoenix,
    PhoenixEgg,
    Phylactery,
    PinkJelly,
    PitBloat,
    Pixie,
    Rat,
    Revenant,
    Salamander,
    Sentinel,
    SparkTurret,
    SpectralBlade,
    // SpectralWeapon,
    Spider,
    StoneGuardian,
    TentacleHorror,
    Toad,
    Troll,
    Underworm,
    Unicorn,
    Vampire,
    VampireBat,
    WardenOfYendor,
    WilloTheWisp,
    WingedGuardian,
    Wraith,
    Zombie,
}

impl MonsterKind {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in MONSTER_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in MONSTER_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }    
}

impl std::fmt::Display for MonsterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            MonsterKind::BlackJelly => "black jelly",
            MonsterKind::Centaur => "centaur",
            MonsterKind::DarBattlemage => "dar battlemage",
            MonsterKind::DarBlademaster => "dar blademaster",
            MonsterKind::DarPriestess => "dar priestess",
            MonsterKind::Dragon => "dragon",
            MonsterKind::Flamedancer => "flamedancer",
            MonsterKind::Goblin => "goblin",
            MonsterKind::GoblinConjurer => "goblin conjurer",
            MonsterKind::GoblinMystic => "goblin mystic",
            MonsterKind::GoblinWarlord => "goblin warlord",
            MonsterKind::Golem => "golem",
            MonsterKind::Ifrit => "ifrit",
            MonsterKind::Imp => "imp",
            MonsterKind::MangroveDryad => "mangrove dryad",
            MonsterKind::Monkey => "monkey",
            MonsterKind::Naga => "naga",
            MonsterKind::Ogre => "ogre",
            MonsterKind::OgreShaman => "ogre shaman",
            MonsterKind::Phoenix => "phoenix",
            MonsterKind::PhoenixEgg => "phoenix egg",
            MonsterKind::Pixie => "pixie",
            MonsterKind::Salamander => "salamander",
            MonsterKind::StoneGuardian => "stone guardian",
            MonsterKind::TentacleHorror => "tentacle horror",
            MonsterKind::Troll => "troll",
            MonsterKind::Unicorn => "unicorn",
            MonsterKind::Vampire => "vampire",
            _ => "ERROR MONSTER KIND",
        };
        write!(f, "{}", result)
    }
}

/// Groups used to classify monsters in Brogue.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MonsterClass {
    Airborne,
    Abomination,
    Animal,
    Dar,
    Dragon,
    Fireborne,
    Goblin,
    Infernal,
    Jelly,
    Mage,
    Ogre,
    Troll,
    Turret,
    Undead,
    Waterborne,
}

impl std::fmt::Display for MonsterClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            MonsterClass::Airborne => "airborne",
            MonsterClass::Abomination => "abomination",
            MonsterClass::Animal => "animal",
            MonsterClass::Dar => "dar",
            MonsterClass::Dragon => "dragon",
            MonsterClass::Fireborne => "fireborne",
            MonsterClass::Goblin => "goblin",
            MonsterClass::Infernal => "infernal",
            MonsterClass::Jelly => "jelly",
            MonsterClass::Mage => "mage",
            MonsterClass::Ogre => "ogre",
            MonsterClass::Troll => "troll",
            MonsterClass::Turret => "turret",
            MonsterClass::Undead => "undead",
            MonsterClass::Waterborne => "waterborne",
        };
        write!(f, "{}", result)
    }
}

/// Mutations under the "mutation_name" .csv header.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Mutation {
    Agile,
    Explosive,
    Grappling,
    Infested,
    Juggernaut,
    Reflective,
    Toxic,
    Vampiric,
}

impl Mutation {
    /// Attempts to fully parse from a string using an _exact_ match.
    pub fn parse(value: &str) -> Option<Self> {
        for (name, kind) in MUTATION_KINDS.iter() {
            if name == &value {
                return Some(*kind)
            }
        }

        None
    }
    /// Attempts to parse from a string using a _partial_ match.
    pub fn parse_partial(value: &str) -> Option<Self> {
        for (name, kind) in MUTATION_KINDS.iter() {
            if name.contains(value) {
                return Some(*kind)
            }
        }

        None
    }        
}

impl std::fmt::Display for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Mutation::Agile => "agile",
            Mutation::Explosive => "explosive",
            Mutation::Grappling => "grappling",
            Mutation::Infested => "infested",
            Mutation::Juggernaut => "juggernaut",
            Mutation::Reflective => "reflective",
            Mutation::Toxic => "toxic",
            Mutation::Vampiric => "vampiric",
        };
        write!(f, "{}", result)
    }
}

//   ######    ######   ##    ##   ######  ########
//  ##    ##  ##    ##  ####  ##  ##          ##   
//  ##        ##    ##  ## ## ##   #####      ##   
//  ##    ##  ##    ##  ##  ####       ##     ##   
//   ######    ######   ##    ##  ######      ##   

const ALLY_STATUS_KINDS: [(&str, AllyStatus); 3] = [
    ("allied", AllyStatus::Allied),
    ("caged", AllyStatus::Caged),         
    ("shackled", AllyStatus::Shackled),         
];

const MONSTER_KINDS: [(&str, MonsterKind); 65] = [
    ("acid mound", MonsterKind::AcidMound),
    ("acidic jelly", MonsterKind::AcidicJelly),
    ("arrow turret", MonsterKind::ArrowTurret),
    ("black jelly", MonsterKind::BlackJelly),
    ("bloat", MonsterKind::Bloat),
    ("bog monster", MonsterKind::BogMonster),
    ("centaur", MonsterKind::Centaur),
    ("centipede", MonsterKind::Centipede),
    ("dar battlemage", MonsterKind::DarBattlemage),
    ("dar blademaster", MonsterKind::DarBlademaster),
    ("dar priestess", MonsterKind::DarPriestess),
    ("dart turret", MonsterKind::DartTurret),
    ("dragon", MonsterKind::Dragon),
    ("eel", MonsterKind::Eel),
    ("explosive bloat", MonsterKind::ExplosiveBloat),
    ("flame turret", MonsterKind::FlameTurret),
    ("flamedancer", MonsterKind::Flamedancer),
    ("fury", MonsterKind::Fury),
    ("goblin", MonsterKind::Goblin),
    ("goblin conjurer", MonsterKind::GoblinConjurer),
    ("goblin mystic", MonsterKind::GoblinMystic),
    ("goblin totem", MonsterKind::GoblinTotem),
    ("goblin warlord", MonsterKind::GoblinWarlord),
    ("golem", MonsterKind::Golem),
    ("guardian spirit", MonsterKind::GuardianSpirit),
    ("ifrit", MonsterKind::Ifrit),
    ("imp", MonsterKind::Imp),
    ("jackal", MonsterKind::Jackal),
    ("kobold", MonsterKind::Kobold),
    ("kraken", MonsterKind::Kraken),
    ("lich", MonsterKind::Lich),
    ("mangrove dryad", MonsterKind::MangroveDryad),
    ("mirrored totem", MonsterKind::MirroredTotem),
    ("monkey", MonsterKind::Monkey),
    ("naga", MonsterKind::Naga),
    ("ogre", MonsterKind::Ogre),
    ("ogre shaman", MonsterKind::OgreShaman),
    ("ogre totem", MonsterKind::OgreTotem),
    ("phantom", MonsterKind::Phantom),
    ("phoenix", MonsterKind::Phoenix),
    ("phoenix egg", MonsterKind::PhoenixEgg),
    ("phylactery", MonsterKind::Phylactery),
    ("pink jelly", MonsterKind::PinkJelly),
    ("pit bloat", MonsterKind::PitBloat),
    ("pixie", MonsterKind::Pixie),
    ("rat", MonsterKind::Rat),
    ("revenant", MonsterKind::Revenant),
    ("salamander", MonsterKind::Salamander),
    ("sentinel", MonsterKind::Sentinel),
    ("spark turret", MonsterKind::SparkTurret),
    ("spectral blade", MonsterKind::SpectralBlade),
    ("spider", MonsterKind::Spider),
    ("stone guardian", MonsterKind::StoneGuardian),
    ("tentacle horror", MonsterKind::TentacleHorror),
    ("toad", MonsterKind::Toad),
    ("troll", MonsterKind::Troll),
    ("underworm", MonsterKind::Underworm),
    ("unicorn", MonsterKind::Unicorn),
    ("vampire", MonsterKind::Vampire),
    ("vampire bat", MonsterKind::VampireBat),
    ("warden of yendor", MonsterKind::WardenOfYendor),
    ("will-o-the-wisp", MonsterKind::WilloTheWisp),
    ("winged guardian", MonsterKind::WingedGuardian),
    ("wraith", MonsterKind::Wraith),
    ("zombie", MonsterKind::Zombie),
];

const MUTATION_KINDS: [(&str, Mutation); 8] = [
    ("agile", Mutation::Agile),
    ("explosive", Mutation::Explosive),
    ("grappling", Mutation::Grappling),
    ("infested", Mutation::Infested),
    ("juggernaut", Mutation::Juggernaut),
    ("reflective", Mutation::Reflective),
    ("toxic", Mutation::Toxic),
    ("vampiric", Mutation::Vampiric),          
];
