use std::{io::{Write, stdin, stdout, BufRead}, fmt::Display};

use super::{rooms::{Room, StorageSpace, RoomInteraction}, commands::Command};

// If wasm feature, use serde and derive
#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Factory {
    pub claw_contents: Option<bool>,
    claw_position: usize,
    pub rooms: [Room; 9],
    pub ram: bool
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            claw_contents: None,
            claw_position: 0,
            rooms: [
                Room::Production(true),
                Room::StorageSpace(StorageSpace::default()),
                Room::StorageSpace(StorageSpace::default()),
                Room::StorageSpace(StorageSpace::default()),
                Room::Garbage,
                Room::Shipping(StorageSpace::default()),
                Room::Supply(StorageSpace::default()),
                Room::Invertor(None),
                Room::And(None)
            ],
            ram: false
        }
    }
}

impl Display for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..9 {
            if self.claw_position == row {
                match self.claw_contents {
                    Some(b) => write!(f, " {} -> ", b as u8),
                    None => write!(f, "   -> ")
                }?;
                
            } else {
                write!(f, "      ")?;
            }

            write!(f, "{}", self.rooms[row])?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Factory {
    pub fn interpret_command<C: Into<Command>>(&mut self, command: C) {
        match command.into() {
            Command::Boot => (),
            Command::Left => self.claw_position = self.claw_position.saturating_sub(1),
            Command::Right => self.claw_position = (self.claw_position + 1).min(8),
            Command::Claw => match self.claw_contents {
                Some(b) => {
                    self.rooms[self.claw_position].dropoff(b);
                    self.claw_contents = None;
                },
                None => self.claw_contents = self.rooms[self.claw_position].pickup()
            },
            Command::Ram => match self.claw_contents {
                Some(b) => self.ram = b,
                None => self.ram = !self.ram
            },
            Command::SendShipment => {
                let shipping = self.rooms[5].as_storage_space().unwrap();
                let text = shipping.get_bytes();
                stdout().write(&text).expect("Output buffer error");
                shipping.clear();
            },
            Command::RequestShipment => {
                let supply = self.rooms[6].as_storage_space().unwrap();
                let mut buffer = Vec::new();
                stdin().lock().read_until(b'\n', &mut buffer).expect("Input buffer error");
                supply.clear();

                for &byte in buffer.iter().rev() {
                    if byte == b'\n' || byte == b'\r' { continue }
                    for i in 0..8 {
                        supply.dropoff(((1 << i) & byte) != 0)
                    }
                }
            },
        }
    }

    pub fn claw_current_room(&self) -> &Room {
        &self.rooms[self.claw_position]
    }

    pub fn claw_current_room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.claw_position]
    }
}