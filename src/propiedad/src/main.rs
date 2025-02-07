fn process(_s: String) {}

fn main() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.
}



// pub fn copy_implicito(){
//     fn process(input: u32) {}

//     fn caller() {
//         let n = 1u32;
//         process(n); // Ownership of the number in `n` copied into `process`
//         process(n); // `n` can be used again because it wasn't moved, it was copied.

//         //Aunque devuelve un warning, ya que no sera accesible en el segundo process(), 
//     }
// }

// pub fn movida_funcion(){
//     //TRANSFERENCIA PROPIEDAD
//     fn process(input: String) {}

//     fn caller() {
//         let s = String::from("Hello, world!");
//         process(s); // Ownership of the string in `s` moved into `process`
//         process(s); // Error! ownership already moved.
//     }
// }

// pub fn variable_movida() {
//     {
//         let mascot = String::from("ferris");
//         let ferris = mascot;
//         println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
//     }
// }