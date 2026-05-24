use std::collections::HashMap;
use rand::Rng;

use crate::types::FallingWord;
use pancurses::Window;

pub fn spawn_word(given_fw: &mut Vec<FallingWord>, words_map: &HashMap<usize, Vec<&str>>) {
    let nxpos = rand::thread_rng().gen_range(0..50);
    let wrd_ind = rand::thread_rng().gen_range(0..4);

    let new_word = match wrd_ind {
        0 => String::from(words_map.get(&3).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&3).unwrap().len())]),
        1 => String::from(words_map.get(&4).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&4).unwrap().len())]),
        2 => String::from(words_map.get(&5).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&5).unwrap().len())]),
        3 => String::from(words_map.get(&3).unwrap()[rand::thread_rng().gen_range(0..words_map.get(&3).unwrap().len())]),
        _ => String::from("error?")
    };

    given_fw.push(FallingWord::new(0, nxpos, new_word));
}

pub fn draw_words(window: &Window, given_fw: &Vec<FallingWord>, type_string: &String) {
    for fw in given_fw {
        let mut matching: String = String::from("");

        //check how much match we have with type_string
        for chr_ind in 0..fw.word.len() {
            if chr_ind >= type_string.len() {
                break;
            }
            
            if fw.word.chars().nth(chr_ind) == type_string.chars().nth(chr_ind) {
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

pub fn move_words(given_fw: &mut Vec<FallingWord>) {
    for fw in given_fw {
        fw.ypos += 1;
    }
}
