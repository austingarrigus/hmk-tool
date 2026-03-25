use crate::{
    being::{self, Being, Location},
    core::{Skill, SuccessResult},
};
use anyhow_serde::Result;
use rand::random_range;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
    ops::Deref,
};
use strum::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Mode {
    Melee {
        length: u8,
        heft: u8,
        thrust: bool,
        two_hand: bool,
        aspect: Aspect,
        zone_die: u8,
        impact_die: i8,
        impact_mod: u8,
        defense_mod: i8,
    },
    Range {
        draw: u16,
        two_hand: bool,
        base_range: u16,
        impact_mod: u8,
    },
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Melee {
                length,
                thrust,
                aspect,
                zone_die,
                impact_die,
                impact_mod,
                ..
            } => write!(
                f,
                "{length}{} d{zone_die} d{impact_die}+{impact_mod}{aspect}",
                if *thrust { "T" } else { "" }
            ),
            Mode::Range {
                draw, impact_mod, ..
            } => write!(f, "{draw}lb •+{impact_mod}P"),
        }
    }
}

impl Mode {
    pub fn reach_diff(self, rhs: Mode) -> (i8, i8) {
        match self {
            Mode::Melee {
                length: atk_length,
                thrust: atk_thrust,
                ..
            } => match rhs {
                Mode::Melee {
                    length: def_length,
                    thrust: def_thrust,
                    ..
                } => {
                    let reach_diff = ((atk_length).cast_signed() - def_length.cast_signed()) * 5;
                    (
                        if reach_diff > 0 && atk_thrust {
                            reach_diff
                        } else {
                            0
                        },
                        if reach_diff < 0 && def_thrust {
                            reach_diff
                        } else {
                            0
                        },
                    )
                }
                Mode::Range { .. } => (0, 0),
            },
            Mode::Range { .. } => (0, 0),
        }
    }

    pub fn heft(self) -> u8 {
        match self {
            Mode::Melee { heft, .. } => heft,
            Mode::Range { .. } => 0,
        }
    }

    pub fn defense_mod(self) -> i8 {
        match self {
            Mode::Melee { defense_mod, .. } => defense_mod,
            Mode::Range { .. } => 0,
        }
    }

    pub fn aspect(self) -> Aspect {
        // TODO: Figure out ranged aspects besides arrows
        match self {
            Mode::Melee { aspect, .. } => aspect,
            Mode::Range { .. } => Aspect::Point,
        }
    }

    pub fn impact_die(self) -> i8 {
        match self {
            Mode::Melee { impact_die, .. } => impact_die,
            Mode::Range { .. } => 1,
        }
    }

    pub fn impact_mod(self) -> u8 {
        match self {
            Mode::Melee { impact_mod, .. } | Mode::Range { impact_mod, .. } => impact_mod,
        }
    }

