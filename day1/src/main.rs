mod input;
use input::MASSES;

fn fuel_req(mass: &i64) -> i64 {
    (mass / 3) - 2
}

fn re_fuel_req(mass: &i64) -> i64 {
    let base_fuel = fuel_req(mass);
    let mut accumulated_fuel = 0;
    let mut acc = fuel_req(&base_fuel);
    while acc >= 0 {
        accumulated_fuel += acc;
        acc = fuel_req(&acc);
    }
    base_fuel + accumulated_fuel
}

fn main() {
    let needed: i64 = MASSES.iter().map(re_fuel_req).sum();
    println!("I need {} units of fuel", needed);
}
