struct Aircraft {
    model: String,
    engine: String,
    class: String,
    bombs: bool,
}

impl Aircraft {
    fn new(model: String, engine: String, class: String) -> Self {
        Self {
            model: model,
            engine: engine,
            class: class,
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
    println!("Aircraft name: {}", new_aircraft.model);
    println!("{} armed status is: {}", new_aircraft.model, new_aircraft.bombs);
    new_aircraft.deactivate();
    println!("{} armed status is: {}", new_aircraft.model, new_aircraft.bombs);
}
