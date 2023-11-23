use console_menu::{Menu, MenuItem};

fn main() {
    let mut menu_items = vec![];
    menu_items.push(MenuItem::new("example item 1", || println!("item one!")));
    menu_items.push(MenuItem::new("example item 2", || println!("item two!")));
    
    let mut menu = Menu::new(menu_items, true);
    menu.title("Example menu");
    menu.show();
}
