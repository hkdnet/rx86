use rx86::Emulator;
use std::env::args;
use std::fs::read;

fn main() {
    if let Some(filename) = args().nth(1) {
        match read(filename.clone()) {
            Ok(codes) => {
                let mut emulator = Emulator::new();
                emulator.load(codes).unwrap();
                println!("{}", emulator);
                println!("start emulation...");
                if let Err(e) = emulator.run() {
                    eprintln!("ERROR: {}", e)
                }
                emulator.show_registers();
            }
            Err(e) => eprintln!("cannot read {}: {}", filename, e),
        }
    } else {
        eprintln!("Usage: {} FILENAME", args().nth(0).unwrap());
    }
}
