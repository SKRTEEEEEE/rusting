fn main() {
    //use trae la definición HashMap de la parte collections de la biblioteca estándar de Rust en el ámbito de nuestro programa. Esta sintaxis es parecida a lo que otros lenguajes de programación llaman una importación.
    use std::collections::HashMap;
    //Creamos un mapa hash vacío con el método HashMap::new. Declaramos la variable reviews como mutable para que podamos agregar, o quitar, claves y valores, según sea necesario. En nuestro ejemplo, tanto las claves de mapa hash como los valores usan el tipo String.
    let mut reviews: HashMap<String, String> = HashMap::new();
    //Agregamos elementos al mapa hash mediante el método insert(<key>, <value>). En el código, la sintaxis es <hash_map_name>.insert():
    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );
    //podemos obtener un valor específico para una clave con el método get(<key>).
    let book: &str = "Programming in Rust";
    println!("\nReview for\'{}\': {:?}", book, reviews.get(book));
    //Se pueden quitar entradas de un mapa hash mediante el método .remove(). Si usamos el método get para una clave de mapa hash no válida, el método get devuelve "None".

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
