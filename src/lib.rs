//! A simple yet powerful library for creating beautiful console menus in rust.
//!
//! Allows for easy creation of interactive console menus. A simple example:
//!
//! ```rust
//! use console_menu::{Menu, MenuOption, MenuProps};
//! 
//! let menu_options = vec![
//!     MenuOption::new("option 1", || println!("option one!")),
//!     MenuOption::new("option 2", || println!("option two!")),
//!     MenuOption::new("option 3", || println!("option three!")),
//! ];
//! let mut menu = Menu::new(menu_options, MenuProps::default());
//! menu.show();
//! ```
//!
//! Menus can include a title, subtext, and any combination of [8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
//! colored backgrounds and text by configuring `MenuProps`. Menus that don't fit the console window are paginated.
//!
//! Menu controls are as follows:
//! 
//! | Key Bind | Action      |
//! | -------- | ----------- |
//! | ↓, ↑, ←, →, h, j, k, l | make selection        |
//! | enter    | confirm     |
//! | esc, q   | exit        |

use console::{Key, Term};

/// Stores configuration data passed to a `Menu` on creation.
///
/// Menus use [8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) colors to ensure
/// widespread terminal support. It should be noted that values from 0-15 will make colors vary
/// based on individual terminal settings.
///
/// Configure a subset of properties using the defaults and struct update syntax:
/// ```rust
/// let props = MenuProps {
///     title: "My Menu",
///     ..MenuProps::default()
/// }
/// ```
pub struct MenuProps<'a> {
    /// Displays above the list of menu options. Pass an empty string for no title.
    pub title: &'a str,
    /// Display below the list of menu options. Pass an empty string for no message.
    pub message: &'a str,
    pub bg_color: u8,
    pub fg_color: u8,
    pub msg_color: u8,
    /// If true, menu will exit immediately upon an option being selected.
    pub exit_on_action: bool,
}

/// ```rust
/// MenuProps {
///     title: "",
///     message: "",
///     bg_color: 8,
///     fg_color: 15,
///     msg_color: 7,
///     exit_on_action: true,
/// }
/// ```
impl Default for MenuProps<'_> {
    fn default() -> MenuProps<'static> {
        MenuProps {
            title: "",
            message: "",
            bg_color: 8,
            fg_color: 15,
            msg_color: 7,
            exit_on_action: true,
        }
    }
}

/// An element in a `Menu`.
///
/// Consists of a label and a callback. Callbacks can be any function, including functions that
/// call nested menus:
///
/// ```rust
/// let mut nested_menu = Menu::new(vec![], MenuProps::default());
/// let show_nested = MenuOption::new("show nested menu", move || nested_menu.show());
/// ```

pub struct MenuOption {
    pub label: String,
    pub action: Box<dyn FnMut()>,
}

impl MenuOption {
    pub fn new(label: &str, action: impl FnMut() + 'static) -> Self {
        Self {
            label: label.to_owned(),
            action: Box::new(action),
        }
    }
}

/// ```rust
/// MenuOption::new("exit", || {})
/// ```
impl Default for MenuOption {
    fn default() -> MenuOption {
        MenuOption::new("exit", || {})
    }
}

/// Interactive console menu.
///
/// Create a menu by passing it a list of `MenuOption` and a `MenuProps`. Display using`.show()`.
///
/// ```rust
/// let menu_options = vec![
///     MenuOption::new("option 1", || println!("option one!")),
///     MenuOption::new("option 2", || println!("option two!")),
///     MenuOption::new("option 3", || println!("option three!")),
/// ];
/// let mut menu = Menu::new(menu_options, MenuProps::default());
/// menu.show();
/// ```
pub struct Menu {
    items: Vec<MenuOption>,
    title: Option<String>,
    message: Option<String>,
    bg_color: u8,
    fg_color: u8,
    msg_color: u8,
    exit_on_action: bool,
    selected_item: usize,
    selected_page: usize,
    items_per_page: usize,
    num_pages: usize,
    page_start: usize,
    page_end: usize,
    max_width: usize,
}

impl Menu {
    pub fn new(items: Vec<MenuOption>, props: MenuProps) -> Self {
        let mut items = items;
        if items.len() == 0 { items.push(MenuOption::default()) }

        let items_per_page: usize = (Term::stdout().size().0 - 6) as usize;
        let items_per_page = clamp(items_per_page, 1, items.len());
        let num_pages = ((items.len() - 1) / items_per_page) + 1;

        let mut max_width = (&items).iter().fold(0, |max, item| {
            let label_len = item.label.len();
            if label_len > max { label_len } else { max }
        });
        if props.title.len() > max_width {
            max_width = props.title.len()
        }
        if props.message.len() > max_width {
            max_width = props.message.len()
        }  

        let mut menu = Self {
            items,
            title: if props.title.len() > 0 {
                Some(props.title.to_owned())
            } else {
                None
            },
            message: if props.message.len() > 0 {
                Some(props.message.to_owned())
            } else {
                None
            },
            bg_color: props.bg_color,
            fg_color: props.fg_color,
            msg_color: props.msg_color,
            exit_on_action: props.exit_on_action,
            selected_item: 0,
            selected_page: 0,
            items_per_page,
            num_pages,
            page_start: 0,
            page_end: 0,
            max_width,
        };
        menu.set_page(0);
        menu
    }

