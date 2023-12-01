# Console Menu

A simple yet powerful library for creating beautiful console menus in rust.


## Usage

To get started, create a Menu object and pass it a list of MenuOptions. A simple example:

```rust
use pretty_menu::{Menu, MenuOption, MenuProps};

let menu_options = vec![
    MenuOption::new("option 1", || println!("option one!")),
    MenuOption::new("option 2", || println!("option two!")),
    MenuOption::new("option 3", || println!("option three!")),
];
let mut menu = Menu::new(menu_options, MenuProps::default());
menu.show();
```
<img width="893" alt="Screen Shot 2023-11-26 at 6 09 02 PM" src="https://github.com/Bdeering1/console-menu/assets/55864293/aab7d039-a83a-40e0-9c78-93817df0b819">

Menus can include a title, subtext, and any combination of [8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) colored backgrounds and text.

```rust
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
```
<img width="894" alt="Screen Shot 2023-11-29 at 12 56 45 PM" src="https://github.com/Bdeering1/console-menu/assets/55864293/f7e65fa2-4f9b-419f-b812-fa9ca32e46bd">

Menus can be nested, and options can include any type of callback. Please refer to the [docs](https://docs.rs/pretty-menu/0.1.1/console_menu/) for more information.
