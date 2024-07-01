struct Aircraft {
    model: String,
    engine: String,
    class: String,
    bombs: bool,
}

impl Aircraft {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            model: username,
            engine: email,
            class: uri,
            bombs: true,
        }
    }
    fn deactivate(&mut self) {
        self.bombs = false;
    }
}

fn main() {
    let mut new_aircraft = Aircraft::new(
        String::from("Satan II"),
        String::from("Merlin"),
        String::from("Racer"),
    );
    println!("Hello, {}!", new_aircraft.model);
    println!("Account {} status is: {}", new_aircraft.model, new_aircraft.bombs);
    new_aircraft.deactivate();
    println!("Account {} status is: {}", new_aircraft.model, new_aircraft.bombs);
}
