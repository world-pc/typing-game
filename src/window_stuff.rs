use pancurses::Window;

pub fn begin_game_screen(window: &Window) {
    window.clear();
    window.mvprintw(10, 10, "Welcome to my typing game!");
    window.mvprintw(20, 10, "press any key to begin.");
    window.refresh();
}

pub fn game_over_screen(window: &Window, score: &i32) {
    window.clear();
    window.mvprintw(21, 20, format!("final score: {}", score));
    window.mvprintw(20, 20, "GAME OVER");
    window.mvprintw(23, 20, "enter - restart. esc - quit.");
    window.refresh();
}
