//! Minithread - A minimal green threads implementation
//! 
//! This is an experimental project to learn about green threads implementation
//! using setjmp/longjmp for context switching. The goal is to create a simple
//! but functional green threads library for educational purposes.
//! 
//! # Roadmap
//! - [x] Basic project structure
//! - [ ] Context switching with setjmp/longjmp
//! - [ ] Stack allocation and management
//! - [ ] Round-robin scheduler
//! - [ ] Task lifecycle management
//! - [ ] Basic examples and tests
//! - [ ] Error handling and safety improvements
//! - [ ] Performance optimizations

pub mod context;
pub mod stack;
pub mod scheduler;
pub mod task;
pub mod macros;
pub mod util;

//! Public API should include:
//! - spawn() function to create new green threads
//! - yield_now() function to yield control
//! - run() function to start the scheduler
//! - TaskId type for identifying tasks
