use pancurses::Window;
use crate::types::FallingWord;
use crate::draw_words;

pub fn begin_game_screen(window: &Window) {
    window.clear();
    
    window.mvprintw(10, 10, "Welcome to my typing game!");
    window.mvprintw(20, 10, "press any key to begin.");
    
    window.refresh();
}

pub fn game_screen(window: &Window, falling_words: &Vec<FallingWord>,
                   type_string: &String, player_health: &i32, player_score: &i32) {

    window.clear();

    window.printw("esc - quit game. enter - submit / refresh ur typing\n");
    window.printw("Type the falling words before they reach the bottom.");
    window.mvprintw(25, 0, "-".repeat(25));
    window.mvprintw(28, 0, format!("Health: {player_health}"));
    window.mvprintw(28, 20, format!("Score: {player_score}"));
    draw_words(&window, &falling_words, &type_string);
    window.mvprintw(30, 0, &type_string);

    window.refresh();
}

pub fn game_over_screen(window: &Window, score: &i32) {
    window.clear();
    
    window.mvprintw(21, 20, format!("final score: {}", score));
    window.mvprintw(20, 20, "GAME OVER");
    window.mvprintw(23, 20, "enter - restart. esc - quit.");
    
    window.refresh();
}
