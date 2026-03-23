use crate::being;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, ops::Deref};
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SuccessResult {
    CriticalSuccess(u8),
    Success(u8),
    Fail(u8),
    CriticalFail(u8),
}

impl SuccessResult {
    pub fn numeric(self) -> u8 {
        match self {
            SuccessResult::CriticalSuccess(_) => 3,
            SuccessResult::Success(_) => 2,
            SuccessResult::Fail(_) => 1,
            SuccessResult::CriticalFail(_) => 0,
        }
    }
    pub fn value_modifer(self) -> i8 {
        self.numeric().cast_signed() - 2
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpposedResult {
    Victory,
    Tie(Tiebreak),
    Loss,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tiebreak {
    Lhs,
    Rhs,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, clap::ValueEnum)]
pub enum Skill {
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
        match self {
            Skill::Language(s)
            | Skill::Musician(s)
            | Skill::Ritual(s)
            | Skill::Script(s)
            | Skill::Shipwright(s)
            | Skill::Summoning(s)
            | Skill::Talent(s) => write!(f, "{s}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl Testable for Skill {
    fn sb(&self, character: &being::Being) -> u8 {
        use Attribute::{
            Agility, Aura, Comeliness, Creativity, Dexterity, Eloquence, Empathy, Endurance,
            Perception, Reasoning, Strength, Will,
        };
        let attributes = match self {
            Skill::Alchemy => [Aura, Perception],
            Skill::Animalcraft => [Empathy, Will],
            Skill::Archery | Skill::Glassworking | Skill::Jewelcraft => [Perception, Dexterity],
            Skill::Astrology | Skill::Runecraft | Skill::Tarotry => [Aura, Empathy],
            Skill::Brewing
            | Skill::Cookery
            | Skill::Herblore
            | Skill::Mineralogy
            | Skill::Perfumery
            | Skill::Timbercraft => [Perception, Reasoning],
            Skill::Ceramics
            | Skill::Fletching
            | Skill::Hideworking
            | Skill::Legerdemain
            | Skill::Lockcraft
            | Skill::Slings
            | Skill::Textilecraft
            | Skill::Throwing => [Dexterity, Perception],
            Skill::Charm => [Comeliness, Empathy],
            Skill::Climbing => [Agility, Dexterity],
            Skill::Command => [Will, Eloquence],
            Skill::Dancing => [Agility, Creativity],
            Skill::Discourse => [Reasoning, Eloquence],
            Skill::Dodge => [Agility, Perception],
            Skill::Drawing => [Dexterity, Creativity],
            Skill::Embalming
            | Skill::Mercantilism
            | Skill::Physician
            | Skill::Piloting
            | Skill::Script(_)
            | Skill::Tracking => [Reasoning, Perception],
            Skill::Engineering | Skill::Mathematics | Skill::Shipwright(_) => {
                [Reasoning, Creativity]
            }
            Skill::Agriculture | Skill::Awareness | Skill::Fishing => [Perception, Will],
            Skill::Folklore | Skill::Heraldry | Skill::Law => [Reasoning, Will],
            Skill::Guile => [Empathy, Creativity],
            Skill::Initiative | Skill::Ritual(_) | Skill::Survival => [Will, Reasoning],
            Skill::Intrigue => [Empathy, Reasoning],
            Skill::Jumping => [Agility, Strength],
            Skill::Language(_) => [Eloquence, Reasoning],
            Skill::Masonry | Skill::Metalcraft | Skill::Weaponcraft | Skill::Woodworking => {
                [Dexterity, Strength]
            }
            Skill::Melee => [Dexterity, Agility],
            Skill::Milling => [Perception, Strength],
            Skill::Musician(_) => [Perception, Creativity],
            Skill::Pvarism => [Aura, Reasoning],
            Skill::Riding => [Empathy, Agility],
            Skill::Seamanship => [Will, Perception],
            Skill::Shock => [Strength, Endurance],
            Skill::Spirit | Skill::Talent(_) => [Aura, Will],
            Skill::Stealth => [Agility, Will],
            Skill::Summoning(_) => [Aura, Eloquence],
            Skill::Acrobatics | Skill::Swimming => [Agility, Endurance],
            Skill::Theatrics => [Creativity, Eloquence],
            Skill::Trance => [Aura, Creativity],
        };
        let mut attributes = attributes.iter().map(|k| character.attribute(*k));
        let sum: u8 = attributes.clone().sum();
        if attributes.next() > attributes.next() && !sum.is_multiple_of(2) {
            (sum / 2) + 1
        } else {
            sum / 2
        }
    }

    fn ml(&self, character: &being::Being) -> u8 {
        if let Some(mut sm) = character.skill(self) {
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

    fn impaired_by(&self) -> Vec<being::Zone> {
        use being::Zone::{Arm, Head, Leg, Torso};
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
            | Self::Throwing => vec![Head, Arm, Torso, Leg],
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
            | Self::Woodworking => vec![Head, Torso, Leg],
            Self::Legerdemain => vec![Head, Arm],
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone, Copy, EnumIter)]
pub enum Attribute {
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
    fn sb(&self, character: &being::Being) -> u8 {
        character.attribute(*self)
    }

    fn ml(&self, character: &being::Being) -> u8 {
        self.sb(character) * 5
    }

    fn impaired_by(&self) -> Vec<being::Zone> {
        use being::Zone::{Arm, Head, Leg, Torso};
        match self {
            Self::Agility => vec![Head, Torso, Leg],
            Self::Dexterity => vec![Head, Arm, Torso],
            Self::Strength => vec![Head, Arm, Torso, Leg],
            _ => Vec::new(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SkillSet(HashMap<Skill, u8>);
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AttributeSet(HashMap<Attribute, u8>);

impl Deref for AttributeSet {
    type Target = HashMap<Attribute, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for SkillSet {
    type Target = HashMap<Skill, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

pub trait Testable: Display {
    fn sb(&self, character: &being::Being) -> u8;
    fn ml(&self, character: &being::Being) -> u8;
    fn impaired_by(&self) -> Vec<being::Zone>;
    fn index(&self, character: &being::Being) -> u8 {
        self.ml(character) / 10
    }
    fn eml(&self, modifier: i8, character: &being::Being) -> u8 {
        let mut block = false;
        let imparement = character.injuries().iter().fold(0, |acc, x| {
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
        }
        let eml = self.ml(character).cast_signed() + modifier - imparement;
        match eml {
            ..5 => 5,
            5..95 => eml.cast_unsigned(),
            _ => 95,
        }
    }
}
