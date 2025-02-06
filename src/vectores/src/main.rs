fn main() {
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);
    // 💡 Rust no sabe cómo mostrar un vector de enteros. Si intentamos mostrar la información del vector sin usar un formato especial, el compilador emite un error. 



    //Los vectores también se pueden crear mediante el método Vec::new(). Este método de creación de vectores nos permite agregar y quitar valores al final del vector. Para admitir este comportamiento, declaramos la variable de vector como mutable con la palabra clave mut.

    let mut fruits = Vec::new();
    // Poblar vectores con .push()
    fruits.push("Apple");
    fruits.push("Banana");
    fruits.push("Cherry");
    fruits.push("Strawberry");

    // fruits.push(31); //❌ Error, no podemos agregar un entero a un vector de cadenas -> expected `&str`, found integer

    //Mostrar el vector
    println!("Fruits: {:?}", fruits);

    // Quitar el ultimo valor, con .pop()
    println!("Popped: {:?}", fruits.pop());
    println!("Fruits: {:?}", fruits);


    // Indexación
    let mut numbers = vec![4,52,3];
    let first = numbers[0];
    println!("Vector: {:?} First: {}", numbers, first);

    /* Busqueda de valores de indice fuera de los limites del vector

    Al igual que con las matrices, no se puede acceder a un elemento de un vector con un índice que no esté en el intervalo permitido. Este tipo de expresión para una matriz hace que el compilador devuelva un error. En el caso de los vectores, la compilación pasa, pero el programa entra en un estado de pánico irrecuperable en la expresión y detiene la ejecución del programa.
    
     */

     let beyond = numbers[10];
     println!("{}", beyond);
}
