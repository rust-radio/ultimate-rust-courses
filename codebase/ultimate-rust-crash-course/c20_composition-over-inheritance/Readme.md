# Rust takes the composition over inheritance approach

**Rust takes the composition over inheritance approach**

**ChatGPT**: Yes, Rust takes a composition over inheritance approach to object-oriented programming. Rust's focus on composition is reflected in its support for structs and enums, which can be used to define new types and implement behavior for those types through methods and associated functions.

In Rust, inheritance is not directly supported, but it can be emulated to some extent through trait objects and other language features. Trait objects allow you to define a set of methods that can be implemented by multiple types, and then group those types together under a single interface. This allows for a form of polymorphism similar to inheritance, where objects of different types can be treated as instances of the same interface.

However, Rust encourages a more flexible and modular approach to building complex systems, where functionality is composed of smaller, independent parts rather than inherited from a base class. This approach is known as composition over inheritance, and it allows for greater code reuse and flexibility in designing complex systems.

In summary, Rust's focus on composition over inheritance reflects its overall design philosophy of providing powerful and flexible language features that allow developers to build efficient and modular systems.

**Give me a sample code to clarify it?**

**ChatGPT:** Sure! Here's an example that demonstrates Rust's composition over inheritance approach using structs and traits:

```rust
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
```

In this example, we define a Vehicle trait with a single method speed(). We then define two structs, Car and Bicycle, both of which implement the Vehicle trait by providing their own implementation of the speed() method.

Next, we define a Driver struct that contains a reference to a Vehicle object. The Driver struct also has a drive() method that calls the speed() method on the Vehicle object to determine the current speed and prints it out.

In the main() function, we create two instances of Car and Bicycle, respectively. We then create two instances of Driver, each with a reference to one of the Vehicle objects. Finally, we call the drive() method on each Driver object, which calls the appropriate speed() method on the underlying Vehicle object to print out the current speed.

This example demonstrates Rust's composition over inheritance approach by using traits to define a common interface (Vehicle) that can be implemented by multiple types (Car and Bicycle). This allows the Driver struct to accept any object that implements the Vehicle trait, rather than being limited to a single inheritance hierarchy. This approach provides greater flexibility and modularity in designing complex systems, while still allowing for polymorphism through trait objects.

