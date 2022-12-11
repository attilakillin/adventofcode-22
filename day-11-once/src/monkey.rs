#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    items: Vec<isize>,
    operation: (String, char, String),
    test_modulo: isize,
    division_constant: isize,
    true_target_id: usize,
    false_target_id: usize,

    pub inspections: usize
}

impl Monkey {
    pub fn new(text: &[String], division_constant: isize) -> Self {
        let id = text[0].replace("Monkey ", "").replace(":", "").parse::<usize>().unwrap();
        let items = text[1].replace("  Starting items: ", "").split(", ").map(|e| e.parse::<isize>().unwrap()).collect();

        let op_content: Vec<String> = text[2].replace("  Operation: new = ", "").split(" ").map(|s| s.to_string()).collect();
        let test_modulo = text[3].replace("  Test: divisible by ", "").parse::<isize>().unwrap();

        let true_target_id = text[4].replace("    If true: throw to monkey ", "").parse::<usize>().unwrap();
        let false_target_id = text[5].replace("    If false: throw to monkey ", "").parse::<usize>().unwrap();

        return Monkey {
            id,
            items,
            operation: (op_content[0].clone(), op_content[1].chars().nth(0).unwrap(), op_content[2].clone()),
            test_modulo,
            division_constant,
            true_target_id,
            false_target_id,
            inspections: 0
        };
    }

    pub fn run_round(&mut self, divide_by_three: bool) -> Vec<(usize, isize)> {
        let mut result: Vec<(usize, isize)> = vec![];

        for item in &self.items {
            let item = item % self.division_constant;

            let op_left = if self.operation.0 == "old" { item } else { self.operation.0.parse::<isize>().unwrap() };
            let op_right = if self.operation.2 == "old" { item } else { self.operation.2.parse::<isize>().unwrap() };
            let op_result = match self.operation.1 {
                '+' => op_left + op_right,
                '*' => op_left * op_right,
                def => panic!("No such operation exists: {}", def)
            };

            let worry = if divide_by_three {
                op_result / 3
            } else {
                op_result % self.division_constant
            };

            if worry % self.test_modulo == 0 {
                result.push((self.true_target_id, worry));
            } else {
                result.push((self.false_target_id, worry));
            }

            self.inspections += 1;
        }

        self.items.clear();
        return result;
    }

    pub fn accept_item(&mut self, item: isize) {
        self.items.push(item);
    }
}
