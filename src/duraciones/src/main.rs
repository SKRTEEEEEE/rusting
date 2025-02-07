fn main() {
    longest_duration();
    }

// pub fn puntero_no_colgante(){
//     let x;
//     {
//         let y = 42;
//         x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
//     }
//     println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
// }

// pub fn misma_duracion(){
    
//     let magic1 = String::from("abracadabra!");
//     let magic2 = String::from("shazam!");

//     let result = longest_word(&magic1, &magic2);
//     println!("The longest magic word is {}", result);
    
    
//     // fn longest_word(x: &String, y: &String) -> &String {
//     //     if x.len() > y.len() {
//     //         x
//     //     } else {
//     //         y
//     //     }
//     // }
//     fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     }
//     // ⬆️ no hay nada especial acerca del nombre 'a. Sería igual de correcto usar cualquier otra palabra, como 'response o 'program. Lo importante que debe tenerse en cuenta es que todos los parámetros y el valor devuelto estarán activos al menos mientras esté vigente la duración asociada a cada uno de ellos.
// }
pub fn longest_duration(){
    
        let magic1 = String::from("abracadabra!");
        let result;
        {
            let magic2 = String::from("shazam!");
            result = longest_word(&magic1, &magic2);
        }
        println!("The longest magic word is {}", result);
    
    
    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    //⬆️El compilador esperaba que la duración de magic2 fuera la misma que la duración del valor devuelto y del argumento de entrada x. Rust esperaba este comportamiento porque se han anotado las duraciones de los parámetros de función y el valor devuelto con el mismo nombre de duración: 'a. Si inspeccionáramos el código, como humanos, veríamos que magic1 es más largo que magic2.
}