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

use crate::{
    Attribute, AttributeSet, Injury, OpposedResult, ShockState, Skill, SkillSet, SuccessResult,
    Testable, Tiebreak, armor,
    date::Date,
    weapons::{self, Attack},
};

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

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}",
            self.name(),
            if let Some(state) = self.shock_state {
                format!(" [{state}]")
            } else {
                "".to_string()
            }
        )?;
        writeln!(f, "\nAttributes:")?;
        for i in &self.attributes.0 {
            writeln!(
                f,
                "   {:.<14} {:>2} {:>2}",
                format!("{}: ", i.0),
                i.1,
                i.0.eml(0, self)
            )?;
        }
        writeln!(f, "\nSkills:")?;
        for i in &self.skills.0 {
            writeln!(
                f,
                "   {:.<14} {:>2} {:>2}",
                format!("{}: ", i.0),
                i.1,
                i.0.eml(0, self)
            )?;
        }
        writeln!(f, "\n{}", self.armor)?;
        writeln!(f, "            B  E  P  F")?;
        for i in armor::Location::iter() {
            writeln!(
                f,
                "{:>10} {:>2} {:>2} {:>2} {:>2}  {}",
                format!("{i}:"),
                self.armor.protection(i, weapons::Aspect::Blunt),
                self.armor.protection(i, weapons::Aspect::Edge),
                self.armor.protection(i, weapons::Aspect::Point),
                self.armor.protection(i, weapons::Aspect::FireFrost),
                self.injuries
                    .iter()
                    .filter(|x| x.location == i)
                    .format(", ")
            )?;
        }
        Ok(())
    }
}

impl Being for Character {
    fn name(&self) -> &String {
        &self.name
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

    fn attribute(&self, attribute: Attribute) -> u8 {
        *self
            .attributes
            .0
            .get(&attribute)
            .expect("Attribute does not exist!")
    }

    fn skill(&self, skill: Skill) -> Option<u8> {
        self.skills.0.get(&skill).copied()
    }
}

impl Character {
    pub fn write_sheet<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let buf = rmp_serde::to_vec(self)?;
        File::create(path)?.write_all(&buf)?;
        Ok(())
    }

    pub fn read_sheet<P>(path: P) -> Result<Character>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Character = rmp_serde::from_slice(&buf)?;
        Ok(o)
    }

    pub fn read_sheet_toml<P>(path: P) -> Result<Character>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Character = toml::from_slice(&buf)?;
        Ok(o)
    }
}

pub trait Being: Clone {
    fn name(&self) -> &String;
    fn attribute(&self, attribute: Attribute) -> u8;
    fn skill(&self, skill: Skill) -> Option<u8>;
    fn mh_weapon(&self) -> &Option<weapons::MeleeWeapon>;
    fn oh_weapon(&self) -> &Option<weapons::MeleeWeapon>;
    fn rg_weapon(&self) -> &Option<weapons::RangeWeapon>;
    fn ammo(&self) -> Option<weapons::Projectile>;
    fn armor(&self) -> &armor::ArmorSet;
    fn injuries(&mut self) -> &mut Vec<Injury>;
    fn shock_state(&mut self) -> &mut Option<ShockState>;

    fn success_test(&self, test: impl Testable, modifier: i8) -> SuccessResult {
        let eml = test.eml(modifier, self);
        println!("{} tests {test} with EML of {eml}", self.name());
        let roll = random_range(1..=100);
        println!("{} rolled {roll}", self.name());
        let result = if roll <= eml {
            if roll.is_multiple_of(5) {
                SuccessResult::CriticalSuccess(roll)
            } else {
                SuccessResult::Success(roll)
            }
        } else if roll.is_multiple_of(5) {
            SuccessResult::CriticalFail(roll)
        } else {
            SuccessResult::Fail(roll)
        };
        println!("{result:?}");
        result
    }

    fn ml(&self, test: impl Testable) -> u8 {
        test.ml(self)
    }
    fn eml(&self, test: impl Testable) -> u8 {
        test.eml(0, self)
    }

    fn opposed_test(
        &self,
        test: impl Testable,
        modifier: i8,
        rhs: SuccessResult,
    ) -> (OpposedResult, u8) {
        let lhs = self.success_test(test, modifier);
        match (&lhs, &rhs) {
            (SuccessResult::CriticalSuccess(l), SuccessResult::CriticalSuccess(r))
            | (SuccessResult::Success(l), SuccessResult::Success(r)) => (
                OpposedResult::Tie(if l >= r { Tiebreak::Lhs } else { Tiebreak::Rhs }),
                0,
            ),
            (SuccessResult::CriticalSuccess(_), _) => {
                (OpposedResult::Victory, lhs.numeric() - rhs.numeric())
            }
            (_, SuccessResult::CriticalSuccess(_)) => {
                (OpposedResult::Loss, rhs.numeric() - lhs.numeric())
            }
            (SuccessResult::Success(_), _) => {
                (OpposedResult::Victory, lhs.numeric() - rhs.numeric())
            }
            (_, SuccessResult::Success(_)) => (OpposedResult::Loss, rhs.numeric() - lhs.numeric()),
            _ => (OpposedResult::Tie(Tiebreak::None), 0),
        }
    }

    fn value_test(&self, test: impl Testable, modifier: i8) -> i8 {
        let index = test.index(self);
        index as i8 + self.success_test(test, modifier).value_modifer()
    }

