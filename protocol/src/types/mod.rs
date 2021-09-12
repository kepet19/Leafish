// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod metadata;
pub use self::metadata::*;

pub mod bit;
pub mod hash;
pub mod nibble;

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl From<u8> for GameMode {
    fn from(val: u8) -> Self {
        GameMode::from(val as i32)
    }
}

impl From<f32> for GameMode {
    fn from(val: f32) -> Self {
        GameMode::from(val as i32)
    }
}

impl From<i32> for GameMode {
    fn from(val: i32) -> Self {
        match val {
            3 => GameMode::Spectator,
            2 => GameMode::Adventure,
            1 => GameMode::Creative,
            0 => GameMode::Survival,
            _ => GameMode::Survival,
        }
    }
}
impl GameMode {
    pub fn can_fly(&self) -> bool {
        matches!(*self, GameMode::Creative | GameMode::Spectator)
    }

    pub fn is_spectator(&self) -> bool {
        matches!(*self, GameMode::Spectator)
    }
}
