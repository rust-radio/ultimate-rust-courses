trait Vehicle {
    fn speed(&self) -> f64;
}

struct Car {
    max_speed: f64,
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.max_speed
    }
}

struct Bicycle {
    max_speed: f64,
}

impl Vehicle for Bicycle {
    fn speed(&self) -> f64 {
        self.max_speed
    }
}

struct Driver<'a> {
    vehicle: &'a dyn Vehicle,
}

impl<'a> Driver<'a> {
    fn new(vehicle: &'a dyn Vehicle) -> Driver<'a> {
        Driver { vehicle }
    }

    fn drive(&self) {
        println!("Driving at {} mph", self.vehicle.speed());
    }
}

fn main() {
    let car = Car { max_speed: 120.0 };
    let bicycle = Bicycle { max_speed: 20.0 };

    let driver1 = Driver::new(&car);
    let driver2 = Driver::new(&bicycle);

    driver1.drive(); // This will print "Driving at 120 mph".
    driver2.drive(); // This will print "Driving at 20 mph".
}