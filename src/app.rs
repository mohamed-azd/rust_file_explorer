use std::path::PathBuf;
use ratatui::widgets::ListState;

pub struct App {
    pub current_dir: PathBuf,
    pub items: Vec<Item>,
    pub state: ListState,
    pub selected_item: usize,
}

pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub path: PathBuf,
}

pub enum ItemType {
    Directory,
    File
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            items: Vec::new(),
            state: ListState::default(),
            selected_item: 0,
        };

        app.load_directory();
        app
    }

    pub fn load_directory(&mut self) {
        self.items.clear();

        if let Ok(result) = std::fs::read_dir(&self.current_dir) {
            for entry in result {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        let item = Item {
                            name: entry.file_name().into_string().unwrap(),
                            item_type: if file_type.is_dir() { ItemType::Directory } else { ItemType::File },
                            path: entry.path(),
                        };
                        self.items.push(item);
                    }
                }
            }
        }

        if self.items.len() > 0 {
            self.set_selected_item(0);
        }
    }

    pub fn set_selected_item(&mut self, selected_item: usize) {
        self.selected_item = selected_item;
        self.state.select(Some(selected_item));
    }

    pub fn select_next_item(&mut self) {
        if self.selected_item + 1 > self.items.len() -1 {
            self.set_selected_item(0);
        } else {
            self.set_selected_item(self.selected_item + 1);
        }
    }

    pub fn select_previous_item(&mut self) {
        if self.selected_item > 0 {
            self.set_selected_item(self.selected_item - 1);
        } else {
            self.set_selected_item(self.items.len() - 1);
        }
    }

    pub fn open_directory(&mut self) {
        if let Some(item) = self.items.get(self.selected_item) {
            if matches!(item.item_type, ItemType::Directory) {
                self.current_dir = item.path.clone();
                self.load_directory()
            }
        }
    }

    pub fn go_to_parent_directory(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.load_directory()
        }
    }
}

impl Item {
    pub fn new() -> Self {
        Item {
            name: String::new(),
            item_type: ItemType::Directory,
            path: PathBuf::new(),
        }
    }
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Directory => "directory",
            ItemType::File => "file",
        }
    }
}