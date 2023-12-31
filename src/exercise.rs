#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Obtener la calidad del automóvil probando
// el valor del argumento de entrada...
// - miles (u32)...
// Devolver una tupla con 'car age' ("New" o "Used")...
fn car_quality(miles: u32) -> (Age, u32) {
    // Verificar si el vehículo tiene millas acumuladas...
    // Devolver una tupla anticipadamente para un auto usado...
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Devolver una tupla para un auto nuevo. Nes necesario
    // el uso de la instrucción 'return' o del ";"...
    (Age::New, miles)
}

// Se crea un un "Car" con los argumentos de entrada...
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Verificar índices del array 'colors', se resetea si es necesario...
    // Colores válidos = 1, 2, 3 o 4...
    // if color > 4, reduce el color para un ídice válido...
    let mut color = order as usize;
    if color > 4 {
        // color = 5 --> index 1, color = 6 --> 2,
        // color = 7 --> 3, color = 8 --> 4...
        color = color - 4;
    }

    // Agregar variedad de órdenes para el tipo e motor
    // y tipo de techo...
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    } else {
        // 1, 5, 7, 11
        motor = motor;
        roof = true
    }

    // Devolver auto requerido...
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {
    // Se inicializa un 'hash map' para las órdenes
    // de los autos...
    // - Key: Car order number, i32...
    // - Value: Car order details, Car struct...
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Se inicializa variable contadora...
    let mut order = 1;
    // Se declara un car como mutable "Car" struct...
    let mut car: Car;

    // Se ordenan 6 autos, incementando "order"
    // para cada requisición...
    // Orden para el auto #1: Usado, Techo rígido...
    car = car_factory(order, 1000);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Orden para el auto #2: Usado, Convertible...
    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Orden para el auto #3: Nuevo, Techo rígido...
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Orden para el auto #4: Nuevo, Convertible...
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Orden para el auto #5: Usado, Techo rígido...
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Orden para el auto #6: Usado, Techo rígido
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    // println!(
    //     "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     order, car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
    println!("Car order {}: {:?}", order, orders.get(&order));
}
