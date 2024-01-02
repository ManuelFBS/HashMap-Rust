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
    while color > 4 {
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

    // Inicializando con 0 millas...
    let mut miles = 0;

    // Se declara un car como mutable "Car" struct...
    let mut car: Car;

    for order in 1..12 {
        // Se llama a car_factory para realizar
        // los pedidos...
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order: {}: {:?}", order, orders.get(&order));

        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
