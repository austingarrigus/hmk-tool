use crate::core::AttributeSet;
use crate::core::OpposedResult;
use crate::core::Skill;
use crate::core::SkillSet;
use crate::core::SuccessResult;
use crate::core::Testable;
use crate::core::Tiebreak;
use crate::item::Inventory;
use crate::item::Item;
use crate::item::Mode;
use crate::{core::Attribute, item};
use anyhow_serde::Result;
use itertools::Itertools;
use rand::random_range;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
};
use strum::EnumIter;

#[derive(Default, Hash, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BodyType(Vec<(u8, Vec<LocationEntry>)>);

#[derive(Hash, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct LocationEntry {
    die_limit: u8,
    location: Location,
    shock: i8,
}

impl LocationEntry {
    fn new(die: u8, location: Location, shock: i8) -> Self {
        Self {
            die_limit: die,
            location,
            shock,
        }
    }
}

impl BodyType {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Self = yaml_serde::from_slice(&buf)?;
        Ok(o)
    }

    pub fn write_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let buf = yaml_serde::to_string(self)?;
        File::create(path)?.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn roll(&self, zone_die: u8, aim: u8) -> Option<Location> {
        let z = random_range(1..=zone_die) + aim;
        let l = random_range(1..=10);
        eprintln!("Zone roll: {z}\tLocation roll: {l}");
        Some(
            self.0
                .iter()
                .find(|x| z <= x.0)?
                .1
                .iter()
                .find(|x| l <= x.die_limit)?
                .location,
        )
    }

    fn flatten(&self) -> impl Iterator<Item = &LocationEntry> {
        self.0.iter().flat_map(|x| x.1.iter())
    }

    fn select(&self, location: Location) -> Option<&LocationEntry> {
        self.flatten().find(|x| x.location == location)
    }

    pub fn locations(&self) -> Vec<Location> {
        self.flatten().map(|x| x.location).collect()
    }

    pub fn shock(&self, injury: Injury) -> Option<i8> {
        Some(self.select(injury.location)?.shock + injury.shock)
    }
}

#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Chance {
    None,
    Low,
    Mid,
    High,
}

impl Chance {
    pub fn circle(&self) -> &str {
        match self {
            Chance::None => " ",
            Chance::Low => "○",
            Chance::Mid => "◐",
            Chance::High => "●",
        }
    }
    pub fn triangle(&self) -> &str {
        match self {
            Chance::None => " ",
            Chance::Low => "▽",
            Chance::Mid => "⧨",
            Chance::High => "▼",
        }
    }
}

#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Location {
    Skull,
    Face,
    Abdomen,
    Knee,
    LowerArm,
    Calf,
    Ear,
    LowerLeg,
    Elbow,
    Neck,
    Pelvis,
    FrontFoot,
    Quarter,
    FrontLeg,
    Shoulder,
    Flank,
    Thigh,
    Forearm,
    Tail,
    Foot,
    Trunk,
    Hand,
    Thorax,
    HindLeg,
    UpperArm,
    HindFoot,
    UpperLeg,
    Horn,
    Wing,
    Head,
    Arm,
    Torso,
    Leg,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Skull => "skull",
                Self::Face => "face",
                Self::Neck => "neck",
                Self::Shoulder => "shoulder",
                Self::UpperArm => "upper arm",
                Self::Elbow => "elbow",
                Self::Forearm => "forearm",
                Self::Hand => "hand",
                Self::Thorax => "thorax",
                Self::Abdomen => "abdomen",
                Self::Pelvis => "pelvis",
                Self::Thigh => "thigh",
                Self::Knee => "knee",
                Self::Calf => "calf",
                Self::Foot => "foot",
                Self::LowerArm => "lower arm",
                Self::Ear => "ear",
                Self::LowerLeg => "lower leg",
                Self::FrontFoot => "front foot",
                Self::Quarter => "quarter",
                Self::FrontLeg => "front leg",
                Self::Flank => "flank",
                Self::Tail => "tail",
                Self::Trunk => "trunk",
                Self::HindLeg => "hind leg",
                Self::HindFoot => "hind foot",
                Self::UpperLeg => "upper leg",
                Self::Horn => "horn",
                Self::Wing => "wing",
                Self::Head => "head",
                Self::Arm => "arm",
                Self::Torso => "torso",
                Self::Leg => "leg",
            }
        )
    }
}

