#![feature(try_find, hash_map_macro)]
mod armor;
mod armory;
mod beastiary;
mod date;
mod weapons;
use anyhow_serde::{Context, Result};
use clap::{Parser, Subcommand};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, hash::Hash, io::Write, path::PathBuf};
use strum::EnumIter;

use crate::beastiary::{Being, Character, Npc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SuccessResult {
    CriticalSuccess(u8),
    Success(u8),
    Fail(u8),
    CriticalFail(u8),
}

impl SuccessResult {
    fn numeric(&self) -> u8 {
        match self {
            SuccessResult::CriticalSuccess(_) => 3,
            SuccessResult::Success(_) => 2,
            SuccessResult::Fail(_) => 1,
            SuccessResult::CriticalFail(_) => 0,
        }
    }
    fn value_modifer(&self) -> i8 {
        self.numeric() as i8 - 2
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum OpposedResult {
    Victory,
    Tie(Tiebreak),
    Loss,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tiebreak {
    Lhs,
    Rhs,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, clap::ValueEnum)]
enum Skill {
    Acrobatics,
    Agriculture,
    Alchemy,
    Animalcraft,
    Archery,
    Astrology,
    Awareness,
    Brewing,
    Ceramics,
    Charm,
    Climbing,
    Command,
    Cookery,
    Dancing,
    Discourse,
    Dodge,
    Drawing,
    Embalming,
    Engineering,
    Fishing,
    Fletching,
    Folklore,
    Glassworking,
    Guile,
    Heraldry,
    Herblore,
    Hideworking,
    Initiative,
    Intrigue,
    Jewelcraft,
    Jumping,
    #[clap(skip)]
    Language(String),
    Law,
    Legerdemain,
    Lockcraft,
    Masonry,
    Mathematics,
    Melee,
    Mercantilism,
    Metalcraft,
    Milling,
    Mineralogy,
    #[clap(skip)]
    Musician(String),
    Perfumery,
    Physician,
    Piloting,
    Pvarism,
    Riding,
    #[clap(skip)]
    Ritual(String),
    Runecraft,
    #[clap(skip)]
    Script(String),
    Seamanship,
    #[clap(skip)]
    Shipwright(String),
    Shock,
    Slings,
    Spirit,
    Stealth,
    #[clap(skip)]
    Summoning(String),
    Survival,
    Swimming,
    #[clap(skip)]
    Talent(String),
    Tarotry,
    Textilecraft,
    Theatrics,
    Throwing,
    Timbercraft,
    Tracking,
    Trance,
    Weaponcraft,
    Woodworking,
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Testable for Skill {
    fn sb(&self, character: &impl Being) -> u8 {
        use Attribute::*;
        let attributes = match self {
            Skill::Acrobatics => [Agility, Endurance],
            Skill::Agriculture => [Perception, Will],
            Skill::Alchemy => [Aura, Perception],
            Skill::Animalcraft => [Empathy, Will],
            Skill::Archery => [Perception, Dexterity],
            Skill::Astrology => [Aura, Empathy],
            Skill::Awareness => [Perception, Will],
            Skill::Brewing => [Perception, Reasoning],
            Skill::Ceramics => [Dexterity, Perception],
            Skill::Charm => [Comeliness, Empathy],
            Skill::Climbing => [Agility, Dexterity],
            Skill::Command => [Will, Eloquence],
            Skill::Cookery => [Perception, Reasoning],
            Skill::Dancing => [Agility, Creativity],
            Skill::Discourse => [Reasoning, Eloquence],
            Skill::Dodge => [Agility, Perception],
            Skill::Drawing => [Dexterity, Creativity],
            Skill::Embalming => [Reasoning, Perception],
            Skill::Engineering => [Reasoning, Creativity],
            Skill::Fishing => [Perception, Will],
            Skill::Fletching => [Dexterity, Perception],
            Skill::Folklore => [Reasoning, Will],
            Skill::Glassworking => [Perception, Dexterity],
            Skill::Guile => [Empathy, Creativity],
            Skill::Heraldry => [Reasoning, Will],
            Skill::Herblore => [Perception, Reasoning],
            Skill::Hideworking => [Dexterity, Perception],
            Skill::Initiative => [Will, Reasoning],
            Skill::Intrigue => [Empathy, Reasoning],
            Skill::Jewelcraft => [Perception, Dexterity],
            Skill::Jumping => [Agility, Strength],
            Skill::Language(_) => [Eloquence, Reasoning],
            Skill::Law => [Reasoning, Will],
            Skill::Legerdemain => [Dexterity, Perception],
            Skill::Lockcraft => [Dexterity, Perception],
            Skill::Masonry => [Dexterity, Strength],
            Skill::Mathematics => [Reasoning, Creativity],
            Skill::Melee => [Dexterity, Agility],
            Skill::Mercantilism => [Reasoning, Perception],
            Skill::Metalcraft => [Dexterity, Strength],
            Skill::Milling => [Perception, Strength],
            Skill::Mineralogy => [Perception, Reasoning],
            Skill::Musician(_) => [Perception, Creativity],
            Skill::Perfumery => [Perception, Reasoning],
            Skill::Physician => [Reasoning, Perception],
            Skill::Piloting => [Reasoning, Perception],
            Skill::Pvarism => [Aura, Reasoning],
            Skill::Riding => [Empathy, Agility],
            Skill::Ritual(_) => [Will, Reasoning],
            Skill::Runecraft => [Aura, Empathy],
            Skill::Script(_) => [Reasoning, Perception],
            Skill::Seamanship => [Will, Perception],
            Skill::Shipwright(_) => [Reasoning, Creativity],
            Skill::Shock => [Strength, Endurance],
            Skill::Slings => [Dexterity, Perception],
            Skill::Spirit => [Aura, Will],
            Skill::Stealth => [Agility, Will],
            Skill::Summoning(_) => [Aura, Eloquence],
            Skill::Survival => [Will, Reasoning],
            Skill::Swimming => [Agility, Endurance],
            Skill::Talent(_) => [Aura, Will],
            Skill::Tarotry => [Aura, Empathy],
            Skill::Textilecraft => [Dexterity, Perception],
            Skill::Theatrics => [Creativity, Eloquence],
            Skill::Throwing => [Dexterity, Perception],
            Skill::Timbercraft => [Perception, Reasoning],
            Skill::Tracking => [Reasoning, Perception],
            Skill::Trance => [Aura, Creativity],
            Skill::Weaponcraft => [Dexterity, Strength],
            Skill::Woodworking => [Dexterity, Strength],
        };
        let mut attributes = attributes.iter().map(|k| character.attribute(*k));
        let sum: u8 = attributes.clone().sum();
        if attributes.next() > attributes.next() && !sum.is_multiple_of(2) {
            (sum / 2) + 1
        } else {
            sum / 2
        }
    }

    fn ml(&self, character: &impl Being) -> u8 {
        if let Some(mut sm) = character.skill(self.clone()) {
            let mut boosts = 0;
            if sm > 5 {
                boosts = sm - 5;
                sm = 5;
            }
            let mut ml = sm * self.sb(character);
            while boosts > 0 {
                ml = match ml {
                    0..40 => ml + 10,
                    40..45 => ml + 9,
                    45..50 => ml + 8,
                    50..60 => ml + 7,
                    60..70 => ml + 6,
                    70..80 => ml + 5,
                    80..100 => ml + 4,
                    _ => ml + 3,
                };
                boosts -= 1;
            }
            ml
        } else {
            0
        }
    }

    fn impaired_by(&self) -> Vec<armor::Zone> {
        use armor::Zone::*;
        match self {
            Self::Acrobatics
            | Self::Archery
            | Self::Climbing
            | Self::Dancing
            | Self::Jumping
            | Self::Melee
            | Self::Riding
            | Self::Slings
            | Self::Swimming
            | Self::Throwing => vec![Head, Arms, Torso, Legs],
            Self::Awareness => vec![Head],
            Self::Dodge
            | Self::Stealth
            | Self::Ceramics
            | Self::Drawing
            | Self::Fletching
            | Self::Glassworking
            | Self::Hideworking
            | Self::Jewelcraft
            | Self::Lockcraft
            | Self::Masonry
            | Self::Metalcraft
            | Self::Milling
            | Self::Musician(_)
            | Self::Textilecraft
            | Self::Weaponcraft
            | Self::Woodworking => vec![Head, Torso, Legs],
            Self::Legerdemain => vec![Head, Arms],
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone, Copy, EnumIter)]
enum Attribute {
    Strength,
    Endurance,
    Dexterity,
    Agility,
    Perception,
    Comeliness,
    Aura,
    Will,
    Reasoning,
    Creativity,
    Empathy,
    Eloquence,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Testable for Attribute {
    fn sb(&self, character: &impl Being) -> u8 {
        character.attribute(*self)
    }

    fn ml(&self, character: &impl Being) -> u8 {
        self.sb(character) * 5
    }

    fn impaired_by(&self) -> Vec<armor::Zone> {
        use armor::Zone::*;
        match self {
            Self::Agility => vec![Head, Torso, Legs],
            Self::Dexterity => vec![Head, Arms, Torso],
            Self::Strength => vec![Head, Arms, Torso, Legs],
            _ => Vec::new(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct SkillSet(HashMap<Skill, u8>);
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct AttributeSet(HashMap<Attribute, u8>);

impl Display for AttributeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Attributes:")?;
        for i in &self.0 {
            writeln!(f, "   {:.<14} {:>2}", format!("{}: ", i.0), i.1)?;
        }
        Ok(())
    }
}

impl Display for SkillSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Skills:")?;
        for i in &self.0 {
            writeln!(f, "   {:.<14} {:>2}", format!("{}: ", i.0), i.1)?;
        }
        Ok(())
    }
}

impl SkillSet {
    fn learn(&mut self, skill: Skill) {
        if let Some(level) = self.0.insert(skill.clone(), 1) {
            self.0.insert(skill, level + 1);
        }
    }

    fn unlearn(&mut self, skill: Skill) {
        if let Some(level) = self.0.get(&skill) {
            if *level == 1 {
                self.0.remove(&skill);
            } else {
                self.0.insert(skill, level - 1);
            }
        }
    }
}

trait Testable: Display {
    fn sb(&self, character: &impl Being) -> u8;
    fn ml(&self, character: &impl Being) -> u8;
    fn impaired_by(&self) -> Vec<armor::Zone>;
    fn index(&self, character: &impl Being) -> u8 {
        self.ml(character) / 10
    }
    fn eml(&self, modifier: i8, character: &impl Being) -> u8 {
        let mut block = false;
        let imparement = character.clone().injuries().iter().fold(0, |acc, x| {
            if self.impaired_by().contains(&x.location.zone()) {
                if x.shock > 3 {
                    block = true;
                    acc
                } else if x.shock > 1 {
                    acc + 10
                } else {
                    acc + 5
                }
            } else {
                acc
            }
        });
        if block {
            return 0;
        };
        let eml = self.ml(character) as i8 + modifier - imparement;
        match eml {
            ..5 => 5,
            5..95 => eml as u8,
            _ => 95,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum ShockState {
    Stunned,
    Incapacitated,
    Unconscious,
    Killed,
}

impl Display for ShockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Injury {
    location: armor::Location,
    shock: i8,
    aspect: weapons::Aspect,
    bleed: bool,
}

impl PartialEq for Injury {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.shock == other.shock
    }
}

impl Eq for Injury {}

impl PartialOrd for Injury {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Injury {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.shock < other.shock {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Injury {
    fn new(
        location: armor::Location,
        shock: u8,
        aspect: weapons::Aspect,
        bloodloss_potential: u8,
    ) -> Option<Injury> {
        match shock {
            0 => None,
            1..5 => Some(Injury {
                location,
                shock: 1,
                aspect,
                bleed: false,
            }),
            5..10 => Some(Injury {
                location,
                shock: 2,
                aspect,
                bleed: false,
            }),
            10..15 => Some(Injury {
                location,
                shock: 3,
                aspect,
                bleed: aspect == weapons::Aspect::Edge && bloodloss_potential >= 3,
            }),
            15..20 => Some(Injury {
                location,
                shock: 4,
                aspect,
                bleed: aspect == weapons::Aspect::Edge
                    || aspect == weapons::Aspect::Point && bloodloss_potential >= 2,
            }),
            _ => Some(Injury {
                location,
                shock: 5,
                aspect,
                bleed: aspect != weapons::Aspect::FireFrost && bloodloss_potential >= 1,
            }),
        }
    }

    fn compound(&mut self) -> &mut Self {
        if self.shock < 5 {
            self.shock += 1
        };
        self
    }
}

impl Display for Injury {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.shock {
            1 => write!(
                f,
                "{:?} M{}{}{}",
                self.location,
                self.shock,
                self.aspect,
                if self.bleed { " BLD" } else { "" }
            ),
            2..=3 => write!(
                f,
                "{:?} S{}{}{}",
                self.location,
                self.shock,
                self.aspect,
                if self.bleed { " BLD" } else { "" }
            ),
            4..=5 => write!(
                f,
                "{:?} G{}{}{}",
                self.location,
                self.shock,
                self.aspect,
                if self.bleed { " BLD" } else { "" }
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    CreateNpc {
        name: String,
    },

    FullHeal {
        name: String,
    },

    RangeAttack {
        attacker: String,
        target: String,
        range: u16,
    },

    MeleeAttack {
        attacker: String,
        target: String,
        defense_option: weapons::DefenseOption,
    },

    Test {
        skill: Skill,
        name: String,
    },

    OpposedTest {
        lhs_skill: Skill,
        lhs_character: String,
        rhs_skill: Skill,
        rhs_character: String,
    },

    ValueTest {
        skill: Skill,
        name: String,
    },

    Sheet {
        name: String,
    },

    Initiative {
        names: Vec<String>,
    },
}

fn main() -> Result<()> {
    let command = Cli::parse().command.context("No subcommand")?;
    match command {
        Commands::CreateNpc { name } => {
            let npc: Character = beastiary::Npc::load(format!("beastiary/{name}.toml"))?.into();
            npc.write_sheet(name)?;
        }
        Commands::FullHeal { name } => {
            let name = name.to_lowercase();
            let mut c = beastiary::Character::read_sheet(&name)?;
            c.injuries = Vec::new();
            c.shock_state = None;
            c.write_sheet(name)?;
        }
        Commands::RangeAttack {
            attacker,
            target,
            range,
        } => {
            let attacker = attacker.to_lowercase();
            let target = target.to_lowercase();
            let a = beastiary::Character::read_sheet(attacker)?;
            let mut t = beastiary::Character::read_sheet(&target)?;
            a.range_attack(&mut t, range);
            t.write_sheet(target)?;
        }
        Commands::MeleeAttack {
            attacker,
            target,
            defense_option,
        } => {
            let attacker = attacker.to_lowercase();
            let target = target.to_lowercase();
            let mut a = beastiary::Character::read_sheet(&attacker)?;
            let mut t = beastiary::Character::read_sheet(&target)?;
            a.melee_attack(0, &mut t, defense_option, 0);
            a.write_sheet(attacker)?;
            t.write_sheet(target)?;
        }
        Commands::Test { skill, name } => {
            let name = name.to_lowercase();
            let c = beastiary::Character::read_sheet(&name)?;
            println!("{:?}", c.success_test(skill, 0));
        }
        Commands::OpposedTest {
            lhs_skill,
            lhs_character,
            rhs_skill,
            rhs_character,
        } => {
            let lhs_character = lhs_character.to_lowercase();
            let rhs_character = rhs_character.to_lowercase();
            let l = beastiary::Character::read_sheet(&lhs_character)?;
            let r = beastiary::Character::read_sheet(&rhs_character)?;
            println!("{:?}", l.opposed_test(lhs_skill, 0, r.success_test(rhs_skill, 0)));
        }
        Commands::ValueTest { skill, name } => {
            let name = name.to_lowercase();
            let c = beastiary::Character::read_sheet(&name)?;
            println!("{:?}", c.value_test(skill, 0));
        }
        Commands::Sheet { name } => {
            let name = name.to_lowercase();
            let c = beastiary::Character::read_sheet(&name)?;
            println!("{c}");
        }
        Commands::Initiative { names } => {
            names
                .iter()
                .map(|name| {
                    let c = beastiary::Character::read_sheet(name).unwrap();
                    (
                        c.name().clone(),
                        c.ml(Skill::Initiative),
                        if let Some(shock) = c.shock_state {
                            format!("{shock:?}")
                        } else {
                            String::new()
                        },
                    )
                })
                .sorted_by_key(|x| x.1)
                .rev()
                .for_each(|x| println!("{:<10} {} {}", x.0, x.1, x.2));
        }
    }
    Ok(())
}
