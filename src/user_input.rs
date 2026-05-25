use pancurses::{Window, Input};
use crate::types::FallingWord;

pub fn game_input(window: &Window, type_string: &mut String,
                  falling_words: &mut Vec<FallingWord>, player_score: &mut i32) -> i32 {
    match window.getch() {
        Some(Input::Character(c)) if c.is_alphabetic() => {
            type_string.push(c);
        },
        Some(Input::KeyBackspace) | Some(Input::Character('\x7f')) => {
            type_string.pop();
        },
        Some(Input::Character('\n')) => {
            for fw_ind in (0 .. falling_words.len()).rev() {
                if falling_words[fw_ind].word == *type_string {
                    falling_words.remove(fw_ind);
                    *player_score += 100;
                }
            }

            *type_string = String::from("");
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
