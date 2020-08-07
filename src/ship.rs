pub struct Ship {
    fuel: f32,
    resources: i32
}

impl Ship {
    pub fn new(rounds: u32) -> Ship {
        Ship {
            fuel: (rounds * 2) as f32,
            resources: 0
        }
    }

    pub fn refuel(&mut self){
        if self.resources >= 10 {
            self.fuel += 10.0;
            self.resources -= 10;
            println!("Successfuly refueled. Fuel: {}, Resources: {}.", self.fuel, self.resources);
        }
        println!("Not enough resources.")
    }

    pub fn scavenge(&mut self) {
        self.resources += 10;
        println!("You found 10 resources.");
    }

    pub fn jump(&mut self) -> bool {
        if self.fuel >= 10.0 {
            self.fuel -= 10.0;
            return true;
        }
        false
    }

    pub fn display(&self) {
        println!("Current fuel: {}", self.fuel);
        println!("Current resources: {}", self.resources);
    }

}