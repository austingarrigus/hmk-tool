use std::{fmt::Display, fs::File, io::Read};

use anyhow_serde::{Result, bail};
use itertools::Itertools;
use rand::random_range;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::weapons::{self, Attack};
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
                Material::Cloth => "Cloth",
                Material::Leather => "Leather",
                Material::Padded => "Padded",
                Material::Quilted => "Quilted",
                Material::Gambeson => "Gambeson",
                Material::Kurbul => "Kûrbúl",
                Material::Scale => "Scale",
                Material::Mail => "Mail",
                Material::Plate => "Plate",
            }
        )
    }
}

impl Material {
    fn av(&self) -> [u8; 4] {
        match self {
            Material::Cloth => [0, 1, 0, 1],
            Material::Leather => [1, 2, 1, 3],
            Material::Padded => [2, 2, 1, 2],
            Material::Quilted => [4, 3, 2, 3],
            Material::Gambeson => [6, 5, 4, 5],
            Material::Kurbul => [4, 6, 5, 4],
            Material::Scale => [4, 8, 5, 5],
            Material::Mail => [2, 8, 7, 3],
            Material::Plate => [6, 11, 9, 5],
        }
    }
    fn rigidity(&self) -> Rigidity {
        match self {
            Material::Cloth => Rigidity::None,
            Material::Leather => Rigidity::None,
            Material::Padded => Rigidity::None,
            Material::Quilted => Rigidity::None,
            Material::Gambeson => Rigidity::Rigid,
            Material::Kurbul => Rigidity::Rigid,
            Material::Scale => Rigidity::Mail,
            Material::Mail => Rigidity::Mail,
            Material::Plate => Rigidity::Plate,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum Slot {
    Under,
    Base,
    Over,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Head,
    Arms,
    Torso,
    Legs,
    None,
}

impl TryFrom<u8> for Location {
    type Error = anyhow_serde::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        let location = random_range(1..=10);
        Ok(match value {
            1 => match location {
                1..=5 => Self::Skull,
                6..=8 => Self::Face,
                9..=10 => Self::Neck,
                _ => unreachable!(),
            },
            2..=3 => match location {
                1..=3 => Self::Shoulder,
                4..=6 => Self::UpperArm,
                7 => Self::Elbow,
                8..=9 => Self::Forearm,
                10 => Self::Hand,
                _ => unreachable!(),
            },
            4..=7 => match location {
                1..=4 => Self::Thorax,
                5..=7 => Self::Abdomen,
                8..=10 => Self::Pelvis,
                _ => unreachable!(),
            },
            8..=10 => match location {
                1..=4 => Self::Thigh,
                5 => Self::Knee,
                6..=8 => Self::Calf,
                9..=10 => Self::Foot,
                _ => unreachable!(),
            },
            _ => bail!("Missed!"),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Location {
    Skull,
    Face,
    Neck,
    Shoulder,
    UpperArm,
    Elbow,
    Forearm,
    Hand,
    Thorax,
    Abdomen,
    Pelvis,
    Thigh,
    Knee,
    Calf,
    Foot,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::Skull => "skull",
                Location::Face => "face",
                Location::Neck => "neck",
                Location::Shoulder => "shoulder",
                Location::UpperArm => "upper arm",
                Location::Elbow => "elbow",
                Location::Forearm => "forearm",
                Location::Hand => "hand",
                Location::Thorax => "thorax",
                Location::Abdomen => "abdomen",
                Location::Pelvis => "pelvis",
                Location::Thigh => "thigh",
                Location::Knee => "knee",
                Location::Calf => "calf",
                Location::Foot => "foot",
            }
        )
    }
}

impl Location {
    pub fn zone(&self) -> Zone {
        match self {
            Location::Skull | Location::Face | Location::Neck => Zone::Head,
            Location::Shoulder
            | Location::UpperArm
            | Location::Elbow
            | Location::Forearm
            | Location::Hand => Zone::Arms,
            Location::Thorax | Location::Abdomen | Location::Pelvis => Zone::Torso,
            Location::Thigh | Location::Knee | Location::Calf | Location::Foot => Zone::Legs,
        }
    }

    fn array_index(&self) -> usize {
        match self {
            Location::Skull => 0,
            Location::Face => 1,
            Location::Neck => 2,
            Location::Shoulder => 3,
            Location::UpperArm => 4,
            Location::Elbow => 5,
            Location::Forearm => 6,
            Location::Hand => 7,
            Location::Thorax => 8,
            Location::Abdomen => 9,
            Location::Pelvis => 10,
            Location::Thigh => 11,
            Location::Knee => 12,
            Location::Calf => 13,
            Location::Foot => 14,
        }
    }

    pub fn bloodloss(&self) -> u8 {
        match self {
            Location::Skull => 1,
            Location::Face => 2,
            Location::Neck => 3,
            Location::Shoulder => 2,
            Location::UpperArm => 1,
            Location::Elbow => 1,
            Location::Forearm => 1,
            Location::Hand => 0,
            Location::Thorax => 2,
            Location::Abdomen => 3,
            Location::Pelvis => 2,
            Location::Thigh => 2,
            Location::Knee => 1,
            Location::Calf => 1,
            Location::Foot => 0,
        }
    }

    pub fn shock(&self) -> i8 {
        match self {
            Location::Skull => 5,
            Location::Face => 4,
            Location::Neck => 5,
            Location::Shoulder => 3,
            Location::UpperArm => 1,
            Location::Elbow => 2,
            Location::Forearm => 1,
            Location::Hand => 2,
            Location::Thorax => 4,
            Location::Abdomen => 4,
            Location::Pelvis => 4,
            Location::Thigh => 3,
            Location::Knee => 2,
            Location::Calf => 1,
            Location::Foot => 2,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub material: Material,
    pub covers: Vec<Location>,
    pub description: String,
}

impl Armor {
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

impl Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.material, self.name)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ArmorSet(pub Vec<Armor>);

impl Display for ArmorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Items:\n{}", self.0.iter().format(", "))?;
        Ok(())
    }
}

impl ArmorSet {
    pub fn don(&mut self, piece: &Armor) -> Result<&mut Self> {
        let mut location_count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.0.iter().for_each(|p| {
            p.covers.iter().for_each(|c| {
                location_count[c.array_index()] += 1;
            })
        });
        let mut flag = false;
        piece.covers.iter().for_each(|c| {
            let i = location_count[c.array_index()];
            if i >= 5 {
                flag = true;
            }
        });
        if !flag {
            self.0.push(piece.clone());
        }
        Ok(self)
    }

    pub fn protection(&self, location: Location, aspect: weapons::Aspect) -> u8 {
        self.0.iter().fold(0, |mut acc, x| {
            x.covers.iter().for_each(|y| {
                if *y == location {
                    acc += x.material.av()[aspect.array_index()]
                }
            });
            acc
        })
    }

    pub fn calc_injury(&self, attack: Attack) -> Option<crate::Injury> {
        println!("Hits {}", attack.location);
        crate::Injury::new(
            attack.location,
            attack
                .impact
                .saturating_sub(self.protection(attack.location, attack.aspect)),
            attack.aspect,
            attack.location.bloodloss(),
        )
    }
}
