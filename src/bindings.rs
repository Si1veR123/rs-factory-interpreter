use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use super::{Factory, commands::{commands_from_str, Command}};

use std::{sync::Mutex, io::BufReader};
use std::io::{Write, Read, self};

struct VirtualWasmBuffer {
    output_buffer: Vec<u8>,

    wasm_output_function: fn(&str),
    wasm_input_function: fn() -> String,
}

impl VirtualWasmBuffer {
    fn new(wasm_input_function: fn() -> String, wasm_output_function: fn(&str)) -> Self {
        Self { output_buffer: Vec::new(), wasm_output_function, wasm_input_function }
    }
}

impl Write for VirtualWasmBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output_buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (self.wasm_output_function)(
            std::str::from_utf8(self.output_buffer.as_slice()).map_err(|_| io::ErrorKind::InvalidData)?
        );
        self.output_buffer.clear();
        Ok(())
    }
}

impl Read for VirtualWasmBuffer {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let input_bytes = (self.wasm_input_function)();
        buf.write_all(&input_bytes.as_bytes())?;
        Ok(input_bytes.len())
    }
}

type WasmFactory = Factory<BufReader<VirtualWasmBuffer>, VirtualWasmBuffer>;
lazy_static!{
    static ref FACTORIES: Mutex<Vec<WasmFactory>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
extern "C" {
    fn stdin() -> String;
    fn stdout(string: &str);
}

#[wasm_bindgen]
pub fn create_factory() -> Option<usize> {
    let factory = Factory::new_with_streams(
        BufReader::new(VirtualWasmBuffer::new(stdin, stdout)),
        VirtualWasmBuffer::new(stdin, stdout)
    );

    let mut lock = FACTORIES.lock().ok()?;
    let index = lock.len();

    lock.push(factory);
    Some(index)
}

#[wasm_bindgen]
pub fn read_commands(string: &str) -> Vec<Command> {
    commands_from_str(string)
}

#[wasm_bindgen]
pub fn interpret_command(factory: usize, command: Command) -> bool {
    let lock_res = FACTORIES.lock();

    match lock_res {
        Ok(mut lock) => {
            let factory_opt = lock.get_mut(factory);

            match factory_opt {
                Some(factory) => {
                    factory.interpret_command(command);
                    true
                },
                None => false
            }
        },
        Err(_) => false
    }
}

#[derive(Deserialize, Serialize)]
pub struct FactoryStats {
    pub claw_position: usize,
    pub claw_contents: Option<bool>,
    pub ram: bool,
    pub production_room: bool,
    pub storage_space_1: Vec<bool>,
    pub storage_space_2: Vec<bool>,
    pub storage_space_3: Vec<bool>,
    pub shipping: Vec<bool>,
    pub supply: Vec<bool>,
    pub invertor: Option<bool>,
    pub and: Option<bool>
}

#[wasm_bindgen]
pub fn get_factory_stats(factory: usize) -> JsValue {
    let mut factories = FACTORIES.lock().expect("Failed to lock mutex in rust");
    let factory_opt = factories.get_mut(factory);

    let Some(factory) = factory_opt else {
        return JsValue::UNDEFINED
    };

    let stats = FactoryStats {
        claw_position: factory.claw_position,
        claw_contents: factory.claw_contents,
        ram: factory.ram,
        production_room: *factory.rooms[0].as_option_bool().unwrap(),
        storage_space_1: factory.rooms[1].as_storage_space().unwrap().bits.clone(),
        storage_space_2: factory.rooms[2].as_storage_space().unwrap().bits.clone(),
        storage_space_3: factory.rooms[3].as_storage_space().unwrap().bits.clone(),
        shipping: factory.rooms[5].as_storage_space().unwrap().bits.clone(),
        supply: factory.rooms[6].as_storage_space().unwrap().bits.clone(),
        invertor: factory.rooms[7].as_option_bool().cloned(),
        and: factory.rooms[8].as_option_bool().cloned()
    };

    match to_value(&stats) {
        Ok(s) => s,
        Err(_) => JsValue::UNDEFINED
    }
}
