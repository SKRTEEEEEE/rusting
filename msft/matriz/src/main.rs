fn main() {

// Declaración

// Declare array, initialize all values, compiler infers length = 7
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
// Declare array, initialize all values to 0, length = 5
let bytes = [0; 5];

// Declare array, initialize first value, compiler infers length = 5
// let clear = [u32;5]; //❌ No funciona asi
let clean: [u32; 5];

// Indexación

let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

let first_day = days[0]; // "Sunday"
let third_day = days[2]; // "Tuesday"
let eighth_day = days[7]; // panic! out of bounds -> ❌ Error, no podemos acceder a un indice que no existe

}
