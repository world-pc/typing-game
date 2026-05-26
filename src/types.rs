pub struct FallingWord {
    pub ypos: u8,
    pub xpos: u8,
    pub word: String
}

impl FallingWord {
    pub fn new(given_ypos: u8, given_xpos: u8, given_word: String) -> FallingWord {
       FallingWord {ypos: given_ypos,
                    xpos: given_xpos,
                    word: given_word}
    }
}

pub struct Explosion {
    pub xpos: u8,
    pub ypos: u8,
    pub age: u8
}

impl Explosion {
    pub fn new(given_ypos: u8, given_xpos: u8) -> Explosion {
        Explosion {ypos: given_ypos, 
                   xpos: given_xpos,
                   age: 0}
    }
}

pub struct GameState {
    pub player_health: i32,
    pub player_score: i32,
    pub type_string: String,
    pub falling_words: Vec<FallingWord>,
    pub frame_count: u32,
    pub explosions: Vec<Explosion>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {player_health: 100,
                   player_score: 0,
                   type_string: String::from(""),
                   frame_count: 1,
                   falling_words: vec![FallingWord::new(2, 0, String::from("begin"))],
                   explosions: Vec::new()}
    }
}
