use wasm_bindgen::prelude::wasm_bindgen;
use lazy_static::lazy_static;

use super::{Factory, commands::{commands_from_str, Command}};

use std::{sync::Mutex, io::BufReader};
use std::io::{Write, Read};

struct VirtualWasmBuffer {
    id: usize,
    output_buffer: Vec<u8>,

    wasm_output_function: fn(usize, &str),
    wasm_input_function: fn(usize) -> String,
}

impl VirtualWasmBuffer {
    fn new(id: usize, wasm_input_function: fn(usize) -> String, wasm_output_function: fn(usize, &str)) -> Self {
        Self { id, output_buffer: Vec::new(), wasm_output_function, wasm_input_function }
    }
}

impl Write for VirtualWasmBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output_buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        (self.wasm_output_function)(self.id, std::str::from_utf8(self.output_buffer.as_slice()).expect("Outputted invalid utf8"));
        self.output_buffer.clear();
        Ok(())
    }
}

impl Read for VirtualWasmBuffer {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let input_bytes = (self.wasm_input_function)(self.id);
        buf.write_all(&input_bytes.as_bytes())?;
        Ok(input_bytes.len())
    }
}

// #[wasm_bindgen]
// pub struct FactoryStats {
//     pub claw_position: usize,
//     pub claw_contents: Option<u8>,
//     pub ram: u8,
//     pub production_room: u8,
//     pub storage_space_1: Vec<bool>,
//     pub storage_space_2: Vec<bool>,
//     pub storage_space_3: Vec<bool>,
//     pub shipping: Vec<bool>,
//     pub supply: Vec<bool>,
//     pub invertor: Option<bool>,
//     pub and: Option<bool>
// }

type WasmFactory = Factory<BufReader<VirtualWasmBuffer>, VirtualWasmBuffer>;
lazy_static!{
    static ref FACTORIES: Mutex<Vec<WasmFactory>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
extern "C" {
    fn stdin(id: usize) -> String;
    fn stdout(id: usize, string: &str);
}

#[wasm_bindgen]
pub fn create_factory() -> usize {
    let mut lock = FACTORIES.lock().expect("Failed to lock mutex in rust");
    let index = lock.len();

    let factory = Factory::new_with_streams(
        BufReader::new(VirtualWasmBuffer::new(index, stdin, stdout)),
        VirtualWasmBuffer::new(index, stdin, stdout)
    );

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
            factory.interpret_command(command);
        },
        None => return
    }
}

// #[wasm_bindgen]
// pub fn get_factory_stats(factory: usize) -> Option<FactoryStats> {
//     let mut factories = FACTORIES.lock().expect("Failed to lock mutex in rust");
//     let factory = factories.get_mut(factory)?;
//     Some(FactoryStats {
//         claw_position: factory.claw_position,
//         claw_contents: factory.claw_contents,
//         ram: factory.ram,
//         production_room: *factory.rooms[0].as_option_bool().unwrap(),
//         storage_space_1: factory.rooms[1].as_storage_space().unwrap().bits.clone(),
//         storage_space_2: factory.rooms[2].as_storage_space().unwrap().bits.clone(),
//         storage_space_3: factory.rooms[3].as_storage_space().unwrap().bits.clone(),
//         shipping: factory.rooms[5].as_storage_space().unwrap().bits.clone(),
//         supply: factory.rooms[6].as_storage_space().unwrap().bits.clone(),
//         invertor: factory.rooms[7].as_option_bool().cloned(),
//         and: factory.rooms[8].as_option_bool().cloned()
//     })
// }