    pub fn calc_range(self, value: u16) -> MissileRange {
        match self {
            Mode::Range { base_range, .. } => {
                if value <= base_range / 2 {
                    MissileRange::DirectPB
                } else if value <= base_range {
                    MissileRange::Direct
                } else if value <= base_range * 2 {
                    MissileRange::Volley2
                } else if value <= base_range * 3 {
                    MissileRange::Volley3
                } else if value <= base_range * 4 {
                    MissileRange::Volley4
                } else {
                    MissileRange::None
                }
            }
            Mode::Melee { .. } => MissileRange::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MissileRange {
    DirectPB,
    Direct,
    Volley2,
    Volley3,
    Volley4,
    None,
}

impl MissileRange {
    pub fn test_mod(self) -> Option<i32> {
        match self {
            MissileRange::DirectPB => Some(10),
            MissileRange::Direct | MissileRange::Volley2 => Some(0),
            MissileRange::Volley3 => Some(-20),
            MissileRange::Volley4 => Some(-40),
            MissileRange::None => None,
        }
    }

    pub fn zone_die(self) -> Option<u8> {
        match self {
            MissileRange::DirectPB => Some(6),
            MissileRange::Direct => Some(8),
            MissileRange::Volley2 | MissileRange::Volley3 | MissileRange::Volley4 => Some(10),
            MissileRange::None => None,
        }
    }

    pub fn impact_mod(self) -> Option<i8> {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Item {
    Weapon {
        name: String,
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Armor { name, material, .. } => write!(f, "{material} {name}"),
            Item::Ammo { shaft, head } => write!(f, "{shaft:?} {head:?}"),
            Item::Weapon { name, .. } | Item::Misc { name, .. } => write!(f, "{name}"),
        }
    }
}

impl Item {
    pub fn calc_impact(&self, modifier: i8) -> i8 {
        match self {
            Item::Ammo { shaft, head } => {
                let impact_die = match shaft {
                    ProjectileShaft::Heavy => 12,
                    ProjectileShaft::Light => 8,
                };
                let head_mod_div = match head {
                    ProjectileHead::Blunt => 2,
                    _ => 1,
                };
                (random_range(1..=impact_die) + modifier) / head_mod_div
            }
            _ => 0,
        }
    }

    pub fn from_file<P>(path: P) -> Result<Item>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Item = toml::from_slice(&buf)?;
        Ok(o)
    }

    pub fn write_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let buf = toml::to_string_pretty(self)?;
        File::create(path)?.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn modes(&self) -> Vec<Mode> {
        match self {
            Item::Weapon { modes, .. } => modes.clone(),
            _ => Vec::new(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Inventory(Vec<Item>);

impl Deref for Inventory {
    type Target = Vec<Item>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Inventory {
    pub fn ammo(&self) -> Vec<Item> {
        // TODO: select a type of ammo from options
        self.iter()
            .filter(|x| matches!(x, Item::Ammo { .. }))
            .cloned()
            .collect()
    }

    pub fn protection(&self, location: Location, aspect: Aspect) -> i8 {
        self.0.iter().fold(0, |mut acc, x| {
            if let Item::Armor {
                material, covers, ..
            } = x
            {
                for y in covers {
                    if *y == location {
                        acc += material.av()[aspect.array_index()];
                    }
                }
            }
            acc
        })
    }

    pub fn calc_injury(&self, attack: Attack) -> Option<being::Injury> {
        println!("Hits {}", attack.location);
        being::Injury::new(
            attack.location,
            attack
                .impact
                .saturating_sub(self.protection(attack.location, attack.aspect)),
            attack.aspect,
            attack.location.bleed(),
        )
    }

    pub fn equip(&mut self, item: Item) {
        self.0.push(item);
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
    pub fn array_index(self) -> usize {
        match self {
            Aspect::Blunt => 0,
            Aspect::Edge => 1,
            Aspect::Point => 2,
            Aspect::FireFrost => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attack {
    pub location: Location,
    pub impact: i8,
    pub aspect: Aspect,
}

impl Attack {
    pub fn new(location: Location, impact: i8, aspect: Aspect) -> Self {
        Attack {
            location,
            impact,
            aspect,
        }
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Material {
    #[default]
    Cloth,
    Leather,
    Padded,
    Quilted,
    Gambeson,
    Kurbul,
    Scale,
    Mail,
    Plate,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cloth => "Cloth",
                Self::Leather => "Leather",
                Self::Padded => "Padded",
                Self::Quilted => "Quilted",
                Self::Gambeson => "Gambeson",
                Self::Kurbul => "Kûrbúl",
                Self::Scale => "Scale",
                Self::Mail => "Mail",
                Self::Plate => "Plate",
            }
        )
    }
}

impl Material {
    fn av(self) -> [i8; 4] {
        match self {
            Self::Cloth => [0, 1, 0, 1],
            Self::Leather => [1, 2, 1, 3],
            Self::Padded => [2, 2, 1, 2],
            Self::Quilted => [4, 3, 2, 3],
            Self::Gambeson => [6, 5, 4, 5],
            Self::Kurbul => [4, 6, 5, 4],
            Self::Scale => [4, 8, 5, 5],
            Self::Mail => [2, 8, 7, 3],
            Self::Plate => [6, 11, 9, 5],
        }
    }
    fn rigidity(self) -> Rigidity {
        match self {
            Self::Cloth | Self::Leather | Self::Padded | Self::Quilted => Rigidity::None,
            Self::Gambeson | Self::Kurbul => Rigidity::Rigid,
            Self::Scale | Self::Mail => Rigidity::Mail,
            Self::Plate => Rigidity::Plate,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rigidity {
    None,
    Rigid,
    Mail,
    Plate,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, EnumIter)]
pub enum DefenseOption {
    Block,
    Dodge,
    Counterstrike,
}

impl Display for DefenseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl DefenseOption {
    pub fn test(self, being: &Being, mode: Mode, def_mod: i8, reach_penalty: i8) -> SuccessResult {
        let basic_mod = (mode.defense_mod() + being.heft_mod(mode).unwrap_or(0) + def_mod).into();
        match self {
            DefenseOption::Block => being.success_test(&Skill::Melee, basic_mod),
            DefenseOption::Dodge => being.success_test(&Skill::Dodge, basic_mod),
            DefenseOption::Counterstrike => {
                being.success_test(&Skill::Melee, basic_mod - reach_penalty as i32)
            }
        }
    }
}
