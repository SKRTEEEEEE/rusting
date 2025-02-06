fn main() {
    bucle_for_basico();
    bucle_for_puntos();
}
pub fn bucle_for_puntos(){
    for number in 0..5 {
        println!("{}", number * 2);
    }
}
pub fn bucle_for_basico(){
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
}
pub fn bucle_while() {
    let mut counter = 1;

    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }
}
pub fn bucle_loop() {
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);
}