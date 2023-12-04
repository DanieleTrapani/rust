pub struct Airport {
    code: String,
    city: String,
}

impl Airport {
    pub fn new(code: &str, city: &str) -> Airport {
        Airport {
            code: code.to_string(),
            city: city.to_string(),
        }
    }
}

impl std::fmt::Display for Airport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.code, self.city)
    }
}

pub struct Flight {
    pub departure: Airport,
    pub arrival: Airport,
    pub duration: u16,
    passengers: Vec<Person>,
}

impl Flight {
    pub fn new(departure: Airport, arrival: Airport, duration: u16) -> Flight {
        Flight {
            departure,
            arrival,
            duration,
            passengers: Vec::new(),
        }
    }

    pub fn add_passenger(&mut self, person: Person) {
        self.passengers.push(person)
    }

    pub fn print_passengers(&self) {
        for passenger in &self.passengers {
            println!("{}", passenger)
        }
    }
}

pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
