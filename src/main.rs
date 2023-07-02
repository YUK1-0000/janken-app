#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

use libui::{controls::*, prelude::*};
use rand::Rng;

const HAND_TYPES: [&str;3] = ["Rock", "Paper", "Scissors"];


fn main() {
    let ui = UI::init()
        .expect("Couldn't initialize UI library");

    let mut window = Window::new(&ui, "Rock Paper Scissors", 300, 100, 
        WindowType::NoMenubar);


    let mut layout = VerticalBox::new();
    let mut messages = VerticalBox::new();
    let mut buttons = HorizontalBox::new();

    let mut player = "";
    let mut cpu = "";

    let title_lbl = Label::new("Rock Paper Scissors");
    let player_lbl = Label::new("");
    let cpu_lbl = Label::new("");

    let mut rock_btn = Button::new("Rock");
    let mut paper_btn = Button::new("Paper");
    let mut scissors_btn = Button::new("Scissors");

    let mut player_lbl_cln_1 = player_lbl.clone();
    let mut player_lbl_cln_2 = player_lbl.clone();
    let mut player_lbl_cln_3 = player_lbl.clone();
    let mut cpu_lbl_cln_1 = cpu_lbl.clone();
    let mut cpu_lbl_cln_2 = cpu_lbl.clone();
    let mut cpu_lbl_cln_3 = cpu_lbl.clone();


    rock_btn.on_clicked(move |rock_btn| {
        player = HAND_TYPES[0];
        cpu = HAND_TYPES[rnd()];
        player_lbl_cln_1.set_text(player);
        cpu_lbl_cln_1.set_text(cpu);

    });
    paper_btn.on_clicked(move |paper_btn| {
        player = HAND_TYPES[1];
        cpu = HAND_TYPES[rnd()];
        player_lbl_cln_2.set_text(player);
        cpu_lbl_cln_2.set_text(cpu);
    });
    scissors_btn.on_clicked(move |scissors_btn| {
        player = HAND_TYPES[2];
        cpu = HAND_TYPES[rnd()];
        player_lbl_cln_3.set_text(player);
        cpu_lbl_cln_3.set_text(cpu);
    });


    for control in [title_lbl, player_lbl, cpu_lbl] {
        messages.append(control, LayoutStrategy::Stretchy
        );
    }

    for child in [rock_btn, paper_btn, scissors_btn] {
        buttons.append(child, LayoutStrategy::Stretchy);
    };

    layout.append(messages, LayoutStrategy::Stretchy);
    layout.append(buttons, LayoutStrategy::Stretchy);

    window.set_child(layout);
    window.show();
    ui.main();
}

fn rnd() -> usize {
    loop {
        let tmp = (rand::thread_rng().gen::<f32>() * 3.).floor() as i8;
        if tmp != 3 { break tmp as usize; };
    }
}