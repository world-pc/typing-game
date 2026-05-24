use pancurses::{initscr, noecho, endwin, Input, Window, start_color, init_pair};
use std::collections::HashMap;
use std::thread::{sleep};
use std::time::Duration;
use rand::Rng;

mod window_stuff;

struct FallingWord {
    ypos: u8,
    xpos: u8,
    word: String
}

impl FallingWord {
    fn new(given_ypos: u8, given_xpos: u8, given_word: String) -> FallingWord {
       FallingWord {ypos: given_ypos,
                    xpos: given_xpos,
                    word: given_word}
    }
}

fn spawn_word(given_fw: &mut Vec<FallingWord>, words_map: &HashMap<usize, Vec<&str>>) {
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

fn draw_words(window: &Window, given_fw: &Vec<FallingWord>, type_string: &String) {
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

fn move_words(given_fw: &mut Vec<FallingWord>) {
    for fw in given_fw {
        fw.ypos += 1;
    }
}

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
            //draw some stuffs
            window.printw("esc - quit game. enter - submit / refresh ur typing\n");
            window.printw("Type the falling words before they reach the bottom.");
            window.mvprintw(25, 0, "-".repeat(25));
            window.mvprintw(28, 0, format!("Health: {player_health}"));
            window.mvprintw(28, 20, format!("Score: {player_score}"));
            draw_words(&window, &falling_words, &type_string);
            window.mvprintw(30, 0, &type_string);

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
