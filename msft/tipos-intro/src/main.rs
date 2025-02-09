fn main() {
    // let number: u32 = 42;
    // println!("number u32: {}", number);
    //⬆️Esto compila por 14 es un número entero positivo menor a 32 bits
    // let number: u32 = -42;
    // let number: u32 = "42";
    // println!("number u32: {}", number);
    //⬆️Esto no compila porque lo que se le asigna no es un número entero positivo menor a 32 bits

    // Operaciones con números

    println!("1 + 2 = {} and -8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, -8i32 - 5, 15 * 3);
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
    
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);// Integer and Floating point division


    // Booleanos

    let is_bigger = 1 > 4; // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    println!("Is 1 > 4? {}", is_bigger);  

    // Caracteres

    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';

    println!("uppercase_s: {}", uppercase_s);
    println!("lowercase_f: {}", lowercase_f);
    println!("smiley_face: {}", smiley_face);

    // String

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";//no es necesario declarar el tipo de dato ya que el compilador lo infiere

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}
