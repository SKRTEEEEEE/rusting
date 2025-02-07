fn main() {
    iflet();

}
pub fn get_option(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    //.get() devuelve un Option
    let first = fruits.get(0);
    println!("{:?}", first);
    
    let third = fruits.get(2);
    println!("{:?}", third);
    
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
}
pub fn detect_patterns(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() //Se recorre la iteración del array
    
    {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}
pub fn detect_oth_patterns(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}
pub fn comodin(){
    //nos gustaría ignorar la variante None y todos los valores de Some<u8> que no coincidan con Some(7). Los patrones comodín son útiles para este tipo de situación. Puede agregar el patrón de carácter comodín _ (subrayado) después de que todos los demás patrones coincidan con cualquier otra cosa; este patrón se usa para satisfacer las demandas del compilador de agotar las secciones de coincidencia.
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }
}
pub fn iflet() {
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }
}