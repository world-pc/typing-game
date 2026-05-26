use pancurses::{initscr, noecho, endwin, Input, start_color, init_pair};
use std::collections::HashMap;
use std::thread::{sleep};
use std::time::Duration;

mod window_stuff;
use window_stuff::game_screen;

mod word_stuff;
use word_stuff::{spawn_word, draw_words, move_words};

mod types;
use types::{FallingWord, GameState};

mod user_input;
use user_input::{game_input, game_over_input};

fn main() {
    //read in the english words
    let words: Vec<&str> = include_str!("../assets/words.txt")
        .lines().collect();

    //create associative array linking word length to vector of words with that length
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

    //stuff to keep track of for the game (score, health, etc.)
    let mut game_state = GameState::new();

    window_stuff::title_screen(&window);
    window.getch();

    window_stuff::transition(&window);

    window.nodelay(true);

    loop {
        window.clear();

        //check for game over
        if game_state.player_health <= 0 {
            window_stuff::game_over_screen(&window, &game_state);

            match window.getch() {
                Some(Input::Character('\x1b')) => break,
                Some(Input::Character('\n')) => {
                    game_state = GameState::new();
                    window_stuff::transition(&window);
                },
                _ => {}
            }
        }

        else {

            game_screen(&window, &mut game_state);

            window.refresh();

            //check for damage
            for fw in &game_state.falling_words {
                if fw.ypos >= 25 {
                    game_state.player_health -= 10;
                }
            }

            //filter out the words on the line
            game_state.falling_words.retain(|fw| fw.ypos < 25);

            if game_state.frame_count % 60 == 0 {
                spawn_word(&mut game_state, &words_map);
                game_state.frame_count = 0;
            }

            if game_state.frame_count % 20 == 0 {
                move_words(&mut game_state);
            }

            //handle user input
            let usrin: i32 = game_input(&window, &mut game_state);
            if usrin == 1 {break;}

            game_state.frame_count += 1;
        }

        sleep(Duration::from_millis(16)); //~60fps
    }

    endwin();
}