impl Location {
    pub fn bleed(self) -> Chance {
        match self {
            Self::Tail | Self::Foot | Self::Hand | Self::Horn => Chance::None,
            Self::Wing
            | Self::Skull
            | Self::Knee
            | Self::LowerArm
            | Self::Ear
            | Self::Calf
            | Self::UpperArm
            | Self::Arm
            | Self::HindFoot
            | Self::Forearm
            | Self::LowerLeg
            | Self::Trunk
            | Self::Leg
            | Self::Elbow
            | Self::FrontLeg
            | Self::FrontFoot => Chance::Low,
            Self::Face
            | Self::Pelvis
            | Self::Quarter
            | Self::Shoulder
            | Self::Flank
            | Self::Thigh
            | Self::Thorax
            | Self::HindLeg
            | Self::UpperLeg => Chance::Mid,
            Self::Head | Self::Neck | Self::Abdomen | Self::Torso => Chance::High,
        }
    }

    pub fn amputate(self) -> Chance {
        match self {
            Location::Skull
            | Location::Face
            | Location::Abdomen
            | Location::Pelvis
            | Location::Quarter
            | Location::Shoulder
            | Location::Thorax
            | Location::Horn
            | Location::Flank
            | Location::Head
            | Location::Torso => Chance::None,
            Location::Neck | Location::Thigh | Location::UpperLeg | Location::Wing => Chance::Low,
            Location::Knee
            | Location::LowerArm
            | Location::Calf
            | Location::Ear
            | Location::LowerLeg
            | Location::Elbow
            | Location::FrontFoot
            | Location::FrontLeg
            | Location::Forearm
            | Location::Foot
            | Location::Trunk
            | Location::HindLeg
            | Location::UpperArm
            | Location::HindFoot
            | Location::Arm
            | Location::Leg => Chance::Mid,
            Location::Tail | Location::Hand => Chance::High,
        }
    }

    pub fn zone(self) -> Zone {
        match self {
            Self::Skull
            | Self::Face
            | Self::Neck
            | Self::Horn
            | Self::Ear
            | Self::Trunk
            | Self::Head => Zone::Head,
            Self::Shoulder
            | Self::UpperArm
            | Self::Elbow
            | Self::Forearm
            | Self::Hand
            | Self::LowerArm
            | Self::FrontFoot
            | Self::FrontLeg
            | Self::Wing
            | Self::Arm => Zone::Arm,
            Self::Thorax | Self::Abdomen | Self::Pelvis | Self::Flank | Self::Torso => Zone::Torso,
            Self::Thigh
            | Self::Knee
            | Self::Calf
            | Self::Foot
            | Self::LowerLeg
            | Self::Quarter
            | Self::Tail
            | Self::HindLeg
            | Self::HindFoot
            | Self::UpperLeg
            | Self::Leg => Zone::Leg,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Head,
    Arm,
    Torso,
    Leg,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ShockState {
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
pub struct Injury {
    pub location: Location,
    pub shock: i8,
    pub aspect: item::Aspect,
    pub bleed: bool,
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
    pub fn new(
        location: Location,
        shock: i8,
        aspect: item::Aspect,
        bleed: Chance,
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
                bleed: aspect == item::Aspect::Edge && bleed >= Chance::High,
            }),
            15..20 => Some(Injury {
                location,
                shock: 4,
                aspect,
                bleed: aspect == item::Aspect::Edge
                    || aspect == item::Aspect::Point && bleed >= Chance::Mid,
            }),
            _ => Some(Injury {
                location,
                shock: 5,
                aspect,
                bleed: aspect != item::Aspect::FireFrost && bleed >= Chance::Low,
            }),
        }
    }

    fn compound(&mut self) -> &mut Self {
        if self.shock < 5 {
            self.shock += 1;
        }
        self
    }
}

impl Display for Injury {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            match self.shock {
                1 => "M",
                2..=3 => "S",
                4..=5 => "G",
                _ => unreachable!(),
            },
            self.shock,
            self.aspect,
            if self.bleed { " BLD" } else { "" }
        )
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Being {
    pub name: String,
    pub attributes: AttributeSet,
    pub skills: SkillSet,
    pub prime_hand: Option<Item>,
    pub off_hand: Option<Item>,
    pub inventory: Inventory,
    pub body: BodyType,
    pub injuries: Vec<Injury>,
    pub shock: Option<ShockState>,
}

impl Display for Being {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}",
            self.name(),
            if let Some(state) = self.shock {
                format!(" [{state}]")
            } else {
                String::new()
            }
        )?;
        writeln!(f, "\nAttributes:")?;
        for i in self.attributes.iter() {
            writeln!(
                f,
                "   {:.<14} {:>2} {:>2}",
                format!("{}: ", i.0),
                i.1,
                i.0.eml(0, self)
            )?;
        }
        writeln!(f, "\nSkills:")?;
        for i in self.skills.iter() {
            writeln!(
                f,
                "   {:.<14} {:>2} {:>2}",
                format!("{}: ", i.0),
                i.1,
                i.0.eml(0, self)
            )?;
        }
        writeln!(f, "\nInventory:")?;
        writeln!(f, "{}", self.inventory.iter().format(", "))?;
        writeln!(f, "\n            B  E  P  F  Shk")?;
        for i in self.body.locations() {
            writeln!(
                f,
                "{:>10} {:>2} {:>2} {:>2} {:>2}  {}{}{}  {}",
                format!("{i}:"),
                self.inventory.protection(i, item::Aspect::Blunt),
                self.inventory.protection(i, item::Aspect::Edge),
                self.inventory.protection(i, item::Aspect::Point),
                self.inventory.protection(i, item::Aspect::FireFrost),
                i.bleed().circle(),
                self.body.select(i).unwrap().shock,
                i.amputate().triangle(),
                self.injuries
                    .iter()
                    .filter(|x| x.location == i)
                    .format(", ")
            )?;
        }
        Ok(())
    }
}

