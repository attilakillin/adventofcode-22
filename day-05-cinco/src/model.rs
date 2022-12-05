
#[derive(Clone)]
pub struct Crate {
    pub name: char,
}

pub struct System {
    stacks: Vec<Vec<Crate>>
}

impl System {
    pub fn initialize(lines: &[String]) -> Self {
        let (count, crates) = lines.split_last().unwrap();
        let count = count.split_ascii_whitespace().last().unwrap();
        let mut stacks: Vec<Vec<Crate>> = vec![vec![]; count.parse::<usize>().unwrap()];

        for line in crates.iter().rev() {
            let chars: Vec<u8> = line.chars().map(|c| c as u8).collect();
            for (stack_idx, chunk) in chars.chunks(4).enumerate() {
                let name = chunk[1];
                if name != ' ' as u8 {
                    stacks[stack_idx].push(Crate { name: name as char });
                }
            }
        }

        return System { stacks };
    }

    pub fn move_crates_v1(&mut self, count: usize, source: usize, target: usize) {
        for _ in 0..count {
            let element = self.stacks[source - 1].pop().unwrap();
            self.stacks[target - 1].push(element);
        }
    }

    pub fn move_crates_v2(&mut self, count: usize, source: usize, target: usize) {
        let source = &mut self.stacks[source - 1];
        let mut elements = source.split_off(source.len() - count);
        self.stacks[target - 1].append(&mut elements);
    }

    pub fn get_message(&self) -> String {
        let mut message = String::new();

        for stack in &self.stacks {
            message.push(stack.last().unwrap().name);
        }

        return message;
    }
}
