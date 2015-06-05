mod apu;
mod cpu;
mod ppu;

use cpu::CPU2A03;

fn main() {
    let nes = CPU2A03::new();
    println!("No emulation to see here!");
}
