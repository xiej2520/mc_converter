#![allow(non_snake_case)]

/// https://minecraft.wiki/w/Bedrock_Edition_level_format/History#0.8.1_and_below
// In Bedrock and Pocket Edition, numbers are encoded in little-endian.
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The level.dat file is in NBT format, based on the format of level.dat in a Desktop world.
/// level.dat is an uncompressed little-endian NBT file that stores environmental data (time of day, for example) and player health, inventory, velocity, and position within the map.
///
/// The file begins with an 8-byte header, consisting of a little-endian 4-byte integer indicating the storage version. It is followed by another integer containing the length of the file, minus the header.
///
/// NBT Structure
///
/// [NBT Compound / JSON Object] World data.
///     [Int] GameType: Whether in survival (0) or in creative (1) mode.
///     [Long] LastPlayed: Stores the Unix time stamp (in seconds) when the player saved the game.
///     [String] LevelName: Specifies the name of the level.
///     [Int] Platform: Seems to store the platform that the level is created on. Currently observed value is 2.
///     [NBT Compound / JSON Object] Player: Player entity information. See Entity Format and Mob Entity Format for details. It is missing the id tag and has additional elements:
///         [NBT List / JSON Array] Armor: Each TAG_Compound in this list defines a piece of armor that the player is wearing. This is a list with length 4 - for helmet, chestplate, leggings, and boots.
///             [NBT Compound / JSON Object] Inventory item data
///                 [Short] id: Item or Block ID.
///                 [Byte] Count: Number of items stacked in this inventory slot. Any item can be stacked, including tools. Range is 1-255. Values above 255 are not displayed in-game.
///                 [Short] Damage: For armor, the amount of wear they have suffered. The maximum durability of the armor means undamaged. When the Damage reaches 0, it breaks and disappears.
///         [Int] Dimension: The dimension the player is in. 0 is the Overworld.
///         [NBT List / JSON Array] Inventory: Each TAG_Compound in this list defines an item the player is carrying or holding.
///             [NBT Compound / JSON Object] Inventory item data
///                 [Byte] Slot: Indicates which inventory slot this item is in.
///                 [Short] id: Item or Block ID.
///                 [Byte] Count: Number of items stacked in this inventory slot. Any item can be stacked, including tools. Range is 1-255. Values above 255 are not displayed in-game.
///                 [Short] Damage: For tools, the amount of wear they have suffered. The maximum durability of the tool (for example, 33 for golden tools) means undamaged. When the Damage reaches 0, it breaks and disappears.
///         [Int] Score: The score of the player.
///         [Byte] Sleeping: 1 or 0 (true/false) - true if the player was in a bed when this tag was saved; has no effect on whether the player is in a bed when they log in.
///         [Short] SleepTimer: The number of ticks the player had been in bed when this tag was saved. No effect.
///         [Int] SpawnX: X coordinate of the player's spawn position. Default is 0.
///         [Int] SpawnY: Y coordinate of the player's spawn position. Default is 64.
///         [Int] SpawnZ: Z coordinate of the player's spawn position. Default is 0.
///         [NBT Compound / JSON Object] abilities: The abilities this player has.
///             [Byte] mayfly: 1 or 0 (true/false) - true if the player can fly.
///             [Byte] flying: 1 or 0 (true/false) - true if the player is currently flying.
///             [Byte] invulnerable: 1 or 0 (true/false) - true if the player is immune to all damage and harmful effects except for void damage.
///             [Byte] mayBuild: 1 or 0 (true/false) - true if the player can place and destroy blocks.
///             [Byte] instabuild: 1 or 0 (true/false) - true if the player can instantly destroy blocks.
///     [Long] RandomSeed: Random number providing the Random Seed for the terrain.
///     [Long] SizeOnDisk: Estimated size of the entire world in bytes.
///     [Int] SpawnX: X coordinate of the world's spawn position. Default is 0.
///     [Int] SpawnY: Y coordinate of the world's spawn position. Default is 64.
///     [Int] SpawnZ: Z coordinate of the world's spawn position. Default is 0.
///     [Int] StorageVersion: Version of Bedrock Edition NBT, is 3.
///     [Long] Time: Stores the current "time of day" in ticks. There are 20 ticks per real-life second, and 19200 ticks per Minecraft daylight cycle, making the full cycle length 16 minutes—4 minutes shorter than the standard 20 minute daylight cycle. 0 is the start of daytime, 9600 is the start of sunset, 11040 is the start of nighttime, 17760 is the start of sunrise, and 19200 is daytime again. The value stored in level.dat is always increasing and can be larger than 19200, but the "time of day" is always modulo 19200 of the "Time" field value.
///     [Long] dayCycleStopTime: Determines the tick the daylight cycle is paused at. Values at or above 2,147,483,648 (231) result in the daylight cycle not being paused. Default value is 5,000 in creative mode, and 18,446,744,073,709,551,615 (264-1) in survival mode.
///     [Int] spawnMobs: Disable (0) or enable (1) mob spawning.

//
// header info
//    storage_version: i32, (is 3)
//    file_len: i32,

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LevelDat {
    pub GameType: GameType,
    pub LastPlayed: i64,
    pub LevelName: String,
    // 2 in PE 0.8.1 and below
    pub Platform: i32,
    pub Player: Player,
    pub RandomSeed: i64,
    pub SizeOnDisk: i64,
    pub SpawnX: i32,
    pub SpawnY: i32,
    pub SpawnZ: i32,
    // is 3
    pub StorageVersion: i32,
    pub Time: i64,
    pub dayCycleStopTime: i64,
    pub spawnMobs: SpawnMobs,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum GameType {
    Survival = 0,
    Creative = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum SpawnMobs {
    Disable = 0,
    Enable = 1,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub Armor: Vec<ArmorItem>,
    // always 0 since the nether wasn't added until 0.12.1
    pub Dimension: i32,
    pub Inventory: Vec<InventoryItem>,
    pub Score: i32,
    pub Sleeping: IsSleeping,
    pub SleepTimer: i16,
    pub abilities: PlayerAbilities,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArmorItem {
    pub id: i16,
    pub Count: i8,
    pub Damage: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItem {
    pub Slot: i8,
    pub id: i16,
    pub Count: i8,
    pub Damage: i16,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum IsSleeping {
    Sleeping = 1,
    NotSleeping = 0,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerAbilities {
    pub mayfly: bool,
    //pub flying: bool,
    //pub invulnerable: bool,
    ////pub mayBuild: bool,
    //pub instabuild: bool,
}

fn load_nbt() {}
