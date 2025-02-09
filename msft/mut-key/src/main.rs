fn main() {
    // let a_number = 10;
    // a_number = 20;
    // println!("a_number: {}", a_number);
    //⬆️ -> esto nos dara un error en la compilación, ya que estamos reasignando una variable que es inmutable.
    // Para solucionar esto, debemos de hacer que la variable sea mutable.
    let mut a_number = 10;
    a_number = 20;
    println!("a_number: {}", a_number); //esto nos dara un warning, ya que la variable 'a_number original(10)' no esta siendo usada jamas, pero si se usara no nos lo dara.
}
