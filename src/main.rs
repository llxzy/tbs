extern crate num;
#[macro_use]
extern crate num_derive;

use std::io;

mod ship;

#[derive(FromPrimitive)]
enum Action {
    Scavenge = 1,
    Refuel   = 2,
    Jump     = 3
}

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
        let mut ship = ship::Ship::new(self.game_state.max_turns);
        let mut distance = 100;
        while self.game_state.current_turn < self.game_state.max_turns {
            println!("");
            ship.display();
            println!("Turns remaining: {}", self.game_state.max_turns - self.game_state.current_turn);
            let action: Action = user_choice();
            match action {
                Action::Scavenge => ship.scavenge(),
                Action::Refuel => ship.refuel(),
                Action::Jump => {
                    let jmp = ship.jump();
                    if jmp {
                        distance -= 10;
                        println!("You jump through hyperspace.");
                        println!("Remaining distance: {} ly", distance);
                    } else {
                        println!("Not enough fuel.");
                        continue;
                    }
                    
                }
            }
            if distance == 0 {
                println!("Congratulations, you win on turn {}", self.game_state.current_turn);
                return;
            }
            self.game_state.advance_turn();            
        }
        println!("Unfortunately, you didn't make it.")
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

fn user_choice() -> Action {
    println!("Choose what to do: ");
    println!("1 - Scavenge");
    println!("2 - Refuel");
    println!("3 - Hyperspace jump");
    loop {
        let mut input_string: String = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read");
        let action = input_string.trim().parse::<i32>();
        match action {
            Ok(num) => {
                let number = num::FromPrimitive::from_i32(num);
                match number {
                    Some(a) => return a,
                    None => continue
                }
            },
            Err(_) => continue
        }
    }
}

fn main() {
    let base_rounds = 20;
    let mut game = Game::new(base_rounds);
    game.game_loop();
}

