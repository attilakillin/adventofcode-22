pub struct Node {
    pub value: char,
    pub x: usize,
    pub y: usize,

    pub children: Vec<Box<Node>>,
    pub depth: usize
}

impl Node {
    pub fn new(value: char, x: usize, y: usize, depth: usize) -> Self {
        return Node { value, x, y, children: vec![], depth };
    }

    pub fn append(&mut self, node: Node) {
        if self.depth + 1 != node.depth {
            panic!("Node to append has wrong depth! ({} instead of {})", node.depth, self.depth + 1);
        };
        self.children.push(Box::new(node));
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        return (self.x == x && self.y == y) || self.children.iter().any(|c| c.contains(x, y));
    }

    pub fn find(&mut self, x: usize, y: usize) -> Option<&mut Node> {
        if self.x == x && self.y == y {
            return Some(self);
        } else {
            for child in &mut self.children {
                let result = child.find(x, y);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }
    }

    pub fn find_by_value(&self, value: char) -> Option<&Node> {
        if self.value == value {
            return Some(self);
        } else {
            for child in &self.children {
                let result = child.find_by_value(value);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }
    }

    pub fn find_depth_of_closest(&self, value: char) -> Option<usize> {
        if self.value == value {
            return Some(0);
        } else {
            let mut closest: Option<usize> = None;
            for child in &self.children {
                if let Some(distance) = child.find_depth_of_closest(value) {
                    if closest.is_none() || distance + 1 < closest.unwrap() {
                        closest = Some(distance + 1);
                    }
                }
            }

            return closest;
        }
    }
}
