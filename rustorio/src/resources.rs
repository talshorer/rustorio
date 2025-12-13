//! Resources are the fundamental units of value in Rustorio.
//! Resources are held in either [`Resource`](crate::Resource) or [`Bundle`](crate::Bundle) objects.
//! [`Bundle`](crate::Bundle) objects are used to hold a fixed amount of a resource, while [`Resource`](crate::Resource) objects can hold any amount.
//!
//! This module defines the core resources used in Rustorio.

use rustorio_engine::resource_type;

resource_type!(
    /// Used to win the game in some game modes.
    Point);

resource_type!(
    /// Raw iron ore mined from the ground.
    /// Can be smelted into iron ingots using a [`Furnace`](crate::buildings::Furnace).
    IronOre);

resource_type!(
    /// Refined iron ingots produced by smelting iron ore.
    /// Used in various recipes and to build structures.
    Iron);

resource_type!(
    /// Raw copper ore mined from the ground.
    /// Can be smelted into copper ingots using a [`Furnace`](crate::buildings::Furnace).
    CopperOre);

resource_type!(
    /// Refined copper ingots produced by smelting copper ore.
    /// Used in various recipes and to build structures.
    Copper);
