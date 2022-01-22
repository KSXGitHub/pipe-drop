use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "View drop order when there are no explicit drop calls")]
    Auto,
    #[clap(about = "View drop order when std::mem::drop is used")]
    StdMemDrop,
    #[clap(about = "View drop order when pipe_drop::PipeDrop is used")]
    PipeDrop,
}

struct Object {
    id: usize,
}

impl Default for Object {
    fn default() -> Self {
        println!("create 0");
        Object { id: 0 }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("drop {}", self.id)
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let id = self.id + 1;
        println!("create {}", id);
        Object { id }
    }
}

fn main() {
    #![allow(unused_variables)]
    #![allow(clippy::redundant_clone)]
    match Cli::parse().command {
        Command::Auto => {
            let a = Object::default();
            let b = a.clone();
            let c = b.clone();
            println!("-- end of scope --");
        }
        Command::StdMemDrop => {
            let a = Object::default();
            let b = a.clone();
            drop(a);
            let c = b.clone();
            drop(b);
            println!("-- end of scope --");
        }
        Command::PipeDrop => {
            use pipe_drop::PipeDrop;
            let c = Object::default() // a
                .pipe_ref_drop(Object::clone) // b
                .pipe_ref_drop(Object::clone);
            println!("-- end of scope --");
        }
    }
}
