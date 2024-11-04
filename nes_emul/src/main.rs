pub mod cpu;

fn main() {
    println!("Hello, world!");
    let cpu = cpu::CPU::new();
    println!("{:?}", cpu)
}