    fn range_attack(&self, target: &mut impl Being, range: u16) -> Option<Injury> {
        let w = self.rg_weapon().clone()?;
        println!(
            "{} performs a ranged attack against {} using {}",
            self.name(),
            target.name(),
            w.name
        );
        let r = w.calc_range(range);
        let stars = match self.success_test(Skill::Archery, r.test_mod()?) {
            SuccessResult::CriticalSuccess(_) => 2,
            SuccessResult::Success(_) => 1,
            _ => return None,
        };
        let attack = Attack::new(
            random_range(1..=r.zone_die()?).try_into().ok()?,
            self.ammo()?
                .calc_impact(r.impact_mod()? + (w.impact_mod * stars) as i8),
            weapons::Aspect::Point,
        );
        let injury = target.armor().calc_injury(attack);
        if let Some(i) = injury {
            target.injure(i);
        }
        injury
    }

    fn calc_heft_penalty(&self) -> Option<i8> {
        let strength = self.attribute(Attribute::Strength) as i8;
        let heft = self.mh_weapon().clone()?.heft as i8
            + match self.oh_weapon() {
                Some(_) => -5,
                None => 0,
            };
        Some(if strength < heft {
            (heft - strength) * -5
        } else {
            0
        })
    }

    fn melee_attack(
        &mut self,
        atk_mod: i8,
        target: &mut impl Being,
        defense: weapons::DefenseOption,
        def_mod: i8,
    ) -> Option<Injury> {
        let atk_w = self.mh_weapon().clone()?;
        println!(
            "{} performs a melee attack against {} using {}",
            self.name(),
            target.name(),
            atk_w.name
        );
        let def_w = target.mh_weapon().clone()?;
        let reach_diff = (atk_w.length as i8 - def_w.length as i8) * 5;
        let atk_r = if reach_diff > 0 && atk_w.thrusting {
            reach_diff
        } else {
            0
        };
        let def_r = if reach_diff < 0 && def_w.thrusting {
            reach_diff
        } else {
            0
        };
        let t = match defense {
            weapons::DefenseOption::Block => {
                target.success_test(Skill::Melee, def_mod + target.calc_heft_penalty()?)
            }
            weapons::DefenseOption::Dodge => {
                target.success_test(Skill::Dodge, def_mod + target.calc_heft_penalty()?)
            }
            weapons::DefenseOption::Counterstrike => {
                target.success_test(Skill::Melee, def_mod + target.calc_heft_penalty()? - def_r)
            }
        };
        let (res, stars) =
            self.opposed_test(Skill::Melee, atk_mod + self.calc_heft_penalty()? + atk_r, t);
        let atk = Attack::new(
            random_range(1..=atk_w.zone_die).try_into().ok()?,
            random_range(1..=atk_w.impact_die) + stars * atk_w.impact_mod,
            atk_w.aspect,
        );
        let def = Attack::new(
            random_range(1..=def_w.zone_die).try_into().ok()?,
            random_range(1..=def_w.impact_die) + stars * def_w.impact_mod,
            def_w.aspect,
        );
        match res {
            OpposedResult::Victory => {
                let injury = target.armor().calc_injury(atk);
                target.injure(injury?);
                injury
            }
            OpposedResult::Tie(winner) => match defense {
                weapons::DefenseOption::Block => None,
                weapons::DefenseOption::Dodge => {
                    if winner == Tiebreak::Lhs {
                        let injury = target.armor().calc_injury(atk);
                        target.injure(injury?);
                        injury
                    } else {
                        None
                    }
                }
                weapons::DefenseOption::Counterstrike => {
                    if winner != Tiebreak::None {
                        let injury = self.armor().calc_injury(def);
                        self.injure(injury?);
                        injury
                    } else {
                        None
                    }
                }
            },
            OpposedResult::Loss => None,
        }
    }

    fn injure(&mut self, injury: Injury) -> &mut Self {
        println!("{} takes {injury} injury!", self.name());
        let existing = self
            .injuries()
            .iter()
            .filter(|i| i.location == injury.location)
            .fold(0, |mut acc, i| {
                acc += i.shock;
                acc
            });
        let injuries = self.injuries();
        injuries.push(injury);
        let roll = random_range(1..=10);
        let to_shock = if existing > 0 && roll <= injury.shock + existing {
            let (mut index, mut highest) = (0, injuries.last().unwrap());
            for i in injuries
                .iter()
                .enumerate()
                .filter(|x| x.1.location == injury.location)
            {
                if i.1 > highest {
                    (index, highest) = i
                }
            }
            *injuries[index].compound()
        } else {
            injury
        };

        self.shock(to_shock);
        self
    }

    fn shock(&mut self, injury: Injury) -> Option<ShockState> {
        /* TODO: implement fatigue */
        let i: i8 = injury.location.shock()
            + injury.shock
            + match self.success_test(Skill::Shock, 0) {
                SuccessResult::CriticalSuccess(_) => -1,
                SuccessResult::Success(_) => 0,
                SuccessResult::Fail(_) => 1,
                SuccessResult::CriticalFail(_) => 2,
            };
        let shock = match i {
            ..=6 => None,
            7 => Some(ShockState::Stunned),
            8 => Some(ShockState::Incapacitated),
            9 => Some(ShockState::Unconscious),
            _ => Some(ShockState::Killed),
        };
        if let Some(s) = shock {
            println!("{} is {s}!", self.name())
        }
        let s = self.shock_state();
        *s = shock;
        shock
    }
}
