#![feature(try_find, hash_map_macro)]
#![warn(clippy::pedantic)]

mod being;
mod core;
mod item;

use anyhow_serde::Result;

use crate::{being::BodyType, core::AttributeSet, item::Mode};

/*
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
*/

/*
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
*/

fn main() -> Result<()> {
    use core::Attribute::*;
    let mut nolora = being::Being::read_sheet_toml("nolora")?;
    let mut elben = being::Being::read_sheet_toml("elben")?;
    let nolora_attack_mode = *nolora.prime_hand.clone().unwrap().modes().first().unwrap();
    let elben_attack_mode = *elben.prime_hand.clone().unwrap().modes().first().unwrap();
    elben.attack(
        &mut nolora,
        3,
        1,
        elben_attack_mode,
        item::DefenseOption::Dodge,
        nolora_attack_mode,
        0,
        0,
    );
    println!("{nolora}");
    println!("{elben}");
    nolora.write_sheet("nolora")?;
    elben.write_sheet("elben")?;
    /*
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
            println!(
                "{:?}",
                l.opposed_test(lhs_skill, 0, r.success_test(rhs_skill, 0))
            );
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
    */
    Ok(())
}
