use pancurses::Window;
use crate::types::{FallingWord, GameState};
use crate::draw_words;

pub fn title_screen(window: &Window) {
    window.clear();
    
    window.mvprintw(10, 10, "Welcome to my typing game!");
    window.mvprintw(20, 10, "press any key to begin.");
    
    window.refresh();
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
