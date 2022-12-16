const WIDTH: usize = 40;
const HEIGHT: usize = 6;

pub struct GPU {
    framebuffer: [[bool; WIDTH]; HEIGHT],
}
impl GPU {
    pub fn init() -> Self {
        GPU {
            framebuffer: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn display(self: &Self) {
        println!("");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let lit = self.framebuffer[y][x];
                if lit {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn tick(self: &mut Self, cycle: usize, reg_x: isize) {
        let x = (cycle - 1) % WIDTH;
        let y = ((cycle - 1) / WIDTH) % HEIGHT;
        let xi = x as isize;
        let lit = reg_x == xi || reg_x == xi + 1 || reg_x == xi - 1;
        self.framebuffer[y][x] = lit;
        if x + 1 == WIDTH && y + 1 == HEIGHT {
            self.display();
        }
    }
}
