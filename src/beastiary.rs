use crate::Skill;
use crate::core::Attribute;
use crate::being::ShockState;
use crate::being::Injury;
use crate::core::SkillSet;
use anyhow_serde::Result;
use itertools::Itertools;
use rand::random_range;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{Read, Write},
};
use strum::IntoEnumIterator;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
enum Folk {
    #[default]
    Human,
    Kuzhai,
    Aenarin,
    Sinai,
    Sidhe,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Npc {
    pub name: String,
    pub skills: SkillSet,
    pub action_base: u8,
    pub mh_weapon: Option<weapons::MeleeWeapon>,
    pub oh_weapon: Option<weapons::MeleeWeapon>,
    pub rg_weapon: Option<weapons::RangeWeapon>,
    pub ammo: Option<weapons::Projectile>,
    pub armor: armor::ArmorSet,
    pub injuries: Vec<Injury>,
    pub shock_state: Option<ShockState>,
}

impl From<Npc> for Character {
    fn from(val: Npc) -> Self {
        let mut val = val;
        let mut map = HashMap::new();
        Attribute::iter().for_each(|x| {
            map.insert(x, 10);
        });
        [Skill::Initiative, Skill::Shock, Skill::Melee, Skill::Dodge]
            .iter()
            .for_each(|x| val.set_skill(x.clone(), val.action_base));
        Character {
            name: val.name,
            folk: Folk::default(),
            gender: Gender::default(),
            birthdate: Date::default(),
            attributes: AttributeSet(map),
            skills: val.skills,
            mh_weapon: val.mh_weapon,
            oh_weapon: val.oh_weapon,
            rg_weapon: val.rg_weapon,
            ammo: val.ammo,
            armor: val.armor,
            injuries: val.injuries,
            shock_state: val.shock_state,
        }
    }
}

impl Npc {
    pub fn set_skill(&mut self, skill: Skill, value: u8) {
        self.skills.0.insert(skill, value);
    }
    pub fn load<P>(path: P) -> Result<Npc>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Npc = toml::from_slice(&buf)?;
        Ok(o)
    }

    pub fn write_sheet<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let buf = toml::to_string_pretty(self)?;
        File::create(path)?.write_all(buf.as_bytes())?;
        Ok(())
    }
}

impl Being for Npc {
    fn name(&self) -> &String {
        &self.name
    }

    fn attribute(&self, _: Attribute) -> u8 {
        10
    }

    fn mh_weapon(&self) -> &Option<weapons::MeleeWeapon> {
        &self.mh_weapon
    }

    fn oh_weapon(&self) -> &Option<weapons::MeleeWeapon> {
        &self.oh_weapon
    }

    fn rg_weapon(&self) -> &Option<weapons::RangeWeapon> {
        &self.rg_weapon
    }

    fn ammo(&self) -> Option<weapons::Projectile> {
        self.ammo
    }

    fn armor(&self) -> &armor::ArmorSet {
        &self.armor
    }

    fn injuries(&mut self) -> &mut Vec<Injury> {
        &mut self.injuries
    }

    fn shock_state(&mut self) -> &mut Option<ShockState> {
        &mut self.shock_state
    }

    fn skill(&self, skill: Skill) -> Option<u8> {
        if [Skill::Initiative, Skill::Shock, Skill::Melee, Skill::Dodge].contains(&skill) {
            Some(self.action_base)
        } else {
            self.skills.0.get(&skill).copied()
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    name: String,
    folk: Folk,
    gender: Gender,
    birthdate: Date,
    attributes: AttributeSet,
    skills: SkillSet,
    mh_weapon: Option<weapons::MeleeWeapon>,
    oh_weapon: Option<weapons::MeleeWeapon>,
    rg_weapon: Option<weapons::RangeWeapon>,
    ammo: Option<weapons::Projectile>,
    armor: armor::ArmorSet,
    pub injuries: Vec<Injury>,
    pub shock_state: Option<ShockState>,
}
