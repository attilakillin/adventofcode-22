use std::{cell::RefCell, rc::Rc};

pub trait FilesystemElement {
    fn name(&self) -> &str;
    fn size(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct Folder {
    name: String,
    folders: Vec<Rc<RefCell<Folder>>>,
    files: Vec<File>
}

#[derive(Clone, Debug)]
pub struct File {
    name: String,
    size: usize
}

impl Folder {
    pub fn new(name: &str) -> Self {
        return Folder { name: name.to_string(), folders: vec![], files: vec![] };
    }

    pub fn push_new_file(&mut self, name: &str, size: usize) {
        self.files.push(File::new(name, size));
    }

    pub fn push_new_folder(&mut self, name: &str) {
        self.folders.push(Rc::new(RefCell::new(Folder::new(name))));
    }

    pub fn navigate(&self, name: &str) -> Rc<RefCell<Folder>> {
        return self.folders.iter().find(|e| e.borrow().name() == name).unwrap().clone();
    }

    pub fn visit_folders<F: FnMut(&Folder)>(&self, visitor: &mut F) {
        self.folders.iter().for_each(|f| f.borrow().visit_folders(visitor));
        visitor(self);
    }
}

impl FilesystemElement for Folder {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn size(&self) -> usize {
        return self.folders.iter().map(|e| e.borrow().size()).sum::<usize>()
             + self.files.iter().map(|e| e.size()).sum::<usize>();
    }
}

impl File {
    pub fn new(name: &str, size: usize) -> Self {
        return File { name: name.to_string(), size };
    }
}

impl FilesystemElement for File {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn size(&self) -> usize {
        return self.size;
    }
}
