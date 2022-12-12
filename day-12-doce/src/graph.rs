use crate::node::Node;

pub struct BFTreeBuilder<'v> {
    values: &'v Vec<Vec<char>>,
    root: Box<Node>
}

impl<'v> BFTreeBuilder<'v> {
    pub fn new(values: &'v Vec<Vec<char>>) -> Self {
        return Self { values, root: Self::create_root(values) };
    }

    fn create_root(values: &Vec<Vec<char>>) -> Box<Node> {
        let mut x = 0;
        let mut y = 0;
        for (j, row) in values.iter().enumerate() {
            for (i, col) in row.iter().enumerate() {
                if *col == 'E' {
                    x = i;
                    y = j;
                }
            }
        }

        return Box::new(Node::new('E', x, y, 0));
    }

    pub fn root(&self) -> &Node {
        return &self.root;
    }

    pub fn build<'s>(&'s mut self) {
        let mut queue: Vec<(usize, usize)> = vec![(self.root.x, self.root.y)];

        while queue.len() > 0 {
            let head = queue.remove(0);
            self.process(head, &mut queue);
        }      
    }

    fn process<'s>(&'s mut self, node: (usize, usize), queue: &mut Vec<(usize, usize)>) {
        let mut neighbors: Vec<(usize, usize)> = vec![];
        if node.1 < self.values.len() - 1 { neighbors.push((node.0, node.1 + 1)); }
        if node.0 < self.values[0].len() - 1 { neighbors.push((node.0 + 1, node.1)); }
        if node.1 > 0 { neighbors.push((node.0, node.1 - 1)); }
        if node.0 > 0 { neighbors.push((node.0 - 1, node.1)); }

        let current_height = Self::value_to_height(self.values[node.1][node.0]);
        let depth = { self.root.find(node.0, node.1).unwrap().depth };

        for (x, y) in neighbors {
            if self.root.contains(x, y) {
                continue;
            }

            let neighbor_value = self.values[y][x];
            let neighbor_height = Self::value_to_height(neighbor_value);
            
            if neighbor_height + 1 >= current_height {
                self.root.find(node.0, node.1).unwrap().append(Node::new(neighbor_value, x, y, depth + 1));
                queue.push((x, y));
            }
        }
    }

    fn value_to_height(value: char) -> u8 {
        if value == 'S' { return 0; }
        if value == 'E' { return 25; }

        return value as u8 - 'a' as u8;
    }
}
