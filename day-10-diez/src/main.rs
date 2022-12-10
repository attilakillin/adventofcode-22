use aoc_common::input::Input;

struct CRT {
    cycle: isize,
    x: isize,
    strength: isize,
    display: String
}

impl CRT {
    fn new() -> Self {
        return CRT { cycle: 0, x: 1, strength: 0, display: String::from("") };
    }

    fn execute(&mut self, instruction: Vec<String>) {
        match instruction[0].as_str() {
            "addx" => {
                self.run_cycle();
                self.run_cycle();
                self.x += instruction[1].parse::<isize>().unwrap();
            },
            "noop" => {
                self.run_cycle();
            },
            any => panic!("No such instruction exists: {}!", any)
        }
    }

    fn run_cycle(&mut self) {
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.strength += self.cycle * self.x;
        }

        if isize::abs((self.cycle - 1) % 40 - self.x) <= 1 {
            self.display.push('#');
        } else {
            self.display.push('.');
        }

        if self.cycle % 40 == 0 {
            self.display.push('\n');
        }
    }
}

fn main() {
    let instructions = Input::new().read_lines().lines_as_words();
    let mut crt = CRT::new();

    for inst in instructions {
        crt.execute(inst);
    }

    println!("Answer for #1: {}", crt.strength);
    println!("Answer for #2: \n{}", crt.display);
}
