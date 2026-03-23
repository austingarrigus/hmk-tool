use anyhow_serde::Result;
use std::{fmt::Display, fs::File, io::Read};

use rand::random_range;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::armor::{self, Location, Material};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Melee {
        length: u8,
        thrusting: bool,
        two_hand: bool,
        aspect: Aspect,
        zone_die: u8,
        impact_die: u8,
        impact_mod: u8,
    },
    Range {
        draw: u16,
        two_hand: bool,
        base_range: u16,
        impact_mod: u8,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Item {
    Weapon {
        name: String,
        heft: u8,
        defense_mod: i8,
        modes: Vec<Mode>,
        description: String,
    },
    Armor {
        name: String,
        material: Material,
        covers: Vec<Location>,
        description: String,
    },
    Ammo {
        shaft: ProjectileShaft,
        head: ProjectileHead,
    },
    Misc {
        name: String,
        description: String,
    },
}

/*
fn test() {
    let weapon = Item::Weapon { name: String::from("Test") , heft: 1, defense_mod: 1, modes: (), description: () }
}
*/

impl Item {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Self = toml::from_slice(&buf)?;
        Ok(o)
    }
}

#[derive(Debug)]
pub struct Attack {
    pub location: armor::Location,
    pub impact: u8,
    pub aspect: Aspect,
}

impl Attack {
    pub fn new(location: armor::Location, impact: u8, aspect: Aspect) -> Self {
        Attack {
            location,
            impact,
            aspect,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Aspect {
    Blunt,
    Edge,
    Point,
    FireFrost,
}

impl Display for Aspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aspect::Blunt => write!(f, "B"),
            Aspect::Edge => write!(f, "E"),
            Aspect::Point => write!(f, "P"),
            Aspect::FireFrost => write!(f, "F"),
        }
    }
}

impl Aspect {
    pub fn array_index(&self) -> usize {
        match self {
            Aspect::Blunt => 0,
            Aspect::Edge => 1,
            Aspect::Point => 2,
            Aspect::FireFrost => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Group {
    Axes,
    Bows,
    Crossbows,
    Slings,
    Clubs,
    Flails,
    Knives,
    Polearms,
    Shields,
    Swords,
    Unarmed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Restriction {
    Common,
    Simple,
    Chivalric,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum DefenseOption {
    Block,
    Dodge,
    Counterstrike,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeWeapon {
    pub name: String,
    pub group: Group,
    pub restriction: Restriction,
    pub quality: u8,
    pub heft: u8,
    pub length: u8,
    pub thrusting: bool,
    pub aspect: Aspect,
    pub zone_die: u8,
    pub impact_die: u8,
    pub impact_mod: u8,
    pub weight: u8,
    pub price: u16,
    pub description: String,
}

impl MeleeWeapon {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Self = toml::from_slice(&buf)?;
        Ok(o)
    }
}

#[derive(EnumIter, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProjectileShaft {
    Heavy,
    Light,
}

#[derive(EnumIter, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProjectileHead {
    Bodkin,
    Blunt,
    Broad,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Projectile {
    pub shaft: ProjectileShaft,
    pub head: ProjectileHead,
}

impl Projectile {
    pub fn calc_impact(&self, modifier: i8) -> u8 {
        let impact_die = match self.shaft {
            ProjectileShaft::Heavy => 12,
            ProjectileShaft::Light => 8,
        };
        let head_mod_div = match self.head {
            ProjectileHead::Blunt => 2,
            _ => 1,
        };
        ((random_range(1..=impact_die) + modifier) / head_mod_div) as u8
    }
}

#[derive(Clone, Copy)]
pub enum MissileRange {
    DirectPB,
    Direct,
    Volley2,
    Volley3,
    Volley4,
    None,
}

impl MissileRange {
    pub fn test_mod(&self) -> Option<i8> {
        match self {
            MissileRange::DirectPB => Some(10),
            MissileRange::Direct => Some(0),
            MissileRange::Volley2 => Some(0),
            MissileRange::Volley3 => Some(-20),
            MissileRange::Volley4 => Some(-40),
            MissileRange::None => None,
        }
    }

    pub fn zone_die(&self) -> Option<u8> {
        match self {
            MissileRange::DirectPB => Some(6),
            MissileRange::Direct => Some(8),
            MissileRange::Volley2 => Some(10),
            MissileRange::Volley3 => Some(10),
            MissileRange::Volley4 => Some(10),
            MissileRange::None => None,
        }
    }

    pub fn impact_mod(&self) -> Option<i8> {
        match self {
            MissileRange::DirectPB => Some(2),
            MissileRange::Direct => Some(0),
            MissileRange::Volley2 => Some(-2),
            MissileRange::Volley3 => Some(-3),
            MissileRange::Volley4 => Some(-4),
            MissileRange::None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeWeapon {
    pub name: String,
    pub group: Group,
    pub restriction: Restriction,
    pub quality: u8,
    pub draw: u16,
    pub base_range: u16,
    pub impact_mod: u8,
    pub weight: u8,
    pub price: u16,
    pub description: String,
}

impl RangeWeapon {
    pub fn calc_range(&self, value: u16) -> MissileRange {
        if value >= self.base_range * 2 {
            MissileRange::DirectPB
        } else if value >= self.base_range {
            MissileRange::Direct
        } else if value * 2 >= self.base_range {
            MissileRange::Volley2
        } else if value * 3 >= self.base_range {
            MissileRange::Volley3
        } else if value * 4 >= self.base_range {
            MissileRange::Volley4
        } else {
            MissileRange::None
        }
    }
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Self = toml::from_slice(&buf)?;
        Ok(o)
    }
}
