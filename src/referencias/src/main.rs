fn main() {
    //1.
    // prestamo_basico(); 
    //2. 
    // let greeting = String::from("Hello");
    // print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    // print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
    //3. Inmutable
    // change(&greeting);
    //4. Mutable
    let mut greeting = String::from("hello");
    change(&mut greeting);

}

pub fn prestamo_basico(){
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`
}
fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}
//3.
// fn change(message: &String) {
//     message.push_str("!"); // We try to add a "!" to the end of our message
//   }
  //4.
  fn change(text: &mut String) {
    text.push_str(", world");
}