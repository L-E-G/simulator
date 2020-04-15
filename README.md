# Simulator
Instruction set architecture simulator.

# Table Of Contents
- [Overview](#overview)
- [Development](#development)

# Overview
Simulation of processor which implements the LEG specification.

See [Development](#development) section.

# Development
There are two components in this repository: 

- **Simulator**: Simulator of a processor which implements the LEG specification
- **Graphical User Interface**: Web app which runs the simulator implementation
  was web assembly and provides a user interface
  
The development workflow is different for these components.

## Simulator Development
The simulator is pure Rust. It includes a binary program which provides a very
rudimentary text interface. Additionally many of its components include tests.

Source code is located in `src/*.rs` with the exception of the `src/lib.rs` 
file. Which is part of the graphical user interface.

To run the text interface:

```
cargo run
```

To test:

```
cargo test
```

## Graphical User Interface Development
The GUI is a JavaScript application which invokes a version of the simulator
compiled into web assembly.

The GUI itself is comprised of two components:

**Web assembly compiled simulator**  

The web assembly compiled simulator defines bindings of items in Rust which 
JavaScript can access. These are defined in `src/lib.rs`. When compiled the 
resulting web assembly is located in the `pkg` directory. The 
[WASM Pack](https://rustwasm.github.io/wasm-pack/) tool is required to build 
these bindings.

Any time the simulator or binding rust code changes the web assembly compiled 
simulator must be built:

```
# In the repository root
wasm-pack build
```

**GUI JavaScript application**  

The GUI application is a normal Javascript web pack app who's source is located
in the `gui` directory and entry point is `gui/index.js`. It imports the web 
assembly compiled simulator as a JavaScript module. 

First install its JavasScript dependencies:

```
cd gui
npm install
```

Then start a web pack development server:

```
npm start
```

This server will serve the result on [localhost:8000](http://localhost:8000), 
and rebuild the web app whenever anything in `gui` changes, or when a new 
version of the web pack compiled simulator is built.
