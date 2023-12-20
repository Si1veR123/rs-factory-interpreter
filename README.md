# Factory interpreter
[Factory esolang](https://esolangs.org/wiki/Factory)
## Rust library
Usage is simple and an example can be found in the provided binary.

`Factory::default()` constructs a new interpreter.

`commands::commands_from_str` parses the factory code to a series of commands.

`Factory::interpret_command` runs a given command in a factory.

`Factory` implements `Display`, and printing it shows the factory as following:

```
      Production (1)
      Storage Space: _
      Storage Space: _
      Storage Space: _
      Garbage
      Shipping: 0, 1, 0, 0, 1
      Supply: _
 1 -> Invertor: None
      And: None
```

## Binary
Build with `cargo build -r`.  
The binary runs a hello world script, printing the factory after each step.

# WASM bindings
Build with 
```bash
wasm-pack build --release --target web --out-dir site/pkg -- --features=wasm
```
This will compile the library to WASM and build the JavaScript bindings in the folder `site/pkg`.

[site/basic.html](site/basic.html) is an example of using these bindings.
