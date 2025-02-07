// 1. Limitaciones tipos genéricos

// struct Point {
//     x: i32,
//     y: i32,
// }

//2. Uso de `derive()`
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
//3. Implemtar display?
use std::fmt;


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

}

/*  ⬆️Este código no se compila porque el tipo Point no implementa los rasgos siguientes:

El rasgo Debug, que permite dar formato a un tipo mediante el especificador de formato {:?}, se usa en un contexto de depuración orientado al programador.
El rasgo Display, que permite dar formato a un tipo mediante el especificador de formato {}, es similar a Debug. Pero Display es más adecuado para la salida orientada al usuario.
El rasgo PartialEq, que permite comparar la igualdad de los implementadores.

*/

