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

While the code itself was written in a way to allow for more complex tests, such as adding modifiers to rolls, selecting specific weapons, or choosing how Tactical Advantages are used, these features aren't implemented in the CLI yet.

# Character Sheet Format
The basic format for character sheets uses TOML.
You can manually edit the character sheet files in a text editor if desired.

In the future, I'll have a way to also read character sheets that are encoded in [MessagePack](https://msgpack.org/index.html) to allow for complex skill keys such as `Language("Hârnic")`, which is not possible in most (easily) human readable formats.

The premade characters from the free "On the Silver Way" module are included.

# Installing
You will need Rust and Cargo, which you can install with [Rustup](https://rustup.rs/).

Build and run the tool with Cargo:
```bash
cargo install --path .
```

Currently, the tool relies on a specific directory structure to work, so I'd recommend just running it from the project directory.


This is unofficial Hârn fan material. Hârn®, HârnWorld®, and HârnMaster® are registered trademarks of Arien Crossby, licensed by Keléstia Productions Ltd (www.kelestia.com). All related concepts and material are the property of Arien Crossby and Keléstia Productions Ltd (www.kelestia.com). Used with permission.
