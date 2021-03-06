//! Systems for spawning and despawning entities

mod despawn;
mod spawner;

pub use self::{
    despawn::{DespawnAtBorderSystem, DespawnTimeLimitSystem},
    spawner::{AutoSpawnerSystem, SpawnerSystem},
};
