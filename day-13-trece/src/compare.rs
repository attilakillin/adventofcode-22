use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Packet {
    Integer(isize),
    Array(Box<Vec<Packet>>)
}

impl Packet {
    pub fn new(value: &str) -> Packet {
        return Self::read_next(value).0;
    }

    fn read_next(value: &str) -> (Packet, &str) {
        if value.starts_with('[') {
            let mut content: Box<Vec<Packet>> = Box::new(vec![]);

            let mut value = &value[1..];
            while !value.starts_with(']') {
                let (inner, next) = Packet::read_next(value);
                content.push(inner);

                match next.chars().next().unwrap() {
                    ',' => value = &next[1..],
                    ']' => { value = next; break; },
                    any => panic!("Illegal separator char detected: {}", any)
                }
            }

            return (Packet::Array(content), &value[1..]);
        } else {
            let content = value.chars().take_while(|c| c.is_numeric()).collect::<String>().parse::<isize>().unwrap();
            let digits = (content / 10 + 1) as usize;
            return (Packet::Integer(content), &value[digits..]);
        }
    }

    fn wrap(value: &Packet) -> Self {
        if let Packet::Integer(content) = value {
            return Packet::Array(Box::new(vec![Packet::Integer(content.clone())]));
        } else {
            panic!("Can't wrap an array!");
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        if let (Packet::Integer(left), Packet::Integer(right)) = (self, other) {
            return left.cmp(right);
        };

        if let (Packet::Array(left), Packet::Array(right)) = (self, other) {
            let mut left_iter = left.iter();
            let mut right_iter = right.iter();

            let (mut lnext, mut rnext) = (left_iter.next(), right_iter.next());
            while let (Some(lval), Some(rval)) = (lnext, rnext) {
                let result = lval.cmp(rval);
                if result.is_ne() {
                    return result;
                }

                (lnext, rnext) = (left_iter.next(), right_iter.next());
            }

            return if let Some(_) = lnext {
                Ordering::Greater
            } else if let Some(_) = rnext {
                Ordering::Less
            } else {
                Ordering::Equal
            };
        };

        if let (Packet::Integer(_), Packet::Array(_)) = (self, other) {
            return Packet::wrap(self).cmp(other);
        }

        if let (Packet::Array(_), Packet::Integer(_)) = (self, other) {
            return self.cmp(&Packet::wrap(other));
        }

        panic!("No such scenario should occur!");
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Eq for Packet {}
