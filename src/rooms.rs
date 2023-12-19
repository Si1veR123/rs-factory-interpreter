use std::fmt::Display;

pub trait RoomInteraction {
    fn pickup(&mut self) -> Option<bool>;
    fn dropoff(&mut self, bit: bool);
}

#[derive(Debug)]
pub enum Room {
    Production(bool),
    StorageSpace(StorageSpace),
    Garbage,
    Shipping(StorageSpace),
    Supply(StorageSpace),
    Invertor(Option<bool>),
    And(Option<bool>)
}

impl Room {
    pub fn as_storage_space(&mut self) -> Option<&mut StorageSpace> {
        match self {
            Room::StorageSpace(s) |
            Room::Shipping(s) |
            Room::Supply(s) => Some(s),
            _ => None
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Room::Production(b) => write!(f, "Production ({})", *b as u8),
            Room::StorageSpace(s) => write!(f, "Storage Space: {}", s),
            Room::Garbage => write!(f, "Garbage"),
            Room::Shipping(s) => write!(f, "Shipping: {}", s),
            Room::Supply(s) => write!(f, "Supply: {}", s),
            Room::Invertor(contents) => match contents {
                Some(b) => write!(f, "Invertor: {}", *b as u8),
                None => write!(f, "Invertor: None")
            },
            Room::And(contents) => match contents {
                Some(b) => write!(f, "And: {}", *b as u8),
                None => write!(f, "And: None")
            }
        }
    }
}

impl RoomInteraction for Room {
    fn pickup(&mut self) -> Option<bool> {
        match self {
            Room::Production(b) => Some(*b),
            Room::Garbage => None,

            Room::Invertor(contents) |
            Room::And(contents) => contents.take(),
            
            Room::StorageSpace(s) |
            Room::Shipping(s) |
            Room::Supply(s) => s.pickup()
        }
    }

    fn dropoff(&mut self, bit: bool) {
        match self {
            Room::Production(b) => *b = bit,
            Room::Invertor(contents) => *contents = Some(!bit),
            Room::And(contents) => *contents = Some(contents.map_or(bit, |stored_bit| stored_bit & bit)),

            Room::StorageSpace(s) |
            Room::Shipping(s) |
            Room::Supply(s) => s.dropoff(bit),

            Room::Garbage => ()
        }
    }
}

#[derive(Debug)]
pub struct StorageSpace {
    bits: Vec<bool>
}

impl StorageSpace {
    pub fn clear(&mut self) {
        self.bits.clear()
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.bits.len()/8);

        for byte in self.bits.chunks_exact(8) {
            bytes.push(byte.iter().fold(0, |acc, &b| acc*2 + b as u8))
        }

        bytes
    }
}

impl Default for StorageSpace {
    fn default() -> Self {
        Self { bits: Vec::new() }
    }
}

impl RoomInteraction for StorageSpace {
    fn pickup(&mut self) -> Option<bool> {
        self.bits.pop()
    }

    fn dropoff(&mut self, bit: bool) {
        self.bits.push(bit)
    }
}

impl Display for StorageSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bits.len() == 0 {
            write!(f, "_")?;
            return Ok(())
        }
        
        for bit in self.bits.get(0..self.bits.len()-1).unwrap() {
            write!(f, "{}, ", *bit as u8)?;
        }

        write!(f, "{}", *self.bits.last().unwrap() as u8)?;

        Ok(())
    }
}
