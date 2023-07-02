#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

use libui::{controls::*, prelude::*};
use rand::Rng;

const HAND_TYPES: [&str;4] = ["グー", "パー", "チョキ", "グー"];
const RESULT_TYPES: [&str;3] = ["勝ち", "負け", "あいこ"];


fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");

    let mut window = Window::new(
        &ui, "じゃんけん", 300, 100, WindowType::NoMenubar
    );

    let mut layout = VerticalBox::new();
    let mut messages = VerticalBox::new();
    let mut buttons = HorizontalBox::new();

    let mut rock_btn = Button::new("グー");
    let mut paper_btn = Button::new("パー");
    let mut scissors_btn = Button::new("チョキ");

    let player_lbl = Label::new("あなた:");
    let mut player_lbl_cln_1 = player_lbl.clone();
    let mut player_lbl_cln_2 = player_lbl.clone();
    let mut player_lbl_cln_3 = player_lbl.clone();

    let cpu_lbl = Label::new("CPU:");
    let mut cpu_lbl_cln_1 = cpu_lbl.clone();
    let mut cpu_lbl_cln_2 = cpu_lbl.clone();
    let mut cpu_lbl_cln_3 = cpu_lbl.clone();

    let result_lbl = Label::new("");
    let mut result_lbl_cln_1 = result_lbl.clone();
    let mut result_lbl_cln_2 = result_lbl.clone();
    let mut result_lbl_cln_3 = result_lbl.clone();


    rock_btn.on_clicked(move |_rock_btn| {
        let i = 0;
        let j = rand_idx();
        player_lbl_cln_1.set_text(&format!("{} {}", "あなた:", HAND_TYPES[i]));
        cpu_lbl_cln_1.set_text(&format!("{} {}", "CPU:", HAND_TYPES[j]));
        result_lbl_cln_1.set_text(result(i, j))
    });

    paper_btn.on_clicked(move |_paper_btn| {
        let i = 1;
        let j = rand_idx();
        player_lbl_cln_2.set_text(&format!("{} {}", "あなた:", HAND_TYPES[i]));
        cpu_lbl_cln_2.set_text(&format!("{} {}", "CPU:", HAND_TYPES[j]));
        result_lbl_cln_2.set_text(result(i, j))
    });

    scissors_btn.on_clicked(move |_scissors_btn| {
        let i = 2;
        let j = rand_idx();
        player_lbl_cln_3.set_text(&format!("{} {}", "あなた:", HAND_TYPES[i]));
        cpu_lbl_cln_3.set_text(&format!("{} {}", "CPU:", HAND_TYPES[j]));
        result_lbl_cln_3.set_text(result(i, j))
    });
    

    for control in [player_lbl, cpu_lbl, result_lbl] {
        messages.append(control, LayoutStrategy::Stretchy);
    };

    for child in [rock_btn, scissors_btn, paper_btn] {
        buttons.append(child, LayoutStrategy::Stretchy);
    };

    layout.append(messages, LayoutStrategy::Stretchy);
    layout.append(buttons, LayoutStrategy::Stretchy);

    window.set_child(layout);
    window.show();
    ui.main();
}

fn rand_idx() -> usize {
    loop {
        let i = (rand::thread_rng().gen::<f32>() * 3.).floor();
        if i != 3. {
            break i as usize
        };
    }
}

fn result(player_idx: usize, cpu_idx: usize) -> &'static str {
    if HAND_TYPES[cpu_idx + 1] == HAND_TYPES[player_idx] { RESULT_TYPES[0] }
    else if HAND_TYPES[player_idx + 1] == HAND_TYPES[cpu_idx] { RESULT_TYPES[1] }
    else { RESULT_TYPES[2] }     
}
