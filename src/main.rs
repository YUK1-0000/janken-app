#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

use std::{ cell::RefCell, rc::Rc };

use libui::{controls::*, prelude::*};
use rand::Rng;

const HAND_TYPES: [&str;4] = ["グー", "パー", "チョキ", "グー"];
const RESULT_TYPES: [&str;3] = ["勝ち", "負け", "あいこ"];


struct Janken {
    win: f32,
    lose: f32
}


impl Janken {
    fn new() -> Self {
        Self { win: 0., lose: 0. }
    }

    fn rand_idx(&self) -> usize {
        loop {
            let i = (rand::thread_rng().gen::<f32>() * 3.).floor();
            if i != 3. {
                break i as usize
            };
        }
    }

    fn result(&mut self, player_idx: usize, cpu_idx: usize) -> &'static str {
        if HAND_TYPES[cpu_idx + 1] == HAND_TYPES[player_idx] {
            self.win += 1.;
            RESULT_TYPES[0]
        }
        else if HAND_TYPES[player_idx + 1] == HAND_TYPES[cpu_idx] {
            self.lose += 1.;
            RESULT_TYPES[1]
        }
        else {
            RESULT_TYPES[2]
        }     
    }

    fn on_update(&mut self, i: usize) -> (usize, &'static str) {
        let j = self.rand_idx();
        (j, self.result(i, j))
    }
}


#[derive(Clone)]
struct Labels {
    player: Label,
    cpu: Label,
    result: Label,
    count: Label, 
    win_rate: Label
}


impl Labels {
    fn new() -> Self {
        Self {
            player: Label::new("あなた："), 
            cpu: Label::new("CPU："), 
            result: Label::new(""), 
            count: Label::new("0勝0敗"), 
            win_rate: Label::new("")
        }
    }

    fn update(&mut self, i: usize, j: usize, result: &str, win: f32, lose: f32) {
        self.player.set_text(&format!("あなた：{}", HAND_TYPES[i]));
        self.cpu.set_text(&format!("CPU：{}", HAND_TYPES[j]));
        self.result.set_text(result);
        self.count.set_text(&format!("{}勝{}敗", win, lose));
        self.win_rate.set_text(&format!("勝率{}", win / (win + lose)));
    }
}


fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");

    let mut window = Window::new(
        &ui, "じゃんけん", 300, 100,
        WindowType::NoMenubar
    );

    let mut layout = VerticalBox::new();
    let mut messages = VerticalBox::new();
    let mut buttons = HorizontalBox::new();

    let mut rock_btn = Button::new("グー");
    let mut paper_btn = Button::new("パー");
    let mut scissors_btn = Button::new("チョキ");

    let janken = Rc::new(RefCell::new(Janken::new()));

    let janken_cln_1 = Rc::clone(&janken);
    let janken_cln_2 = Rc::clone(&janken);
    let janken_cln_3 = Rc::clone(&janken);

    let labels = Labels::new();

    let mut labels_cln_1 = labels.clone();
    let mut labels_cln_2 = labels.clone();
    let mut labels_cln_3 = labels.clone();

    rock_btn.on_clicked(move |_rock_btn| {
        update_labels(&janken_cln_1, &mut labels_cln_1, 0);
    });

    paper_btn.on_clicked(move |_paper_btn| {
        update_labels(&janken_cln_2, &mut labels_cln_2, 1);
    });

    scissors_btn.on_clicked(move |_scissors_btn| {
        update_labels(&janken_cln_3, &mut labels_cln_3, 2);
    });
    

    for child in [labels.player, labels.cpu, labels.result, labels.count, labels.win_rate] {
        messages.append(child, LayoutStrategy::Stretchy);
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

fn update_labels(core: &Rc<RefCell<Janken>>, labels: &mut Labels, i: usize) {
    let (j, result) = core.borrow_mut().on_update(i);
    labels.update(i, j, result, core.borrow().win, core.borrow().lose);
}