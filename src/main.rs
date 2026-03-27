#![feature(try_find, hash_map_macro)]
#![warn(clippy::pedantic)]

mod being;
mod core;
mod item;

use std::path::PathBuf;

use anyhow_serde::{Context, Result};
use clap::{Parser, Subcommand};
use inquire::Select;
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    core::{Skill, SkillSet},
    item::{DefenseOption, Inventory, Mode},
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateNpc {
        name: String,
    },
    FullHeal {
        name: PathBuf,
    },
    Attack {
        attacker: PathBuf,
        target: PathBuf,
        #[arg(default_value_t = 5)]
        range: u16,
        #[arg(long, short = 'z', default_value_t = 1)]
        aim_zone: u8,
        #[arg(long, short, default_value_t = 0)]
        attack_modifier: i8,
        #[arg(long, short, default_value_t = 0)]
        defense_modifier: i8,
    },
    Test {
        name: PathBuf,
        skill: core::Skill,
    },
    OpposedTest {
        lhs_character: PathBuf,
        lhs_skill: core::Skill,
        rhs_character: PathBuf,
        rhs_skill: core::Skill,
    },
    ValueTest {
        name: PathBuf,
        skill: core::Skill,
    },
    Sheet {
        name: PathBuf,
    },
    Equip {
        name: PathBuf,
        items: Vec<PathBuf>,
    },
    Initiative {
        names: Vec<PathBuf>,
    },
    ModifySkill {
        name: PathBuf,
        skill: Skill,
        value: i16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::CreateNpc { name } => {
            let npc = being::Being::read_sheet(format!("beastiary/{}.toml", name))?;
            npc.write_sheet(name)?;
        }
        Commands::FullHeal { name } => {
            let mut c = being::Being::read_sheet(&name)?;
            c.injuries = Vec::new();
            c.shock = None;
            c.write_sheet(name)?;
        }
        Commands::Attack {
            attacker,
            target,
            range,
            aim_zone,
            attack_modifier,
            defense_modifier,
            ..
        } => {
            let mut a = being::Being::read_sheet(&attacker)?;
            let mut t = being::Being::read_sheet(&target)?;
            let atk_mode = Select::new("Select attack mode", a.modes()).prompt()?;
            let ammo = if matches!(atk_mode, item::Mode::Range { .. }) {
                Select::new("Ammo:", a.inventory.ammo()).prompt().ok()
            } else {
                None
            };
            let def_mode = Select::new("Select defense mode", t.modes()).prompt()?;
            let defense = if matches!(atk_mode, item::Mode::Melee { .. }) {
                Select::new(
                    "Select defense option",
                    item::DefenseOption::iter().collect(),
                )
                .prompt()?
            } else {
                DefenseOption::Dodge
            };
            a.attack(
                &mut t,
                range,
                aim_zone,
                atk_mode,
                ammo,
                defense,
                def_mode,
                attack_modifier,
                defense_modifier,
            );
            a.write_sheet(attacker)?;
            t.write_sheet(target)?;
        }
        Commands::Test { skill, name } => {
            let c = being::Being::read_sheet(name)?;
            println!("{:?}", c.success_test(&skill, 0));
        }
        Commands::OpposedTest {
            lhs_skill,
            lhs_character,
            rhs_skill,
            rhs_character,
        } => {
            let l = being::Being::read_sheet(&lhs_character)?;
            let r = being::Being::read_sheet(&rhs_character)?;
            println!(
                "{:?}",
                l.opposed_test(&lhs_skill, 0, r.success_test(&rhs_skill, 0))
            );
        }
        Commands::ValueTest { skill, name } => {
            let c = being::Being::read_sheet(&name)?;
            println!("{:?}", c.value_test(&skill, 0));
        }
        Commands::Sheet { name } => {
            let c = being::Being::read_sheet(&name)?;
            println!("{c}");
        }
        Commands::Equip { name, items } => {
            let mut c = being::Being::read_sheet(&name)?;
            for i in items {
                c.equip(item::Item::from_file(i)?);
            }
            c.write_sheet(name)?;
        }
        Commands::Initiative { names } => {
            names
                .iter()
                .map(|name| {
                    let c = being::Being::read_sheet(name).unwrap();
                    (
                        c.name().clone(),
                        c.ml(&Skill::Initiative),
                        if let Some(shock) = c.shock {
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
        Commands::ModifySkill { name, skill, value } => {
            let mut c = being::Being::read_sheet(&name)?;
            c.skills.modify(skill, value);
            c.write_sheet(name)?;
        }
    }
    Ok(())
}
