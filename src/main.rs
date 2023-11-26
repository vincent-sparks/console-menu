use console_menu::{Menu, MenuItem};

fn main() {
    let inner_menu_items = vec![
        MenuItem::new("inner item 1", || println!("selected inner option 1")),
        MenuItem::new("inner item 2", || println!("selected inner option 2")),
        MenuItem::new("inner item 3", || println!("selected inner option 3")),
    ];
    let mut inner_menu = Menu::new(inner_menu_items, true);
    inner_menu.set_title("Inner menu");
    inner_menu.set_bg(208);
    inner_menu.set_fg(233);

    let menu_items = vec![
        MenuItem::new("display inner menu", move || inner_menu.show()),
        MenuItem::new("some menu item", || println!("selected option two")),
        MenuItem::new("another option", || println!("selected option three")),
    ];
    
    let mut menu = Menu::new(menu_items, true);
    menu.set_title("Example Menu");
    menu.set_message("a helpful tip");
    menu.set_bg(32);
    menu.set_fg(233);
    menu.set_msg_color(235);
    menu.show();
}
