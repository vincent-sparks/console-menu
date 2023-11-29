use pretty_menu::{Menu, MenuOption, MenuProps};

fn main() {
    let menu_options = vec![
        MenuOption::new("eggs", || println!("menu item one!")),
        MenuOption::new("bacon", || println!("menu item two!")),
        MenuOption::new("toast", || println!("menu item three!")),
    ];
    
    let mut menu = Menu::new(menu_options, MenuProps {
        title: "My Breakfast Menu",
        message: "*coffee is free!",
        fg_color: 233,
        bg_color: 32,
        msg_color: 236,
        ..MenuProps::default()
    });
    menu.show();

    // let mut menu = Menu::new(menu_options, true);
    // menu.set_title("My Breakfast Menu");
    // menu.set_message("*coffee is free!");
    // menu.set_bg(32);
    // menu.set_fg(233);
    // menu.set_msg_color(236);
    // menu.show();
}
