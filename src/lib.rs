use console::{Key, Style, Term};

pub struct MenuItem {
    pub label: String,
    pub action: Box<dyn FnMut()>,
}

impl MenuItem {
    pub fn new(label: &str, action: impl FnMut() + 'static) -> Self {
        Self {
            label: label.to_owned(),
            action: Box::new(action),
        }
    }
}

pub struct Menu {
    title: Option<String>,
    items: Vec<MenuItem>,
    message: Option<String>,
    exit_on_action: bool,
    selected_item: usize,
    selected_page: usize,
    items_per_page: usize,
    num_pages: usize,
    page_start: usize,
    page_end: usize,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, exit_on_action: bool) -> Self {
        let mut items_per_page: i32 = Term::stdout().size().0 as i32 - 6;
        if items_per_page < 1 { items_per_page = 1 }
        let items_per_page = items_per_page as usize;
        let num_pages = ((items.len() - 1) / items_per_page) + 1;

        let mut menu = Self {
            title: None,
            items,
            message: None,
            exit_on_action,
            selected_item: 0,
            selected_page: 0,
            items_per_page,
            num_pages,
            page_start: 0,
            page_end: 0,
        };
        menu.set_page(0);
        menu
    }

    pub fn title(&mut self, title: &str) {
        self.title = Some(title.to_owned());
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
                Key::Escape | Key::Char('q') => {
                    self.exit(stdout);
                    break;
                }
                Key::Enter | Key::Del => {
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

        if let Some(title) = &self.title {
            let controls_style = Style::new().dim();
            stdout.write_line(&format!("{}", controls_style.apply_to("  ↓,↑,←,→: select |  enter: run action |  q: quit\n"))).unwrap();
            let title_style = Style::new().bold();
            stdout.write_line(&format!("  {}", title_style.apply_to(title))).unwrap();
        } 

        for (i, option) in self.items[self.page_start..=self.page_end].iter().enumerate() {
            if self.page_start + i == self.selected_item {
                let style = Style::new().bold();
                stdout.write_line(&format!("> {}", style.apply_to(&option.label))).unwrap();
            } else {
                stdout.write_line(&format!("  {}", option.label)).unwrap();
            }
        }
        stdout.write_line(&format!("Page {} of {}", self.selected_page + 1, self.num_pages)).unwrap();

        if let Some(message) = &self.message {
            let style = Style::new().red();
            stdout.write_line(&format!("\n{}", style.apply_to(message))).unwrap();
        }

        stdout.flush().unwrap();
    }

    fn exit(&self, stdout: &Term) {
        stdout.clear_screen().unwrap();
        stdout.show_cursor().unwrap();
        stdout.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
