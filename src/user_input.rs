use pancurses::{Window, Input};
use crate::types::{FallingWord, GameState};

pub fn game_input(window: &Window, game_state: &mut GameState) -> i32 {

    match window.getch() {
        Some(Input::Character(c)) if c.is_alphabetic() => {
            game_state.type_string.push(c);
        },
        Some(Input::KeyBackspace) | Some(Input::Character('\x7f')) => {
            game_state.type_string.pop();
        },
        Some(Input::Character('\n')) => {
            for fw_ind in (0 .. game_state.falling_words.len()).rev() {
                if game_state.falling_words[fw_ind].word == game_state.type_string {
                    game_state.falling_words.remove(fw_ind);
                    game_state.player_score += 100;
                }
            }

            game_state.type_string = String::from("");
        },
        Some(Input::Character('\x1b')) => return 1,
        _ => {}
    }

    return 0;
}

pub fn game_over_input(window: &Window) -> i32 {
    match window.getch() {
        Some(Input::Character('\x1b')) => return 1,
        _ => {}
    }

    return 0;
}
