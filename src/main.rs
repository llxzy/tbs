use std::io;

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
    println!("Enter max rounds");
    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read.");
    let rounds = match input_string.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("Error parsing")
    };
    
    let mut game = Game::new(rounds);
    game.game_loop();
}

