use std::collections::HashMap;
use rand::Rng;

use crate::types::{FallingWord, GameState};
use pancurses::Window;

pub fn spawn_word(game_state: &mut GameState, words_map: &HashMap<usize, Vec<&str>>) {
    let nxpos = rand::thread_rng().gen_range(0..50);
    let wrd_ind = rand::thread_rng().gen_range(0..4);

    let new_word = match wrd_ind {
        0 => String::from(words_map.get(&3).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&3).unwrap().len())]),
        1 => String::from(words_map.get(&4).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&4).unwrap().len())]),
        2 => String::from(words_map.get(&5).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&5).unwrap().len())]),
        3 => String::from(words_map.get(&3).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&3).unwrap().len())]),
        _ => String::from("error?")
    };

    game_state.falling_words.push(FallingWord::new(0, nxpos, new_word));
}

pub fn draw_words(window: &Window, game_state: &GameState) {
    for fw in &game_state.falling_words {
        let mut matching: String = String::from("");

        //check how much match we have with type_string
        for chr_ind in 0..fw.word.len() {
            if chr_ind >= game_state.type_string.len() {
                break;
            }
            
            if fw.word.chars().nth(chr_ind) == game_state.type_string.chars().nth(chr_ind) {
                matching.push(fw.word.chars().nth(chr_ind).unwrap_or(' '));
            }
            else {
                break;
            }
        }

        window.mv(fw.ypos as i32, fw.xpos as i32);
        
        window.attron(pancurses::ColorPair(1));
        let mut m_index = 0;
        while m_index < matching.len() {
            window.printw(matching.chars().nth(m_index).unwrap_or(' ').to_string());
            m_index += 1;
        }
        window.attroff(pancurses::ColorPair(1));

        while m_index < fw.word.len() {
            window.printw(fw.word.chars().nth(m_index).unwrap_or(' ').to_string());
            m_index += 1;
        }
    }
}

pub fn draw_explosions(window: &Window, game_state: &mut GameState) {

    for expl_ind in 0 .. game_state.explosions.len() {
        if expl_ind >= game_state.explosions.len() {break;} //make this index situation neater
                                                            //later.
        if game_state.explosions[expl_ind].age > 20 {
            game_state.explosions.remove(expl_ind);
        }
        else {
            //explosion is drawn in the same row left & right of word
            if expl_ind > 0 {
                let mut ex: i32 = (game_state.explosions[expl_ind].xpos as i32) - 
                              (game_state.explosions[expl_ind].age as i32);
            
                if ex > 0 {
                    window.mvprintw(game_state.explosions[expl_ind].ypos as i32, ex as i32, "-");
                }

                ex = (game_state.explosions[expl_ind].xpos as i32) +
                     (game_state.explosions[expl_ind].age as i32);
                window.mvprintw(game_state.explosions[expl_ind].ypos as i32, ex as i32, "-");

                game_state.explosions[expl_ind].age += 1;
            }
        }
    }
}

pub fn move_words(game_state: &mut GameState) {
    for fw in &mut game_state.falling_words {
        fw.ypos += 1;
    }
}
