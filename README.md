# Introduction
This is a tool for handling various operations in the HMK roleplaying system, such as:
* Success Tests
* Opposed Success Tests
* Value Tests
* Ranged Combat Rolls
* Melee Combat Rolls
* Skill and Attribute ML and EML calculation
* Armor Calculation from an Armor Set
* Injury Calculation and Tracking
* Automatic Adjustment to EML from Injuries
* Character Sheet Management
* Shock Management
* NPC Tracking
* Initiative Order Tracking

The CLI is extremely crude at the moment, and is subject to change.
Many more features are planned, such as character creation.

# Character Sheet Format
The character sheets are encoded in [MessagePack](https://msgpack.org/index.html) to allow for complex skill keys such as `Language("Hârnic")`, which is not possible in most (easily) human readable formats.
The tool is capable of reading toml files, but currently there is no interface to access this functionality for player characters.

The premade characters from the free "On the Silver Way" module are included.

# Installing
You will need Rust and Cargo, which you can install with [Rustup](https://rustup.rs/).

Build and run the tool with Cargo:
```bash
cargo install --path .
```