impl Being {
    pub fn modes(&self) -> Vec<Mode> {
        let mut attack_modes = Vec::<Mode>::new();
        if let Some(v) = &self.prime_hand {
            attack_modes.append(&mut v.modes());
        }
        if let Some(v) = &self.off_hand {
            attack_modes.append(&mut v.modes());
        }
        attack_modes
    }
    pub fn attribute(&self, attribute: Attribute) -> u8 {
        *self.attributes.get(&attribute).unwrap_or(&10)
    }
    pub fn skill(&self, skill: &Skill) -> Option<u8> {
        if let Some(o) = self.skills.get(skill).copied() {
            Some(o)
        } else {
            match skill {
                Skill::Initiative | Skill::Shock | Skill::Melee | Skill::Dodge => Some(5),
                _ => None,
            }
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn injuries(&self) -> &Vec<Injury> {
        &self.injuries
    }
    pub fn shock_state(&mut self) -> &mut Option<ShockState> {
        &mut self.shock
    }

    pub fn write_sheet<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let buf = toml::to_string_pretty(self)?;
        File::create(path)?.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn read_sheet<P>(path: P) -> Result<Being>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Being = toml::from_slice(&buf)?;
        Ok(o)
    }

    pub fn read_sheet_toml<P>(path: P) -> Result<Being>
    where
        P: AsRef<std::path::Path>,
    {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let o: Being = toml::from_slice(&buf)?;
        Ok(o)
    }

    pub fn success_test(&self, test: &impl Testable, modifier: i32) -> SuccessResult {
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

    pub fn ml(&self, test: &impl Testable) -> u8 {
        test.ml(self)
    }

    pub fn eml(&self, test: &impl Testable) -> u32 {
        test.eml(0, self)
    }

    pub fn opposed_test(
        &self,
        test: &impl Testable,
        modifier: i8,
        rhs: SuccessResult,
    ) -> (OpposedResult, u8) {
        let lhs = self.success_test(test, modifier as i32);
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

    pub fn value_test(&self, test: &impl Testable, modifier: i8) -> i8 {
        let index = test.index(self);
        index.cast_signed() + self.success_test(test, modifier as i32).value_modifer()
    }

    pub fn equip(&mut self, item: Item) {
        self.inventory.equip(item);
    }

    fn hands_full(&self) -> u8 {
        [&self.prime_hand, &self.off_hand]
            .iter()
            .fold(0, |acc, x| if x.is_some() { acc + 1 } else { acc })
    }

    pub fn heft_mod(&self, mode: Mode) -> Option<i8> {
        let strength = self.attribute(Attribute::Strength).cast_signed();
        let heft = mode.heft().cast_signed() + if self.hands_full() > 1 { -5 } else { 0 };
        Some(if strength < heft {
            (heft - strength) * -5
        } else {
            0
        })
    }

    pub fn str_mod(&self) -> i8 {
        let str = self.attribute(Attribute::Strength).cast_signed();
        match str {
            ..=0 => unreachable!(),
            1..=5 => str - 11,
            6.. => str / 2 - 5,
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
        self.injuries.push(injury);
        let roll = random_range(1..=10);
        let to_shock = if existing > 0 && roll <= injury.shock + existing {
            let (mut index, mut highest) = (0, self.injuries.last().unwrap());
            for i in self
                .injuries
                .iter()
                .enumerate()
                .filter(|x| x.1.location == injury.location)
            {
                if i.1 > highest {
                    (index, highest) = i;
                }
            }
            *self.injuries[index].compound()
        } else {
            injury
        };

        self.shock(to_shock);
        self
    }

    fn shock(&mut self, injury: Injury) -> Option<ShockState> {
        /* TODO: implement fatigue */
        let i: i8 = self.body.shock(injury)?
            + match self.success_test(&Skill::Shock, 0) {
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
            println!("{} is {s}!", self.name());
        }
        let s = self.shock_state();
        *s = shock;
        shock
    }

    pub fn attack(
        &mut self,
        target: &mut Being,
        range: u16,
        aim_zone: u8,
        atk_mode: Mode,
        ammo: Option<Item>,
        defense: item::DefenseOption,
        def_mode: Mode,
        atk_mod: i8,
        def_mod: i8,
    ) -> Option<Injury> {
        match atk_mode {
            Mode::Melee {
                two_hand,
                length,
                aspect,
                zone_die,
                impact_die,
                impact_mod,
                ..
            } => {
                if two_hand && self.hands_full() > 1 {
                    eprintln!("Cannot use two-handed attack mode without free hand");
                    return None;
                }
                eprintln!(
                    "{} performs a melee attack against {} using {}",
                    self.name(),
                    target.name(),
                    atk_mode,
                );
                if range
                    > match length.into() {
                        ..5 => 5,
                        _ => length.into(),
                    }
                {
                    eprintln!("Out of range!");
                    return None;
                }
                let (atk_r, def_r) = atk_mode.reach_diff(def_mode);
                let (res, stars) = self.opposed_test(
                    &Skill::Melee,
                    atk_mod + self.heft_mod(atk_mode)? + atk_r,
                    defense.test(target, def_mode, def_mod, def_r),
                );

                // TODO: other options besides additional impact TA
                let atk = item::Attack::new(
                    self.body.roll(zone_die, aim_zone - 1)?,
                    random_range(1..=impact_die)
                        + self.str_mod()
                        + (stars * impact_mod).cast_signed(),
                    aspect,
                );
                let def = item::Attack::new(
                    target.body.roll(zone_die, aim_zone - 1)?,
                    random_range(1..=def_mode.impact_die())
                        + target.str_mod()
                        + (stars * def_mode.impact_mod()).cast_signed(),
                    def_mode.aspect(),
                );

                match res {
                    OpposedResult::Victory => {
                        let injury = target.inventory.calc_injury(atk);
                        target.injure(injury?);
                        injury
                    }
                    OpposedResult::Tie(winner) => match defense {
                        item::DefenseOption::Block => None,
                        item::DefenseOption::Dodge => {
                            if winner == Tiebreak::Lhs {
                                let injury = target.inventory.calc_injury(atk);
                                target.injure(injury?);
                                injury
                            } else {
                                None
                            }
                        }
                        item::DefenseOption::Counterstrike => {
                            if winner == Tiebreak::None {
                                None
                            } else {
                                let injury = target.inventory.calc_injury(def);
                                target.injure(injury?);
                                injury
                            }
                        }
                    },
                    OpposedResult::Loss => {
                        if matches!(defense, item::DefenseOption::Counterstrike) {
                            let injury = self.inventory.calc_injury(def);
                            self.injure(injury?);
                            injury
                        } else {
                            None
                        }
                    }
                }
            }

            Mode::Range {
                two_hand,
                impact_mod,
                ..
            } => {
                if two_hand && self.hands_full() > 1 {
                    eprintln!("Cannot use two-handed attack mode without free hand");
                    return None;
                }
                // TODO: implement draw weight
                eprintln!(
                    "{} performs a ranged attack against {} using {}",
                    self.name(),
                    target.name(),
                    atk_mode,
                );
                let r = atk_mode.calc_range(range);
                eprintln!("calculating range {r:?}");
                let stars = match self.success_test(&Skill::Archery, r.test_mod()?) {
                    SuccessResult::CriticalSuccess(_) => 2,
                    SuccessResult::Success(_) => 1,
                    _ => return None,
                };
                eprintln!("victory stars {stars}");
                let attack = item::Attack::new(
                    target
                        .body
                        .roll(random_range(1..=r.zone_die()?), aim_zone)?,
                    ammo?.calc_impact(r.impact_mod()? + (impact_mod * stars).cast_signed()),
                    item::Aspect::Point,
                );
                eprintln!("attack {attack:?}");
                let injury = target.inventory.calc_injury(attack);
                eprintln!("injury {injury:?}");
                if let Some(i) = injury {
                    target.injure(i);
                }
                injury
            }
        }
    }
}
