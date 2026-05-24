use pancurses::{initscr, noecho, endwin, Input, start_color, init_pair};
use std::collections::HashMap;
use std::thread::{sleep};
use std::time::Duration;

mod window_stuff;
use window_stuff::game_screen;
mod word_stuff;
mod types;
use types::FallingWord;
use word_stuff::{spawn_word, draw_words, move_words};

fn main() {
    //read in the english words
    let words: Vec<&str> = include_str!("../assets/words.txt")
        .lines().collect();

    let mut words_map: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in &words {
        words_map.entry(word.len()).or_insert_with(Vec::new).push(word);
    }

    //store the words we have falling
    let mut falling_words: Vec<FallingWord> = vec![FallingWord::new(2, 0, String::from("begin"))];

    let window = initscr();
    window.nodelay(false);
    noecho();

    start_color();
    init_pair(1, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);

    let mut frame_count: u32 = 1;

    let mut player_health = 100;
    
    let mut player_score = 0;

    let mut type_string = String::from("");

    window_stuff::begin_game_screen(&window);
    window.getch();

    window.nodelay(true);

    loop {
        window.clear();

        //check for game over
        if player_health <= 0 {
            window_stuff::game_over_screen(&window, &player_score);

            match window.getch() {
                Some(Input::Character('\x1b')) => break,
                Some(Input::Character('\n')) => {
                    player_health = 100;
                    player_score = 0;
                    type_string = String::from("");
                    frame_count = 1;
                    falling_words.clear();
                },
                _ => {}
            }
        }

        else {

            game_screen(&window, &falling_words, &type_string, &player_health, &player_score);

            window.refresh();

            //check for damage
            for fw in &falling_words {
                if fw.ypos >= 25 {
                    player_health -= 10;
                }
            }

            //filter out the words on the line
            falling_words.retain(|fw| fw.ypos < 25);

            if frame_count % 60 == 0 {
                spawn_word(&mut falling_words, &words_map);
                frame_count = 0;
            }

            if frame_count % 20 == 0 {
                move_words(&mut falling_words);
            }

            //handle user input
            match window.getch() {
                Some(Input::Character(c)) if c.is_alphabetic() => {
                    type_string.push(c);
                },
                Some(Input::KeyBackspace) | Some(Input::Character('\x7f')) => {
                    type_string.pop();
                },
                Some(Input::Character('\n')) => {
                    for fw_ind in (0 .. falling_words.len()).rev() {
                        if falling_words[fw_ind].word == type_string {
                            falling_words.remove(fw_ind);
                            player_score += 100;
                        }
                    }

                    type_string = String::from("");
                },
                Some(Input::Character('\x1b')) => break,
                _ => {}
            }

            frame_count += 1;
        }

        sleep(Duration::from_millis(16)); //~60fps
    }

    endwin();
}
