use crate::ship;
use crate::utils;
use std::io;

#[derive(FromPrimitive)]
enum Action {
    Scavenge = 1,
    Refuel = 2,
    Jump = 3,
    Scores = 4,
}

pub struct Game {
    game_state: GameState,
    max_turns: u32,
}

struct GameState {
    current_turn: u32,
    distance: i32,
    ship: ship::Ship,
    scores: Vec<u32>,
}

impl Game {
    pub fn new(max_turns: u32, distance: i32) -> Game {
        Game {
            game_state: GameState::new(max_turns, distance),
            max_turns,
        }
    }

    pub fn game_loop(&mut self) {
        let mut done: bool = false;
        while !done {
            self.game_state.advance_turn(self.max_turns, &mut done);
        }
        if done {
            match utils::write_scores(self.game_state.current_turn, "scores.txt") {
                Ok(_) => return,
                Err(_) => return,
            }
        }
    }
}

impl GameState {
    fn new(max_turns: u32, distance: i32) -> GameState {
        GameState {
            current_turn: 0,
            distance,
            ship: ship::Ship::new(max_turns),
            scores: utils::load_scores("scores.txt"),
        }
    }

    fn advance_turn(&mut self, max_turns: u32, done: &mut bool) {
        println!();
        self.ship.display();
        println!("Remaining distance: {} ly", self.distance);
        println!("Turns remaining: {}", max_turns - self.current_turn);
        let action: Action = user_choice();
        match action {
            Action::Scores => {
                // TODO move to separate func
                println!();
                println!("High scores:\n-------");
                for score in &self.scores {
                    println!("{}", score);
                }
                println!("-------");
            }
            Action::Scavenge => self.ship.scavenge(),
            Action::Refuel => self.ship.refuel(),
            Action::Jump => {
                let jmp: bool = self.ship.jump();
                if jmp {
                    self.distance -= 10;
                    println!("You jump through hyperspace.");
                } else {
                    println!("Not enough fuel.");
                    return;
                }
            }
        }
        self.current_turn += 1;
        if self.distance <= 0 {
            println!("Congratulations, you won on turn {}", self.current_turn);
            *done = true;
            return;
        }
        if self.current_turn == max_turns {
            println!("Unfortunately, you didn't make it.");
        }
    }
}

fn user_choice() -> Action {
    println!("Choose what to do: ");
    println!("1 - Scavenge");
    println!("2 - Refuel");
    println!("3 - Hyperspace jump");
    println!("4 - Scores");
    loop {
        let mut input_string: String = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read");
        let action = input_string.trim().parse::<i32>();
        match action {
            Ok(num) => {
                let number = num::FromPrimitive::from_i32(num);
                match number {
                    Some(a) => return a,
                    None => continue,
                }
            }
            Err(_) => continue,
        }
    }
}
