pub mod models;

fn main() {
    let jfk = models::Airport::new("JFK", "New York");
    let ams = models::Airport::new("AMS", "Amsterdam");

    let daniele = models::Person::new("Daniele", "Trapani");
    let daniella = models::Person::new("Daniella", "Liendo");

    let mut our_flight = models::Flight::new(ams, jfk, 300);
    our_flight.add_passenger(daniella);
    our_flight.add_passenger(daniele);

    our_flight.print_passengers();
    println!("{}", our_flight.departure);
    println!("{}", our_flight.arrival);
    println!("{}", our_flight.duration);
}
