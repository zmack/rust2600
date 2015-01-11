#![feature(box_syntax)]
use cart::Cart;
use cpu::CPU;
use memory::Memory;

mod cart;
mod cpu;
mod memory;


fn main() {
    let mut memory = Memory::new();
    let cart = Cart::new("fixtures/hello2k.bin");
    memory.load(*cart.data, 0);

    let mut cpu = CPU::new(&mut memory);
    for i in range(0,400) {
        cpu.tick();
    }
    println!("This thing is awesome!");
}
