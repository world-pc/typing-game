use pancurses::Window;
use crate::types::{FallingWord, GameState};
use crate::draw_words;
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::thread::{sleep};
use std::time::Duration;

pub fn title_screen(window: &Window) {
    window.clear();
    
    window.mvprintw(10, 10, "Welcome to my typing game!");
    window.mvprintw(20, 10, "press any key to begin.");
    
    window.refresh();
}

pub fn transition(window: &Window) {

    //possble characters to print
    let possible_chrs: [&str; 6] = ["$", "@", "~", "?", "!", "*"];
    let chosen_chr: &str = possible_chrs[rand::thread_rng().gen_range(0..5)];

    //create an array [1, 2, 3, ... , 50] that we'll shuffle
    //to create randomized layouts for each row.

    let mut x_layout: [i32; 50] = [0; 50];
    for i in 0..x_layout.len() {
        x_layout[i] = (i+1) as i32;
    }

    //row_layouts stores the spawn patterns for rows 1->25
    let mut row_layouts: Vec<Vec<i32>> = (0..25)
        .map(|_| {
            x_layout.shuffle(&mut thread_rng());
            x_layout.to_vec()
        }).collect();

    for col in 0..50 {
        for row in 0..25 {
            window.mvprintw(row+1, row_layouts[row as usize][col as usize], chosen_chr);
        }

        window.refresh();
        sleep(Duration::from_millis(64)); // ~15fps
    }
}

pub fn game_screen(window: &Window, game_state: &GameState) {

    window.clear();

    let player_health = game_state.player_health;
    let score = game_state.player_score;

    window.printw("esc - quit game. enter - submit / refresh ur typing\n");
    window.printw("Type the falling words before they reach the bottom.");
    window.mvprintw(25, 0, "-".repeat(25));
    window.mvprintw(28, 0, format!("Health: {player_health}"));
    window.mvprintw(28, 20, format!("Score: {score}"));
    draw_words(window, game_state);
    window.mvprintw(30, 0, &game_state.type_string);

    window.refresh();
}

pub fn game_over_screen(window: &Window, game_state: &GameState) {
    window.clear();
    
    window.mvprintw(21, 20, format!("final score: {}", game_state.player_score));
    window.mvprintw(20, 20, "GAME OVER");
    window.mvprintw(23, 20, "enter - restart. esc - quit.");
    
    window.refresh();
}
