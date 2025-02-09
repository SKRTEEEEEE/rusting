//1. Básica yPasar argumentos

// fn goodbye(message: &str){
//     println!("{}", message);
// }
// fn main() {
//     let normal = "Goodbye, world!";
//     let casual = "Fuck you!";
//     println!("Hello, world!");
//     goodbye(normal);
//     goodbye(casual);
// }
// fn goodbye() {
//     println!("Goodbye, world!");
// }

//2. Devolver valores
// fn divide_by_5(num: u32) -> u32 {
//     num / 5
// }
// fn main() {
//     let num = 25;
//     println!("{} divided by 5 = {}", num, divide_by_5(num));
// }

//3. return
fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // 0 -> esto no es posible, ya que no es la ultima instrucción debemos usar 'return'
        return 0; // al usar return hemos de poner ;
    }
    num / 5
}
fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}