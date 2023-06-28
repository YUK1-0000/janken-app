#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

use libui::{controls::*, prelude::*};

fn main() {
    let ui = UI::init()
        .expect("Couldn't initialize UI library");
    
    let mut win = Window::new(&ui, "じゃんけん", 300, 200, 
        WindowType::NoMenubar);
    let mut layout = HorizontalBox::new();


    let mut button_a = Button::new("A");
    let mut button_b = Button::new("B");

    let mut cloned_button_b = button_b.clone();
    button_a.on_clicked(move |button_a| {
        button_a.hide();
        cloned_button_b.show();
    });

    let mut cloned_button_a = button_a.clone();
    button_b.on_clicked(move |button_b| {
        button_b.hide();
        cloned_button_a.show();
    });

    layout.append(button_a, LayoutStrategy::Compact);
    layout.append(button_b, LayoutStrategy::Compact);
    

    win.set_child(layout);
    win.show();
    ui.main();
}
