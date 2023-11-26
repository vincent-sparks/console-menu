use console_menu::{Menu, MenuItem};

fn main() {
    let inner_menu_items = vec![
        MenuItem::new("inner item 1", || println!("selected inner item 1")),
        MenuItem::new("inner item 2", || println!("selected inner item 2")),
        MenuItem::new("inner item 3", || println!("selected inner item 3")),
    ];
    let mut inner_menu = Menu::new(inner_menu_items, true);
    inner_menu.set_title("Inner menu");

    let menu_items = vec![
        MenuItem::new("display inner menu", move || inner_menu.show()),
        MenuItem::new("some menu item", || println!("item two!")),
        MenuItem::new("another option", || println!("item three!")),
    ];
    
    let mut menu = Menu::new(menu_items, false);
    menu.set_title("Example menu");
    menu.set_bg(208);
    menu.set_fg(16);
    menu.show();
}
