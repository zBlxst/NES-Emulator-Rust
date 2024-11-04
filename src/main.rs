use cpu::CPU;

pub mod cpu;

fn main() {
    println!("Hello, world!");
    let mut cpu : CPU = CPU::new();

    let program : Vec<u8> = vec![0xa9, 0xc0, 0x00];
    println!("{:?}", cpu);
    cpu.interpret(&program);
    println!("{:?}", cpu);

}
