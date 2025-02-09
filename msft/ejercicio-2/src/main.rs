#[derive(PartialEq, Debug, Clone, Copy)] // ← Agregamos Clone y Copy
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    mileage: (Age, u32),
}

fn car_quality(km: u32) -> (Age, u32) {
    let quality = (
        if km == 0 { Age::New } else { Age::Used },
        km
    );

    quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color,
        motor,
        roof,
        mileage: car_quality(miles),
    }
}

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut engine: Transmission = Transmission::Manual;

    // Se crea un auto inicial, aunque nunca se usa.
    let mut car: Car;

    // Car order #1: Nuevo, Manual, Hard top
    car = car_factory(colors[0].to_string(), engine, true, 0);
    println!(
        "Car order 1: {:?}, {}, {:?}, Hard top = {}, {} miles",
        car.mileage.0, car.color, car.motor, car.roof, car.mileage.1
    );

    // Car order #2: Usado, Semi-automático, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(colors[2].to_string(), engine, false, 100);
    println!(
        "Car order 2: {:?}, {}, {:?}, Hard top = {}, {} miles",
        car.mileage.0, car.color, car.motor, car.roof, car.mileage.1
    );

    // Car order #3: Usado, Automático, Hard top
    engine = Transmission::Automatic;
    car = car_factory(colors[3].to_string(), engine, true, 200);
    println!(
        "Car order 3: {:?}, {}, {:?}, Hard top = {}, {} miles",
        car.mileage.0, car.color, car.motor, car.roof, car.mileage.1
    );
}
