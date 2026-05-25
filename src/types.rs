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

pub struct GameState {
}
