use std::{slice::Iter, cell::RefCell, rc::Rc};

use crate::types::Folder;

pub struct Reader<'a> {
    iterator: Iter<'a, String>
}

impl<'a> Reader<'a> {
    pub fn new(iterator: Iter<'a, String>) -> Self {
        return Reader { iterator };
    }

    pub fn parse(&mut self) -> Folder {
        let root = Rc::new(RefCell::new(Folder::new("/")));
        let mut active = Rc::clone(&root);

        let mut stack: Vec<Rc<RefCell<Folder>>> = vec![];

        while let Some(line) = self.iterator.next() {
            let words: Vec<&str> = line.split_whitespace().collect();

            if words[0] == "$" {
                // We only do things with the cd command, but the ls command takes this path too.
                if words[1] == "cd" {
                    match words[2] {
                        "/" => {
                            active = Rc::clone(&root);
                            stack.clear();
                        },
                        ".." => {
                            active = stack.pop().unwrap();
                        },
                        name => {
                            stack.push(Rc::clone(&active));
                            let temp = (*active).borrow().navigate(name);
                            active = temp;
                        }
                    }
                }
            } else {
                // Else the only possible thing is the output of an ls command.
                if words[0] == "dir" {
                    (*active).borrow_mut().push_new_folder(words[1]);
                } else {
                    (*active).borrow_mut().push_new_file(words[1], words[0].parse::<usize>().unwrap());
                }
            }
        }

        return ((*root).borrow()).clone();
    }
}
