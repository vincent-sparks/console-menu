# Console Menu

A simple yet powerful library for creating beautiful console menus in rust.


## Usage

To get started, create a Menu object and pass it a list of MenuOptions. Each option consists of a label and a callback. A simple example:

```rust
use console_menu::{Menu, MenuOption, MenuProps};

let menu_options = vec![
    MenuOption::new("option 1", || println!("option one!")),
    MenuOption::new("option 2", || println!("option two!")),
    MenuOption::new("option 3", || println!("option three!")),
];
let mut menu = Menu::new(menu_options, MenuProps::default());
menu.show();
```
<img width="893" alt="Screen Shot 2023-11-26 at 6 09 02 PM" src="https://github.com/Bdeering1/console-menu/assets/55864293/aab7d039-a83a-40e0-9c78-93817df0b819">

Menus are controlled using the arrow keys to move around, enter to select an option, and escape to exit. Vim style keybindings are also supported. Menus can include a title, footer message, and any combination of [8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) colored backgrounds and text. Color constants are also available to simplify theming.

```rust
use console_menu::{color, Menu, MenuOption, MenuProps};

let menu_options = vec![
    MenuOption::new("eggs", || println!("eggs coming right up!")),
    MenuOption::new("bacon", || println!("bacon it is!")),
    MenuOption::new("toast", || println!("sorry, we're out of toast")),
];

let mut menu = Menu::new(menu_options, MenuProps {
    title: "My Breakfast Menu",
    message: "*coffee is free!",
    fg_color: color::BLACK,
    bg_color: color::BLUE,
    msg_color: Some(color::DARK_GRAY),
    ..MenuProps::default()
});
menu.show();
```
<img width="894" alt="Screen Shot 2023-11-29 at 12 56 45 PM" src="https://github.com/Bdeering1/console-menu/assets/55864293/f7e65fa2-4f9b-419f-b812-fa9ca32e46bd">

Menus can be nested, and options can include any type of callback. Please refer to the [docs](https://docs.rs/console-menu/) for more information.
