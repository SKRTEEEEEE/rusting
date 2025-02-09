fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #[test]
// fn add_works() {
//     assert_eq!(add(1, 2), 3);
//     assert_eq!(add(10, 12), 22);
//     assert_eq!(add(5, -2), 3);
// }

// //2. prueba con errores

// #[test]
// #[should_panic]
// //3. ⬆️ añadimos should_panic, ya que queremos que no salte un error cuando haya un panic! y que salte cuando no lo haya

// fn add_fails() {
//     assert_eq!(add(2, 2), 7);
// }

// //4. añadimos una prueba omitida
// #[test]
// #[ignore = "not yet reviewed by the Q.A. team"]
// fn add_negatives() {
//     assert_eq!(add(-2, -2), -4)
// }

//5. Módulo de pruebas
#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}