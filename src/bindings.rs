use wasm_bindgen::prelude::wasm_bindgen;
use lazy_static::lazy_static;

use super::{Factory, commands::{commands_from_str, Command}};

use std::sync::Mutex;

lazy_static!{
    static ref FACTORIES: Mutex<Vec<Factory>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
pub fn create_factory() -> usize {
    let factory = Factory::default();
    let mut lock = FACTORIES.lock().expect("Failed to lock mutex in rust");
    let index = lock.len();
    lock.push(factory);
    index
}

#[wasm_bindgen]
pub fn read_commands(string: &str) -> Vec<Command> {
    commands_from_str(string)
}

#[wasm_bindgen]
pub fn interpret_command(factory: usize, command: Command) {
    let mut lock = FACTORIES.lock().expect("Failed to lock mutex in rust");
    let factory_opt = lock.get_mut(factory);

    match factory_opt {
        Some(factory) => {
            factory.interpret_command(command)
        },
        None => return
    }
}
