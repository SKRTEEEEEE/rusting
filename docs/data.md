# Rust apuntes 🖊️
## Introducción
Rust, lenguaje de programación enfocado en sistemas. Creado por Modzilla, para Firefox.

Multi-paradigma.

- Cargo: administrador de paquetes de Rust

- Casos de uso: SWC(compilador js/ts 20x/70x mas rapido que Babel, -lo usan: Nextjs, Deno, Shopify, npm), compila a WebAssembly, NO es especialmente bueno con interfaces de usuario básicas..

Su gran competidor es GoLang.

- [Basado en este curso](https://learn.microsoft.com/es-es/training/paths/rust-first-steps/?source=learn)



## Tips
### No utilizar '-' para los nombres de los archivos
Al utilizar '-', en los nombres de nuestros archivos, podemos incurrir en muchos errores:
- Las libs no pueden ser referenciadas si contienen un guión en el nombre: [ver ejercicio_14](../src/ejercicio_14/src/lib.rs)




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
#### [3.2. Vectores](../src/vectores/src/main.rs)
*Almacenan varios valores que tienen el mismo tipo de datos. A diferencia de las matrices, el tamaño o la longitud de un vector puede aumentar o reducirse en cualquier momento. La capacidad de cambiar el tamaño con el tiempo está implícita en tiempo de compilación. Por lo tanto, Rust no puede impedir que se acceda a una posición no válida en el vector como lo hace para el acceso en matrices fuera de los límites.*
- Una manera común de declarar e inicializar un vector es con la macro vec!.
#### [3.3. `if`/`else`](../src/if-else/src/main.rs)

### 4. Bucles
#### [4.1. Mapas hash](../src/hashmap/src/main.rs)
#### [4.2. `loop`, `while` y `for`](../src/bucles/src/main.rs)

### 5. Errores
#### 5.1. `panic!`
El uso de alertas de pánico es el mecanismo más sencillo de control de errores de Rust.

Puede usar la macro panic! para emitir una alerta de pánico para el subproceso actual. La macro imprime un mensaje de error, libera recursos y, luego, sale del programa.

En este sencillo ejemplo se muestra cómo llamar a la macro panic!:

```rs
fn main() {
    panic!("Farewell!");
}
```

    👁️ Rust entra en pánico en algunas operaciones, como una división por cero o un intento de acceder a un índice que no se ha enviado previamente en una matriz, un vector o un mapa hash
#### [5.2. Option](../src/option/src/main.rs)
La biblioteca estándar de Rust proporciona una enumeración Option<T> que se usa cuando la ausencia de un valor es una posibilidad.

*En muchos otros lenguajes, la ausencia de un valor se modelaría con null o nil, pero Rust no usa null fuera del código que inter-opera con otros lenguajes. Rust es explícito acerca de cuándo un valor es opcional. Aunque en muchos lenguajes una función que toma String podría tomar String o null, en Rust esa misma función solo puede un elemento String real. Si quiere modelar una cadena opcional en Rust, debe encapsularla explícitamente en un atributo Option tipo Option<String>.*

```rs
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
```

`None` y `Some` no son tipos sino variantes del tipo `Option<T>`, lo que significa, entre otras cosas, que las funciones no pueden tomar `Some` o `None` como argumentos, sino solo `Option<T>`.


*El intento de acceder al índice no existente de un vector haría que el programa emitiera una alerta panic, sin embargo, podría evitarlo mediante el método Vec::get, que devuelve un tipo Option en lugar de un error. Si el valor existe en un índice especificado, se encapsula en la variante Option::Some(value). Si el índice está fuera de los límites, devolverá en cambio un valor Option::None.*

##### `match`
- Las secciones match se evalúan de arriba abajo. Los casos específicos se deben definir antes que los casos genéricos o nunca se buscará una coincidencia para ellos ni se evaluarán.
- Las secciones match deben cubrir todos los valores posibles que pueda tener el tipo de entrada. Si intenta buscar coincidencias con una lista de patrones no exhaustiva, recibirá un error de compilador.
##### `if let`
Un operador if let compara un patrón con una expresión. Si la expresión coincide con el patrón, se ejecuta el bloque if. Lo bueno de la expresión if let es que no se necesita todo el código re-utilizable de una expresión match cuando solo interesa un patrón con el que buscar coincidencias.

##### `unwrap`
Puede intentar acceder al valor interno de un tipo Option directamente mediante el método unwrap. Sin embargo, tenga cuidado, ya que este método emitirá una alerta de pánico si la variante es None.
##### `expect`
El método expect hace lo mismo que unwrap, pero emite un mensaje de pánico personalizado que su segundo argumento proporciona
##### `expect_or`
    ⚠️ Como unwrap y expect pueden emitir alertas de pánico, no se recomienda usarlas. Use la coincidencia de patrones y administre el caso None explícitamente. O use metodos como `expect_or`.

Devuelve un valor predeterminado si la variante es None o el valor interno si la variante es Some(value).
#### 5.3. Result
Rust proporciona la enumeración Result<T, E> para devolver y propagar errores. Por convención, la variante Ok(T) representa un acierto y contiene un valor, y la variante Err(E) representa un error y contiene un valor de error.

La enumeración Result<T, E> se define como:

```rs
enum Result<T, E> {
    Ok(T),  // A value T was obtained.
    Err(E), // An error of type E was encountered instead.
}
```

A diferencia del tipo Option, que describe la posibilidad de la ausencia de un valor, el tipo Result es más adecuado siempre que se puedan producir errores.

El tipo Result también tiene los métodos unwrap y expect, los cuales:

- Devuelven el valor dentro de la variante Ok.
- Ocasionan alertas de pánico en el programa, si la variante es Err.

**`#[derive(Debug)]` es una macro que indica al compilador de Rust que convierta el tipo en imprimible con fines de depuración.**


### 6. Memoria
#### [6.1. Propiedad](../src/propiedad/src/main.rs)
    👁️ En Rust, las "variables" se suelen denominar "enlaces". Esto se debe a que las "variables" de Rust no son muy variables: no cambian con frecuencia, ya que son inmutables de manera predeterminada. Por el contrario, a menudo pensamos que los nombres están "enlazados" a los datos, de ahí el nombre "enlace". En este módulo, usaremos los términos "variable" y "enlace" indistintamente.
##### Reglas de ámbito
Las variables solo son válidas dentro de un ámbito determinado. En Rust, los ámbitos normalmente se indican con llaves {}. Los ámbitos comunes incluyen cuerpos de función y ramas if, else y match.
##### Anulación
Cada vez que un objeto sale del ámbito, se "descarta". El descarte de una variable libera todo los recursos asociados a ella. En el caso de las variables de archivos, el archivo termina cerrado. En el caso de las variables que tienen asignada memoria asociada a ellas, se libera la memoria.
##### Transferencia de recursos
Cuando se quiere transferir la propiedad de un elemento de un enlace a otro.

```rs
{
    let mascot = String::from("ferris");
    // transfer ownership of mascot to the variable ferris.
    let ferris = mascot;
}
// ferris is dropped here. The string data memory will be freed here.
```

**Una cuestión clave que se debe comprender es que, una vez transferida la propiedad, la variable antigua ya no es válida.**
En Rust, "transferir la propiedad" se conoce como "mover".

##### Propiedad en las funciones
En Rust, la transferencia de propiedad (es decir, la transferencia) es el comportamiento predeterminado.

##### Copia 'implícita'
Los tipos simples como los número son tipos de copia. Implementan el rasgo `Copy`, lo que significa que se copian en lugar de moverse. La misma acción se produce para la mayoría de los tipos simples. La copia de números es muy económica, por lo que tiene sentido que estos valores se copien. La copia de cadenas o vectores, u otros tipos complejos, puede ser costosa, por lo que no implementan el rasgo `Copy` y, en su lugar, se mueven.
##### Copia explícita `.clone()`
Una llamada a .clone duplica la memoria y genera un nuevo valor. El nuevo valor se mueve, lo que significa que todavía se puede usar el valor anterior.

    👁️ Este enfoque puede resultar útil, aunque puede ralentizar el código, ya que cada llamada a clone realiza una copia completa de los datos. Este método a menudo incluye asignaciones de memoria u otras operaciones costosas. Estos costos se pueden evitar si los valores "se toman prestados" mediante referencias.

#### [6.2. Referencias](../src/referencias/src/main.rs)
    🧠 Los valores tienen propietarios. Para transferir la propiedad de un valor, se cambia de una variable a otra. La propiedad no se puede transferir para los tipos que implementan el rasgo Copy, como para valores simples como números.
    Los valores también se pueden copiar de forma explícita mediante el proceso de clonación. Se llama al método clone y se obtienen nuevos valores que se copian, lo que conserva los valores originales y permite seguir utilizándolos.

Las referencias permiten "tomar prestados" valores sin convertirse en propietario de ellos.
##### Prestamos mutables
Con los préstamos &, conocidos como "préstamos inmutables", se pueden leer los datos, pero no cambiarlos. Con los préstamos de &mut, conocidos como "préstamos mutables", los datos se pueden leer y cambiar.

    🧠 También necesitamos declarar el valor original como mutable

##### **Limites**
Al tomar prestado un valor de cualquier tipo T, se aplican las reglas siguientes:

El código debe implementar cualquiera de las definiciones siguientes, pero no las dos al mismo tiempo:

- Una o más referencias inmutables (&T)
- Exactamente una referencia mutable (&mut T)

*Se podría resumir como que: "Cuando usamos una referencia mutable (&mut T), no puede existir ninguna otra referencia (ni mutable ni inmutable) al mismo tiempo."*

<details>
<summary> 💻 ChatGPT 🧑‍🏫 ➕ ❓</summary>

**Explicación:**

1. **Varias referencias inmutables (`&T`) están permitidas** simultáneamente porque no modifican el valor original.
2. **Una referencia mutable (`&mut T`) es exclusiva**, es decir, mientras exista una, no puede haber ninguna otra referencia (ni inmutable ni mutable).

Ejemplo correcto:
```rust
fn main() {
    let mut x = 10;

    let r1 = &x;
    let r2 = &x; // Se permite porque ambas son referencias inmutables

    println!("r1: {}, r2: {}", r1, r2);
}
```

Ejemplo incorrecto:
```rust
fn main() {
    let mut x = 10;

    let r1 = &x;
    let r2 = &mut x; // ❌ Error: no puedes tener una referencia mutable mientras haya una inmutable

    println!("r1: {}", r1);
}
```

Ejemplo correcto con referencia mutable:
```rust
fn main() {
    let mut x = 10;

    {
        let r = &mut x;
        *r += 5; // Se permite porque `r` es la única referencia activa a `x`
    } // `r` deja de existir aquí, liberando el acceso a `x`

    let r2 = &x; // Se permite porque ya no hay referencia mutable activa
    println!("x: {}", r2);
}
```

**Resumen:**

✔ **Múltiples referencias inmutables (`&T`) al mismo tiempo → OK**  
✔ **Solo una referencia mutable (`&mut T`) y sin otras referencias al mismo tiempo → OK**  
❌ **Referencia mutable y referencias inmutables coexistiendo → ERROR**  

</details>

#### [6.3. Validación de referencias mediante el uso de duraciones](../src/duraciones/src/main.rs)

    🧠 PUNTERO COLGANTE: El uso de referencias plantea un problema. El elemento al que se refiere una referencia no realiza un seguimiento de todas sus referencias. Este comportamiento puede generar un problema: cuando se anula el elemento y sus recursos se liberan, ¿cómo se puede tener la seguridad de que no haya referencias que apunten a la memoria ya liberada (y, por tanto, no válida)?

Rust elimina este problema. Garantiza que todas las referencias siempre se refieran a elementos válidos.   

Las duraciones permiten que Rust garantice la seguridad de la memoria sin los costos de rendimiento de la recolección de elementos no utilizados.
##### Anotación de las duraciones en funciones
<details>
<summary> 💻 ChatGPT 🧑‍🏫 ➕ ❓</summary>

📌 **Resumen Teórico sobre Anotación de Lifetimes en Funciones en Rust**  

Rust utiliza **anotaciones de tiempo de vida (`lifetimes`)** para garantizar la seguridad de memoria sin un recolector de basura. Estas anotaciones son necesarias cuando el compilador **no puede inferir automáticamente** la duración de las referencias en una función.

---

📍 **¿Por qué se necesitan Lifetimes en funciones?**  
Cuando una función recibe referencias como parámetros y devuelve una referencia, Rust necesita saber cuánto tiempo debe vivir la referencia de retorno en relación con las referencias de entrada.  

Si la duración de la referencia retornada no está clara, el compilador **restringe la compilación para evitar referencias colgantes**.

---

📍 **Reglas de Lifetimes en Funciones**

1️⃣ **Si una función devuelve una referencia, debe asegurarse de que la referencia de retorno tenga una duración igual o menor que las referencias de entrada.**  
2️⃣ **Si hay múltiples referencias de entrada, el lifetime debe especificar cuál de ellas se usará en la salida.**  
3️⃣ **Cuando hay solo un parámetro de referencia, Rust puede inferir el lifetime en muchos casos, pero con múltiples referencias suele ser necesario anotarlo.**  

---

📍 **Casos en los que se requieren Lifetimes**
✅ **Ejemplo donde es obligatorio usar un lifetime**
```rust
fn mayor<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```
🔹 Aquí `s1` y `s2` tienen el **mismo lifetime `'a`**, garantizando que la referencia retornada **no viva más** que ambas.  
🔹 Sin el lifetime `'a`, el compilador no sabría cuál referencia sobrevive y daría un error.

❌ **Ejemplo donde Rust infiere el lifetime (sin necesidad de anotarlo)**
```rust
fn longitud(s: &str) -> usize {
    s.len() // No se devuelve una referencia, no necesita lifetime
}
```
🔹 Como `s.len()` devuelve un `usize`, **no hay referencias en la salida**, por lo que **no se necesita un lifetime explícito**.

✅ **Ejemplo donde Rust **no** puede inferir el lifetime automáticamente**
```rust
fn primera_palabra(s: &str) -> &str {
    let palabras: Vec<&str> = s.split_whitespace().collect();
    palabras[0] // ❌ Error: la referencia a `palabras` se elimina cuando la función termina.
}
```
🔹 Aquí, la referencia retornada apunta a un vector local que **se elimina al salir de la función**, causando una referencia colgante.

---

📌 **Resumen Final**
- Los **lifetimes en funciones** permiten garantizar que **las referencias devueltas sean válidas** durante el tiempo correcto.
- Rust **puede inferir lifetimes** en algunos casos, pero cuando hay múltiples referencias en los parámetros y una referencia en la salida, **se requiere anotación manual**.
- **Regla clave:** La referencia retornada **no puede vivir más que sus parámetros de entrada**.
  
Rust usa este sistema para **garantizar la seguridad de memoria sin necesidad de un recolector de basura**. 🚀

</details>

##### Anotación de las duraciones en tipos 
Cada vez que un struct o una enumeración contienen una referencia en uno de sus campos, debemos anotar esa definición de tipo con la duración de cada referencia que lleve a cabo con ella.

### 7. Tipos y rasgos genéricos
#### 7.1. Tipos de datos genéricos
Un tipo de datos genérico es un tipo que se define en términos de otros tipos parcialmente desconocidos. 
Ejemplos 'implícitos':
- La enumeración Option<T> es genérica con respecto al tipo T, que es el valor que contiene su variante Some.
- El valor Result<T, E> es genérico tanto en el tipo correcto como en el error, que contiene sus variantes Ok y Err, respectivamente.
- El tipo de vector Vec<T>, el tipo de matriz [T; n] y el mapa hash HashMap<K, V> son genéricos con respecto a los tipos que contienen.
*Pueden existir mas de un tipo genérico*
#### 7.2. Definición del comportamiento compartido con rasgos
Un rasgo es una interfaz común que puede implementar un grupo de tipos.

    💡 Podríamos verlo en typescript, como las --dev-deps que instalamos cuando hacemos @types/[...]

Cada definición de rasgo es una colección de métodos definidos para un tipo desconocido, que normalmente representa una capacidad o un comportamiento que pueden realizar sus implementadores.

Para implementar un rasgo para un tipo, usamos las palabras clave impl Trait for Type, donde Trait es el nombre del rasgo que se implementa y Type es el nombre de la estructura del implementador o la enumeración.

Dentro del bloque impl, se colocan las signaturas de método que la definición de rasgo requiere y se rellena el cuerpo del método con el comportamiento específico que se quiere que tengan los métodos del rasgo para el tipo determinado.

Cuando un tipo implementa un rasgo determinado, promete mantener su contrato. Después de implementar el rasgo, podemos llamar a los métodos en instancias de Circle y Rectangle de la misma forma que llamamos a métodos normales de la siguiente manera

##### Uso de `self`
<details>
<summary> 💻 ChatGPT 🧑‍🏫 ➕ ❓</summary>

📌 **¿Qué hace `&self` en un trait en Rust?**  

En Rust, cuando definimos un **trait**, usamos `&self` dentro de los métodos para indicar que el método opera sobre una referencia al **propio objeto** que implementa el trait.  

---

📍 **Explicación de `&self` en el contexto de Traits**
```rust
trait Area {
    fn area(&self) -> f64;
}
```
🔹 `&self` es **una referencia inmutable** al objeto que implementa el trait.  
🔹 Indica que la función **no modifica el estado interno del objeto**.  
🔹 Permite que el método acceda a los datos internos de `self` sin tomar propiedad ni mutarlos.  

---

📍 **Ejemplo de Implementación**
```rust
struct Circulo {
    radio: f64,
}

impl Area for Circulo {
    fn area(&self) -> f64 {
        3.14 * self.radio * self.radio
    }
}

fn main() {
    let c = Circulo { radio: 5.0 };
    println!("Área del círculo: {}", c.area());
}
```
🔹 `c.area()` llama al método `area` sin mover `c`, ya que `&self` solo toma una referencia.  
🔹 No se puede modificar `self.radio` dentro de `area()` porque `&self` es inmutable.  

---

📍 **Otras Variaciones de `self` en Traits**
| Variante      | Descripción |
|--------------|-------------|
| `fn area(self) -> f64` | Toma **propiedad** del objeto (lo consume). |
| `fn area(&mut self) -> f64` | Toma una referencia **mutable**, permitiendo modificar `self`. |

Ejemplo con `&mut self`:
```rust
trait Contador {
    fn incrementar(&mut self);
}

struct Numero {
    valor: i32,
}

impl Contador for Numero {
    fn incrementar(&mut self) {
        self.valor += 1;
    }
}
```
🔹 `&mut self` permite modificar `self.valor` dentro del método.  

---

📌 **Resumen**
- `&self` en un trait indica que el método opera sobre **una referencia inmutable** del objeto.
- Permite leer los datos sin tomar posesión ni modificar el estado del objeto.
- Otras opciones incluyen `self` (consumo del objeto) y `&mut self` (permite mutación).  

Si una función en un `trait` o `impl` toma **varios parámetros**, `self` sigue siendo obligatorio como primer parámetro (si el método opera sobre una instancia), pero los demás parámetros pueden tener cualquier nombre.  

---

 📍 **Ejemplo con varios parámetros**
```rust
trait Calculadora {
    fn suma(&self, a: i32, b: i32) -> i32;
}
```
🔹 `&self` es el **primer parámetro obligatorio**.  
🔹 `a` y `b` pueden tener **cualquier nombre válido**.  

---

 📍 **Ejemplo con implementación**

```rust
struct Operacion;

impl Calculadora for Operacion {
    fn suma(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let op = Operacion;
    println!("Suma: {}", op.suma(5, 7));  // Output: Suma: 12
}
```
📌 `op.suma(5, 7)` usa `&self` para acceder a la instancia `op`.  

---

 📍 **Ejemplo con `&mut self` y varios parámetros**

```rust
trait Contador {
    fn incrementar(&mut self, cantidad: i32);
}

struct Numero {
    valor: i32,
}

impl Contador for Numero {
    fn incrementar(&mut self, cantidad: i32) {
        self.valor += cantidad;
    }
}

fn main() {
    let mut n = Numero { valor: 10 };
    n.incrementar(5);
    println!("Nuevo valor: {}", n.valor);  // Output: Nuevo valor: 15
}
```
🔹 `&mut self` permite modificar `self.valor`.  
🔹 `cantidad` es un parámetro adicional con **cualquier nombre válido**.  

---

 📌 **Conclusión**
- `self` sigue siendo **obligatorio como primer parámetro** si el método opera sobre la instancia.  
- Los **demás parámetros** pueden llamarse como quieras (`a`, `cantidad`, `nombre`, etc.).  
- `self` puede ser `self`, `&self` o `&mut self`, dependiendo de si necesitas **mover**, **prestar** o **modificar** la instancia. 🚀

</details>

#### [7.3. Uso del rasgo de derivación](../src/derivacion/src/main.rs)
#### [7.4. Uso de límites de rasgos y funciones genéricas](../src/traits/src/main.rs)
#### [7.5. Uso de iteradores](../src/iterador/src/main.rs)
Todos los iteradores implementan un rasgo denominado `Iterator` que se define en la biblioteca estándar y se utiliza para implementar iteradores en colecciones tales como intervalos, matrices, vectores y mapas hash.
```rs
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Un Iterator tiene un método, `next`, que cuando se llama devuelve un objeto `Option<Item>`. El método `next` devolverá `Some(Item)` siempre y cuando haya elementos. Una vez agotados todos, devolverá None para indicar que la iteración ha finalizado.

En esta definición se usa una sintaxis nueva: type `Item` y `Self::Item`, que definen un tipo asociado a este rasgo.

### 8. Módulos, paquetes y contenedores de terceros
#### 8.1. Conceptos básicos
- Un paquete:
    - Contiene funcionalidad dentro de uno o más contenedores.
    - Incluye información sobre cómo crear esos contenedores. La información está en el archivo Cargo.toml.
    - *Cuando ejecutamos `cargo new <project_name>`, estamos creando un paquete.*
- Un contenedor:
    - Es una unidad de compilación, que es la cantidad más pequeña de código en la que puede operar el compilador de Rust.
    - Una vez compilado, genera un archivo ejecutable o una biblioteca.
    - Contiene un módulo de nivel superior implícito y sin nombre.
- Un módulo:
    - Es una unidad (posiblemente anidada) de organización de código dentro de un contenedor.
    - Puede tener definiciones recursivas que abarcan módulos adicionales.
**Contenedores de bibliotecas**
Para crear una biblioteca, pase el parámetro --lib de la línea de comandos al comando cargo new.

Cuando indique a Cargo que compile este contenedor, obtendrá un archivo de biblioteca denominado libmy_library.rlib que se puede publicar y vincular a otros proyectos.
**Módulos**
Se puede usar para dividir jerárquicamente el código en unidades lógicas que también facilitan la lectura y la reutilización. Los módulos también controlan la privacidad del elemento.
#### [8.2. División del código en módulos](../src/modulos/src/main.rs)
#### [8.3. Separación de módulos en archivos diferentes](../src/modulos-separados/src/main.rs)
*Continuación del apartado 8.2.*
#### [8.4. Adición de contenedores de terceros a un proyecto](../src/contenedores/src/main.rs)
La biblioteca estándar de Rust no tiene un módulo para expresiones regulares, así que vamos a agregar el contenedor `regex` que está disponible en crates.io. Este sitio web es el registro de paquetes central de la comunidad de Rust y sirve como una ubicación para buscar y descargar paquetes.

Siempre que queramos agregar contenedores dependientes a nuestro proyecto, podemos confiar todo el trabajo pesado a Cargo. Para depender de una biblioteca hospedada en [crates.io](https://crates.io/), agréguela al archivo `Cargo.toml`:

```toml
[dependencies]
regex = "1.4.2"
```

Si su archivo `Cargo.toml` aún no tiene una sección `[dependencies]`, agréguela. Después, enumere el nombre del contenedor y la versión que quiere usar.

El siguiente paso consiste en ejecutar el comando `cargo build`. Cargo recuperará la nueva dependencia y todas sus dependencias, y las compilará todas

### 9. Pruebas automatizadas
#### [9.1. Escribir pruebas unitarias](../src/escribir-test/src/main.rs)
Las pruebas unitarias en Rust son funciones simples marcadas con el atributo `#[test]` que comprueban que el código que no es de prueba funciona de la manera esperada. Estas funciones solo se compilan cuando se prueba el código.

Las funciones de prueba ejecutan el código que desea probar. Luego, comprueban los resultados, a menudo mediante las macros `assert!` o `assert_eq!`.

- Ejecutar test: `cargo test` o `cargo t`
##### Errores esperados
En muchos escenarios, es importante probar si una condición producirá una función `panic!`.

El atributo `should_panic` permite comprobar si hay un `panic!`. Si agregamos este atributo a nuestra función de prueba, la prueba se supera cuando el código de la función entra en pánico. Se produce un error en la prueba cuando el código no entra en pánico.
##### Omisión de pruebas
Una función anotada con el atributo `[test]` también se puede anotar con el atributo `[ignore]`. Este atributo hace que se omita la función de prueba durante las pruebas.

El atributo `[ignore]` se puede escribir opcionalmente junto con un motivo para la omisión de la prueba.

##### Módulo de prueba
*El atributo cfg controla la compilación condicional y solo compilará el elemento al que está asociado si el predicado es true. Cargo emite automáticamente la marca de compilación test siempre que se ejecuta el comando $ cargo test, por lo que el predicado siempre será true cuando se ejecuten las pruebas.*

La declaración `use super::*`; es necesaria para que el código del módulo pueda acceder a la función add en el módulo externo.
#### [9.2. Escritura de pruebas de documentación](../src/docu-test/src/lib.rs)
Con Rust, puede ejecutar ejemplos de documentación como pruebas. La forma principal de documentar una biblioteca de Rust es mediante la anotación del código fuente con barras diagonales triples (///), lo que se conoce como comentarios de documentación. Los comentarios de documentación se escriben en Markdown y admiten bloques de código, de modo que estos bloques de código se compilan y se usan como pruebas.

Para probar esta característica, primero debe crear un nuevo proyecto de biblioteca utilizando: `cargo new --lib <nombre_libreria>`

- Ejecutar test documentación: `cargo test` o `cargo t`

#### [9.3. Escritura de pruebas de integración](../src/integracion_tests)
Las pruebas unitarias y de documentación proporcionan pruebas concisas y específicas. Pero también es una buena idea probar nuestro contenedor como un todo. Luego, podemos confirmar que los distintos elementos de código del contenedor funcionan juntos según lo previsto.

Para probar nuestro contenedor como un todo, podemos usar pruebas de integración. El conjunto de pruebas con Rust admite este tipo de prueba, que solo llama a las funciones que contiene la API pública de nuestra biblioteca. Podemos usar pruebas de integración para comprobar cómo funciona nuestro código cuando otros lo usan.

La característica exclusiva de estas pruebas es que se encuentran en un directorio y un archivo independientes, por lo que se pueden usar para probar externamente el código de la biblioteca. Al ejecutar pruebas de integración con Cargo, colóquelas en un directorio de `tests`. Cargo ejecuta cada archivo de origen en este directorio. Cree `tests` en el directorio del proyecto, en el mismo nivel que el directorio src.
### 10. Proyecto final
#### 10.1. Serialización y deserialización de tareas mediante `serde_json`
Cuando necesitemos conservar las estructuras e instancias de enumeraciones, debemos pensar en la serialización. Cuando necesitemos devolver esos datos a un programa, hablaremos de deserialización.

La serialización y deserialización son los procesos de almacenar datos en una secuencia de bytes y, luego, recuperarlos para usarlos posteriormente, sin pérdida de información. Después, puede enviar esos bytes a través de una conexión o almacenarlos en un archivo en un dispositivo de almacenamiento.

    ⚠️ La comunidad de Rust recomienda el contenedor `serde` para controlar la mayoría de la serialización y deserialización de las estructuras de datos de Rust de forma eficaz y genérica



## Proyecto Final
### Contenedores de terceros
#### [`structopt`](https://crates.io/crates/structopt)
Para analizar y controlar los argumentos de la línea de comandos.

- Comprobar si está disponible y determinar la versión más reciente: `cargo search structopt`
#### `chrono`
Contenedor que puede usar si necesita controlar los datos de fecha y hora en Rust. Este contenedor proporciona una API sencilla para representar un momento dado.

#### `serde_json` y `serde`
- `serde`. El contenedor base que permitirá a nuestros tipos derivar los rasgos Serialize y Deserialize.
- `serde_json`. El contenedor que implementará esos rasgos en el formato de especificación de archivo elegido, JSON.
#### `home`
Dado que los directorios de inicio varían según el sistema operativo del usuario, se confiará en un contenedor de terceros llamado home para determinar el directorio.

#### `anyhow`
Contenedor para mostrar errores útiles y atractivos a los usuario. Proporciona su propio tipo de error. Este tipo tiene propiedades de impresión atractivas y se puede convertir fácilmente a partir de otros errores.

### Acciones del programa
- Agregar una tarea.
- Quitar una tarea.
- Imprimir la lista de tareas.


