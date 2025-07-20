# Minithread

A minimal green threads implementation for educational purposes.

## Overview

This is an experimental project to learn about green threads implementation using `setjmp`/`longjmp` for context switching. The goal is to create a simple but functional green threads library.

## Project Structure

```
minithread/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs          # Public API: spawn, yield_now, run
│   ├── context.rs      # CPU context: setjmp/longjmp via libc
│   ├── stack.rs        # Stack allocation and raw pointer handling
│   ├── scheduler.rs    # Round-robin scheduler + task queue
│   ├── task.rs         # Task: stack, context, closure
│   ├── macros.rs       # (Optional) Macros that facilitate usage
│   └── util.rs         # Debug, align helpers, ev. FFI
├── examples/
│   └── pingpong.rs     # Two tasks that take turns writing "ping"/"pong"
└── tests/
    └── smoke.rs        # Simple smoke test: spawn, yield, run
```

## Roadmap

- [x] Basic project structure
- [ ] Context switching with setjmp/longjmp
- [ ] Stack allocation and management
- [ ] Round-robin scheduler
- [ ] Task lifecycle management
- [ ] Basic examples and tests
- [ ] Error handling and safety improvements
- [ ] Performance optimizations

## Usage

```rust
use minithread::{spawn, yield_now, run};

fn main() {
    spawn(|| {
        println!("Hello from task 1!");
        yield_now();
        println!("Back in task 1!");
    });
    
    spawn(|| {
        println!("Hello from task 2!");
        yield_now();
        println!("Back in task 2!");
    });
    
    run();
}
```

## License

MIT
