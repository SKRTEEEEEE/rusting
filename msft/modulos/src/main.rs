mod authentication {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    //3. EXTRA ⬆️ funcion hash
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }    
            //2. Creamos las funciones para acceder
        pub fn get_username(&self) -> &String {
            &self.username
        }
        pub fn get_password_hash(&self) -> &u64 {
            &self.password_hash
        }
        pub fn set_password(&mut self, new_password: &str){
            self.password_hash = hash_password(new_password)
        } 
    }
    fn hash_password(input: &str) -> u64 { /*...*/
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    //3. EXTRA ⬆️ funcion hash
    }
    
    
}

fn main() {

    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password_hash());
    //1. ⬆️ No se puede acceder porque son privados

}