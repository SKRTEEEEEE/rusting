fn main() {
// Display the message "Hello, world!"
// todo!("Display the message by using the println!() macro"); // -> Macro usada para que nos haga un warning al compilar de que algo aun esta por terminar, esto, ejecuta la macro 'panic!()' lo cual detiene la ejecución del código.
println!("Hello, world!"); // -> Macro usada para imprimir en consola.
println!("Me gusta la {}!, el {} y el {}", "pizza", "futbol", "cine"); // -> Macro usada para imprimir en consola pasando-le argumentos.

/*
- Variables
Palabra reservada 'let' para declarar una variable.

*/
let a_number; // -> Declaración de una variable sin asignarle un valor.

let a_word = "Hello, world!"; // -> Declaración de una variable asignandole un valor.

a_number = 10; // -> Asignación de un valor a una variable.

println!("a_number: {}, a_word: {}", a_word, a_number);


}