    pub fn show(&mut self) {
        let stdout = Term::buffered_stdout();

        stdout.hide_cursor().unwrap();
        stdout.clear_screen().unwrap();

        self.draw(&stdout);
        self.run_navigation(&stdout);
    }

    fn run_navigation(&mut self, stdout: &Term) {
        loop {
            let key = stdout.read_key().unwrap();

            match key {
                Key::ArrowUp | Key::Char('k') => {
                    if self.selected_item != self.page_start {
                        self.selected_item -= 1;
                    } else if self.selected_page != 0 {
                        self.set_page(self.selected_page - 1);
                        self.selected_item = self.page_end;
                    }
                }
                Key::ArrowDown | Key::Char('j') => {
                    if self.selected_item < self.page_end {
                        self.selected_item += 1
                    } else if self.selected_page < self.num_pages - 1 {
                        self.set_page(self.selected_page + 1);
                    }
                }
                Key::ArrowLeft | Key::Char('h') | Key::Char('b') => {
                    if self.selected_page != 0 {
                        self.set_page(self.selected_page - 1);
                    }
                }
                Key::ArrowRight | Key::Char('l') | Key::Char('w') => {
                    if self.selected_page < self.num_pages - 1 {
                        self.set_page(self.selected_page + 1);
                    }
                }
                Key::Escape | Key::Char('q') | Key::Backspace => {
                    self.exit(stdout);
                    break;
                }
                Key::Enter => {
                    if self.exit_on_action {
                        self.exit(stdout);
                        (self.items[self.selected_item].action)();
                        break;
                    } else {
                        (self.items[self.selected_item].action)();
                    }    
                }
                _ => {}
            }

            self.draw(stdout);
        }
    }

    fn set_page(&mut self, page: usize) {
        self.selected_page = page;
        self.page_start = self.selected_page * self.items_per_page;
        self.selected_item = self.page_start;
        if self.items.len() > self.page_start + self.items_per_page {
            self.page_end = self.page_start + self.items_per_page - 1
        } else {
            self.page_end = self.items.len() - 1
        }
    }

    fn draw(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();

        let menu_width = self.max_width;
        let mut extra_lines = 2;
        if let Some(_) = self.title {
           extra_lines += 2; 
        }
        if let Some(_) = self.message {
            extra_lines += 1;
        }

        let indent: usize = (stdout.size().1 / 2) as usize - ((menu_width + 4) / 2);
        let indent_str = pad_left("".to_string(), indent);

        let vertical_pad: usize = (stdout.size().0 / 2) as usize  - ((self.items_per_page + extra_lines) / 2);
        stdout.write_str(&format!("{:\n<width$}", "", width=vertical_pad)).unwrap();

        stdout.write_str(&format!("\x1b[38;5;{}m", self.fg_color)).unwrap();
        stdout.write_line(&format!("{}{}", indent_str, self.apply_bg("", menu_width))).unwrap();

        let mut ansi_width = 18;
        if let Some(title) = &self.title {
            let title_str = format!("\x1b[4m\x1b[1m{}\x1b[22m\x1b[24m", title);
            stdout.write_line(&format!("{}{}", indent_str, self.apply_bg(&title_str, menu_width + ansi_width))).unwrap();
            stdout.write_line(&format!("{}{}", indent_str, self.apply_bg("", menu_width))).unwrap();
        } 

        for (i, option) in self.items[self.page_start..=self.page_end].iter().enumerate() {
            let selected_item_str = if self.page_start + i == self.selected_item {
                ansi_width = 9;
                format!("\x1b[1m{}\x1b[22m", option.label)
            } else {
                ansi_width = 0;
                format!("{}", option.label)
            };
            stdout.write_line(&format!("{}{}", indent_str, self.apply_bg(&selected_item_str, menu_width + ansi_width))).unwrap();
        }

        if self.num_pages > 1 {
            stdout.write_line(&format!("{}{}", indent_str, self.apply_bg(&format!("Page {} of {}", self.selected_page + 1, self.num_pages), menu_width))).unwrap();
        }
        if let Some(message) = &self.message {
            stdout.write_line(&format!("{}{}", indent_str, self.apply_bg("", menu_width))).unwrap();
            stdout.write_line(&format!("{}\x1b[38;5;{}m{}\x1b[38;5;{}m", indent_str, self.msg_color, self.apply_bg(message, menu_width), self.fg_color)).unwrap();
        }

        stdout.write_line(&format!("{}{}", indent_str, self.apply_bg("", menu_width))).unwrap();
        stdout.write_str("\x1b[39m").unwrap();

        stdout.flush().unwrap();
    }

    fn apply_bg(&self, s: &str, width: usize) -> String {
        format!("\x1b[48;5;{}m{}\x1b[49m", self.bg_color, pad_right(format!("  {}", s), width + 4)) 
    }

    fn exit(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();
        stdout.show_cursor().unwrap();
        stdout.flush().unwrap();
    }
}

fn pad_left(s: String, width: usize) -> String {
    format!("{: >width$}", s, width=width)
}

fn pad_right(s: String, width: usize) -> String {
    format!("{: <width$}", s, width=width)
}

fn clamp(num: usize, min: usize, max: usize) -> usize {
    let out = if num < min { min } else { num };
    if out > max { max } else { out }
}
