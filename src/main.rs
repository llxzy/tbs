struct Game {
    game_state: GameState
}

struct GameState {
    current_turn: u32,
    max_turns: u32
}

impl Game {
    fn new(max_turns: u32) -> Game {
        Game {
            game_state: GameState::new(max_turns)
        }        
    }

    fn game_loop(&mut self) {
        while self.game_state.current_turn < self.game_state.max_turns {
            println!("Turn {}", self.game_state.current_turn);
            self.game_state.advance_turn();            
        }
    }
}

impl GameState {
    fn new(max_turns: u32) -> GameState {
        GameState {
            current_turn: 0,
            max_turns
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn += 1;
    }
}


fn main() {
    let mut game = Game::new(10);
    game.game_loop();
}

