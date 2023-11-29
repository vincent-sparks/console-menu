use console_menu::{Menu, MenuOption};

fn main() {
    //let menu_items = vec![
    //    MenuOption::new("option 1", || println!("option one!")),
    //    MenuOption::new("option 2", || println!("option two!")),
    //    MenuOption::new("option 3", || println!("option three!")),
    //];
    //let mut menu = Menu::new(menu_items, true);
    //menu.show();

    let inner_menu_items = vec![
        MenuOption::new("inner item 1", || println!("selected inner option 1")),
        MenuOption::new("inner item 2", || println!("selected inner option 2")),
        MenuOption::new("inner item 3", || println!("selected inner option 3")),
    ];
    let mut inner_menu = Menu::new(inner_menu_items, true);
    inner_menu.set_title("Inner menu");
    inner_menu.set_bg(208);
    inner_menu.set_fg(233);

    let menu_items = vec![
        MenuOption::new("eggs", || println!("menu item one!")),
        MenuOption::new("bacon", || println!("menu item two!")),
        MenuOption::new("toast", || println!("menu item three!")),
    ];

    let mut menu = Menu::new(menu_items, true);
    menu.set_title("My Breakfast Menu");
    menu.set_message("*coffee is free!");
    menu.set_bg(32);
    menu.set_fg(233);
    menu.set_msg_color(236);
    menu.show();
}
