fn main() {

    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);



    // Estructura clasica

    //1.Declaramos la estructura
    struct Student { name: String,  remote: bool, level: u8}
    //2. Creamos dos instancias de la estructura
    let user_1 = Student { name: "Constance Sharma", remote: true, level: 2 };
    //⬆️ Esto nos lanzara un error porque no le podemos asignar un tipo &str a un tipo String
    // Para eso convertimos el tipo &str a String, tenemos dos maneras por ahora:
    let user_1 = Student { name: "Constance Sharma".to_string(), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
    //3. Mostramos los valores de las instancias
    println!(
        "User 1: Me llamo {} y {} trabajo remotamente",
        user_1.name, 
        if user_1.remote { "sí" } else { "no" }
    );
    println!(
        "User 2: Me llamo {}, {} trabajo remotamente y soy de nivel {}",
        user_2.name, 
        if user_2.remote { "sí" } else { "no" },
        user_2.level
    );
}
