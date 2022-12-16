mod cpu;
mod gpu;

fn main() {
    let input = include_str!("input.txt");
    let mut cpu = cpu::CPU::init(input);
    // println!(
    //     "cumulative signal stregnth: {}",
    //     cpu.get_cumulative_signal_stregnths()
    // );
    let mut gpu = gpu::GPU::init();
    while {
        gpu.tick(cpu.cycle, cpu.reg_x);
        cpu.tick() == Some(())
    } {}
    // for _ in 0..3 {
    //     print!("cycle {} (reg_x is {})", cpu.cycle, cpu.reg_x);
    //     gpu.tick(cpu.cycle, cpu.reg_x);
    //     gpu.display();

    //     cpu.tick();
    // }
}
