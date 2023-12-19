use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use serde_wasm_bindgen::{to_value, Error as SerdeWasmError};

use super::{Factory, commands::{commands_from_str, Command}};


#[wasm_bindgen]
pub fn create_factory() -> Result<JsValue, SerdeWasmError> {
    to_value(&Factory::default())
}

#[wasm_bindgen]
pub fn read_commands(string: &str) -> Vec<Command> {
    commands_from_str(string)
}

