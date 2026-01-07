use std::path::PathBuf;
use crossterm::terminal::size;
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
    pub preview: String,
}

pub enum ItemType {
    Directory,
    File,
    Image,
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
                    let item = Item {
                        name: entry.file_name().into_string().unwrap(),
                        item_type: ItemType::get_item_type(&entry),
                        path: entry.path(),
                        preview: String::new(),
                    };
                    self.items.push(item);
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
        self.load_current_selection()
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

    fn load_current_selection(&mut self) {
        let image_width = self.get_available_preview_width();

        if let Some(item) = self.items.get_mut(self.selected_item) {
            if matches!(item.item_type, ItemType::Image) {
                match image::open(&item.path) {
                    Ok(img) => {
                        item.preview = image_2_ascii::convert(&img, image_width);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    fn get_available_preview_width(&mut self) -> u32 {
        if let Ok(size) = size() {
            (size.0 as f32 * 0.50) as u32
        } else {
            100
        }
    }
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Directory => "directory",
            ItemType::File => "file",
            ItemType::Image => "image",
        }
    }

    pub fn get_item_type(entry: &std::fs::DirEntry) -> ItemType {
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            return ItemType::Directory
        }

        if let Some(extension) = entry.path().extension() {
            if let Some(extension_str) = extension.to_str() {
                match extension_str.to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" | "gif" | "webp" | "ico"  => return ItemType::Image,
                    _ => {}
                }
            }
        }
        ItemType::File
    }
}