# Pretty Menu

A simple yet powerful library for creating beautiful console menus in rust.


## Usage

To get started, create a Menu object and pass it a list of options. A simple example:

```rust
use pretty_menu::{Menu, MenuItem};

let menu_items = vec![
    MenuItem::new("option 1", || println!("option one!")),
    MenuItem::new("option 2", || println!("option two!")),
    MenuItem::new("option 3", || println!("option three!")),
];
let mut menu = Menu::new(menu_items, true);
menu.show();
```

<img width="893" alt="Screen Shot 2023-11-26 at 6 09 02 PM" src="https://github.com/Bdeering1/console-menu/assets/55864293/aab7d039-a83a-40e0-9c78-93817df0b819">


