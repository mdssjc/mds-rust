use std::{fmt::Display, time::Duration};
use std::fmt;

use ggez::event::KeyCode;
use specs::World;

use crate::audio::AudioStore;
use crate::events::Event;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}

pub enum GameplayState {
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}
