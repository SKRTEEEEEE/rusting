mod car_factory {

    // fn build_car() {
    //     println!("Honk honk!");
    // }
    //1. modificamos build_car
    pub fn build_car(){
        println!("Honk honk!");
    }
}

fn main() {
    car_factory::build_car();
}