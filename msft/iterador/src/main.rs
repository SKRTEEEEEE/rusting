//4. Usamos el metodo `next` para implementar nuestro Counter
// fn main() {
//     let mut counter = Counter::new(6);
//     println!("Counter just created: {:#?}", counter);

//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), Some(6));
//     assert_eq!(counter.next(), None);
//     assert_eq!(counter.next(), None);  // further calls to `next` will return `None`
//     assert_eq!(counter.next(), None);

//     println!("Counter exhausted: {:#?}", counter);
// }
//5. Usamos la 'propiedad' `Iterator` para implementar un bucle for
fn main() {
    //5.
    // for number in Counter::new(10) {
    //     println!("{}", number);
    // }
    //6. `Iterator` incluye también otros métodos, pero son métodos predeterminados. Se basan en next, por lo que se obtienen de forma gratuita
    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
}

//1. estructura del estado del contador
#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}
//2. Implementamos un método `new` para controlar como debe iniciarse
impl Counter {
    fn new(length: usize) -> Counter {
	    Counter {
	        count: 0,
	        length,
	    }
    }
}
//3. Implementamos el rasgo Iterator de la estructura Counter. Contaremos con "usize", por lo que declaramos que el tipo Item asociado debe ser de ese tipo.
impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
    
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}
//⬆️El método next() es el único método necesario que hay que definir. Dentro de su cuerpo, se incrementa el recuento en uno en cada llamada (por eso se ha empezado en cero). Luego comprobamos si hemos terminado el recuento o no. Usamos la variante Some(value) del tipo Option para expresar que la iteración sigue produciendo resultados y la variante None para expresar que la iteración debe detenerse.
