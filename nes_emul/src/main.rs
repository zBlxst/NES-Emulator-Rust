use cpu::CPU;

pub mod cpu;

fn main() {
    println!("Hello, world!");
    let mut cpu : CPU = CPU::new();

    let mut program : Vec<u8> = Vec::new();
    program.push(0xa9);
    program.push(0xc0);
    program.push(0xff);
    println!("{:?}", cpu);
    cpu.interpret(&program);
}
