
//4. Utilizamos la funcion send_data_as_json

fn main() {
    //4.
    // let laura = Person {
    // 	name: String::from("Laura"),
	//     age: 31,
	//     favorite_fruit: String::from("apples"),
    // };

    // let fido = Dog {
	//     name: String::from("Fido"),
	//     color: String::from("Black"),
	//     likes_petting: true,
    // };

    // send_data_as_json(&laura);
    // send_data_as_json(&fido);
    //5. pasamos a la función un tipo que no implementa el rasgo esperado
    struct Cat {
        name: String,
        sharp_claws: bool,
    }
    
    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };
    
    send_data_as_json(&kitty);

}

//1.Podemos declarar argumentos de función como un parámetro de tipo anónimo, donde el destinatario debe proporcionar un tipo que tenga los límites declarados por el parámetro de tipo anónimo.

trait AsJson {
    fn as_json(&self) -> String;
}

//2.escribir una función que acepte cualquier tipo que implemente el rasgo AsJson. Se escriben como impl seguido de un conjunto de límites de rasgo.

// fn send_data_as_json<T: AsJson>(value: &T) { ... }


fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

//3. (Creamos las estructuras para los metodos)Podemos entonces declarar nuestros tipos e implementar el rasgo AsJson para ellos:

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
	    format!(
	        r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
	        self.name, self.age, self.favorite_fruit
	    )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
	    format!(
	        r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
	        self.name, self.color, self.likes_petting
	    )
    }
}
