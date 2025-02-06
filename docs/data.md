# Rust apuntes 🖊️
## Introducción
Rust, lenguaje de programación enfocado en sistemas. Creado por Modzilla, para Firefox.

Multi-paradigma.

- Cargo: administrador de paquetes de Rust

- Casos de uso: SWC(compilador js/ts 20x/70x mas rapido que Babel, -lo usan: Nextjs, Deno, Shopify, npm), compila a WebAssembly, NO es especialmente bueno con interfaces de usuario básicas..

Su gran competidor es GoLang.

- [Basado en este curso](https://learn.microsoft.com/es-es/training/paths/rust-first-steps/?source=learn)

## Teoría
### 1. Compilando y ejecutando
#### 1.1. rustc
Con rustc podemos directamente compilar o ejecutar nuestro código
- compilar: `rustc <nombre_ubicación_archivo>`
- ejecutar: `<nombre_ubicacion_archivo>.exe`
#### 1.2. cargo
Con cargo podemos crear el proyecto de carga y que el compile y ejecute el codigo por nosotros.
- crear proyecto: `cargo new <nombre_archivo>`
- compilar y ejecutar: `cargo run`
### 2. Rust introducción
#### [2.1. primeras macros y variable let](../src/basic-structure/src/main.rs)
- macros: `todo!()` y `println!()`
#### [2.2. mut](../src/mut-key/src/main.rs)
#### 2.3. shadowing
**Example**
```rs
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num" 
let shadow_num = shadow_num + 5; 

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);
```

**Uso de memoria**

En Rust, cuando se hace **shadowing** (sombreado de variables), la variable anterior deja de ser accesible en ese ámbito, pero **no sigue ocupando memoria indefinidamente**.  

##### ¿Qué ocurre realmente?  

Cada nueva declaración con `let` crea una nueva variable que **reemplaza la anterior en el mismo ámbito**, pero **Rust maneja esto de manera eficiente**:  

1. Si la variable anterior **no es usada después del sombreado**, el compilador puede optimizar el código y eliminarla completamente.  
2. Si la variable anterior contenía un valor en el **heap** (por ejemplo, un `String` en lugar de un `i32`), Rust se asegura de liberar la memoria cuando la variable queda inaccesible.  

En el ejemplo que diste:  

```rust
let shadow_num = 5;
let shadow_num = shadow_num + 5;
let shadow_num = shadow_num * 2;
```

Cada `let` crea una nueva variable en **stack**, y las versiones anteriores simplemente **dejan de existir** en términos de acceso. Rust optimiza esto de manera segura, evitando desperdicio de memoria.  

##### ¿Cuándo puede ser problemático?  

Si trabajas con **tipos en el heap**, como `Vec<String>`, y haces sombra de la variable en el mismo ámbito sin soltarla antes, puede que estés reteniendo memoria más de lo necesario. Pero en general, Rust maneja esto bien gracias a su **sistema de ownership y lifetimes**.  

En resumen: **no hay fugas de memoria por el shadowing en Rust**, y en la mayoría de los casos, es seguro y útil. 🚀
#### [2.4. tipos](../src/tipos-intro/src/main.rs)
*Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante anotaciones de tipo.*
##### Tipos `escalares`
Tipos de datos primitivos integrados para expresar números, texto y veracidad. Algunos de estos tipos se conocen como escalares, porque representan un solo valor:
- Números enteros
- Números de punto flotante
- Valores booleanos
- Characters

[Principales tipos para números](./img/tipos-intro.png)
##### Texto
Dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. Todos los tipos de texto son representaciones UTF-8 válidas.

**`char`**

El tipo char es el más primitivo de los tipos de texto. El valor se especifica poniendo el elemento entre comillas simples.

*El tipo char de Rust contiene puntos de código Unicode, pero no usa la codificación UTF-8. char en Rust es un entero de 21 bits que se ha agregado para ampliar a 32 bits.*

**cadenas de texto**

*El tipo str, también conocido como segmento de cadena, es una vista de los datos de la cadena. La mayoría de las veces, se hace referencia a estos tipos usando la sintaxis del estilo de referencia que precede al tipo con el símbolo de y comercial &str. Trataremos las referencias en los siguientes módulos. Por ahora, puede imaginarse &str como un puntero a datos de cadena inmutables. Los literales de cadena son todos de tipo &str.*

*Aunque los literales de cadena son convenientes para usarlos en ejemplos de introducción de Rust, no son adecuados para todas las situaciones en las que podríamos querer usar texto. No todas las cadenas pueden conocerse en tiempo de compilación. Un ejemplo se da cuando un usuario interactúa con un programa en tiempo de ejecución y envía texto mediante un terminal.*

*En estos escenarios, Rust tiene un segundo tipo de cadena denominado String. Este tipo se asigna en el montón* ('heap' en C++) *. Cuando se usa el tipo String, no es necesario conocer la longitud de la cadena (número de caracteres) antes de compilar el código.*

*En realidad, Rust tiene más de dos tipos de cadena. En este módulo, solo se describen los tipos String y &str.*


#### [2.5. colecciones de datos](../src/tuplas/src/main.rs)

##### tuplas

Agrupación de valores de distintos tipos recopilados en un valor compuesto. Los valores individuales de una tupla se denominan elementos. Los valores se especifican como una lista separada por comas entre paréntesis (<value>, <value>, ...).

Una tupla tiene una longitud fija, que es igual a su número de elementos. Una vez declarada una tupla, no puede aumentar ni reducir su tamaño. No se pueden agregar ni quitar elementos. El tipo de datos de una tupla se define mediante la secuencia de los tipos de datos de los elementos.

##### `struct`
Tipo compuesto por otros tipos. Los elementos de una estructura se denominan campos. Al igual que las tuplas, los campos de una estructura pueden tener tipos de datos diferentes. Una ventaja importante del tipo de estructura es que puede asignar un nombre a cada campo, por lo que queda claro lo que significa el valor.

Para trabajar con estructuras en un programa con Rust, en primer lugar debe definir la estructura por nombre y especificar el tipo de datos de cada campo. Después, debe crear una instancia de la estructura con otro nombre. Al declarar la instancia, se proporcionan los valores específicos para los campos.

- Las estructuras de C clásicas son las más utilizadas. Cada campo de la estructura tiene un nombre y un tipo de datos. Una vez definida una estructura clásica, se puede acceder a los campos de la estructura usando la sintaxis <struct>.<field>.
- Las estructuras de tupla son parecidas a las clásicas, pero sus campos no tienen nombres. A fin de acceder a los campos de una estructura de tupla, usamos la misma sintaxis que para indexar una tupla: <tuple>.<index>. Al igual que con las tuplas, los valores de índice de la estructura de tupla empiezan por cero.
- Las estructuras de unidad suelen usarse como marcadores. Obtendremos más información sobre por qué las estructuras pueden resultar útiles cuando descubramos la característica rasgos de Rust.

**Definición:**
- Estructura Clásica: El cuerpo de una estructura clásica se define entre llaves {}. A cada campo de la estructura clásica se le asigna un nombre único dentro de la estructura. El tipo de cada campo se especifica con la sintaxis : <type>. Los campos de la estructura clásica se especifican como una lista separada por comas <field>, <field>, ....
- Estructura Tupla:  el cuerpo de una estructura de tupla se define entre paréntesis (). Los paréntesis van inmediatamente después del nombre de la estructura. No hay espacio entre el nombre de la estructura y el paréntesis de apertura. A diferencia de una tupla, la definición de estructura de tupla incluye solo el tipo de datos de cada campo. Los tipos de datos de la estructura de tupla se especifican como una lista separada por comas <type>, <type>, ....

#### [2.6. variantes `enum` para datos compuestos](../src/enumeracion/src/main.rs)

*Las enumeraciones son tipos que pueden ser una de varias variantes. Lo que Rust denomina enumeraciones se conocen habitualmente como tipos de datos algebraicos. Lo importante es que cada variante de enumeración puede tener datos asociados.*

##### Definición
En el ejemplo siguiente, se define una enumeración para clasificar un evento web. Cada variante de la enumeración es independiente y almacena diferentes cantidades y tipos de valores.

```rs
enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enum variant can be like a classic struct with named fields and their data types
    WEClick { x: i64, y: i64 }
}
```

La enumeración de nuestro ejemplo tiene tres variantes de tipos diferentes:

- WELoad no tiene ningún tipo de datos o datos asociados.
- WEKeys tiene dos campos, con tipos de datos String y char.
- WEClick incluye una estructura anónima con campos con nombre x y y, y sus tipos de datos (i64).


Definimos una enumeración con variantes parecidas a la forma en que definimos diferentes clases de tipos de estructura. Todas las variantes se agrupan en el mismo tipo de enumeración WebEvent. Cada variante de la enumeración no es su propio tipo. Cualquier función que use una variante de la enumeración WebEvent debe aceptar todas las variantes de esta. No podemos tener una función que acepte solo la variante WEClick, pero no las demás.

Una manera de evitar los requisitos de variante de la enumeración es definir una estructura independiente para cada variante de esta. Después, cada variante de la enumeración usa la estructura correspondiente. La estructura contiene los mismos datos que tenía la variante de enumeración correspondiente. Este estilo de definición nos permite hacer referencia a cada variante lógica por sí misma.

En el código siguiente se muestra cómo utilizar este estilo de definición alternativo. Las estructuras se definen para contener los datos. Las variantes de la enumeración se definen para hacer referencia a las estructuras.

```rs
// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick { x: i64, y: i64 }

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
```

##### Creación instancia
Ahora vamos a agregar código para crear instancias de nuestras variantes de enumeración. Para cada variante, usamos la palabra clave let a fin de realizar la asignación. Para acceder a la variante específica en la definición de enumeración, usamos la sintaxis <enum>::<variant> con dos puntos dobles ::.

**Variante simple: WELoad(bool)**
La primera variante de la enumeración WebEvent tiene un único valor booleano, WELoad(bool). Creamos una instancia de esta variante de forma parecida a como hemos trabajado con los valores booleanos de la unidad anterior:

```rust
let we_load = WebEvent::WELoad(true);
Variante de estructura: WEClick(MouseClick)
```
**Variante de estructura: WEClick(MouseClick)**

La segunda variante incluye una estructura clásica WEClick(MouseClick). La estructura tiene dos campos con nombre x y y, y ambos campos tienen el tipo de datos i64. Para crear esta variante, en primer lugar creamos una instancia de la estructura. Después, pasamos la estructura como argumento en la llamada para crear una instancia de la variante.

```rs
// Instantiate a MouseClick struct and bind the coordinate values
let click = MouseClick { x: 100, y: 250 };

// Set the WEClick variant to use the data in the click struct
let we_click = WebEvent::WEClick(click);
```

**Variante de tupla: WEKeys(KeyPress)**
La última variante incluye una tupla WEKeys(KeyPress). La tupla tiene dos campos que usan los tipos de datos String y char. Para crear esta variante, primero creamos una instancia de la tupla. Después, pasamos la tupla como argumento en la llamada para crear una instancia de la variante.

```rs
// Instantiate a KeyPress tuple and bind the key values
let keys = KeyPress(String::from("Ctrl+"), 'N');
    
// Set the WEKeys variant to use the data in the keys tuple
let we_key = WebEvent::WEKeys(keys);
```

Observe que usamos la sintaxis String::from("<value>") en este fragmento de código. Esta sintaxis crea un valor de tipo String llamando al método from de Rust. El método espera un argumento de entrada de datos entre comillas dobles.
##### trait `Debug`
 `Debug` permite imprimir estructuras (`struct`) y enumeraciones (`enum`) de manera legible en la consola, lo que es útil para depuración.  

**Ejemplo sin `Debug` (Falla ❌)**
```rust
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User { name: String::from("Alice"), age: 25 };
    println!("{:?}", user); // ❌ ERROR: `User` no implementa `Debug`
}
```
⬆️*Este código dará un error porque `User` no implementa `Debug`.*


**Ejemplo con `Debug` (Funciona ✅)**
```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User { name: String::from("Alice"), age: 25 };
    println!("{:?}", user); // ✅ Imprime: User { name: "Alice", age: 25 }
}
```

📌 También puedes usar `{:#?}` para un formato más bonito:  
```rust
println!("{:#?}", user);
```
Esto imprime:
```
User {
    name: "Alice",
    age: 25,
}
```

🔥 **Conclusión:** `#[derive(Debug)]` hace que puedas imprimir structs y enums de forma clara sin necesidad de escribir una implementación manual. 🚀

#### [2.7. funciones](../src/funcs/src/main.rs)

### 3. Condiciones
#### [3.1. Matrices](../src/matriz/src/main.rs) 
*Colección de objetos del mismo tipo, que se almacenan secuencial-mente en la memoria. La longitud o tamaño de una matriz es igual al número de elementos que esta contiene. El tamaño de una matriz se puede especificar en el código o calcularse mediante el compilador.*
#### [3.2. Vectores]()
*Almacenan varios valores que tienen el mismo tipo de datos. A diferencia de las matrices, el tamaño o la longitud de un vector puede aumentar o reducirse en cualquier momento. La capacidad de cambiar el tamaño con el tiempo está implícita en tiempo de compilación. Por lo tanto, Rust no puede impedir que se acceda a una posición no válida en el vector como lo hace para el acceso en matrices fuera de los límites.*
- Una manera común de declarar e inicializar un vector es con la macro vec!.
