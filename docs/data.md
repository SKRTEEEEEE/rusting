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
#### rustc
Con rustc podemos directamente compilar o ejecutar nuestro código
- compilar: `rustc <nombre_ubicación_archivo>`
- ejecutar: `<nombre_ubicacion_archivo>.exe`
#### cargo
Con cargo podemos crear el proyecto de carga y que el compile y ejecute el codigo por nosotros.
- crear proyecto: `cargo new <nombre_archivo>`
- compilar y ejecutar: `cargo run`
### 2. Rust introducción
#### [primeras macros y variable let](../src/basic-structure/src/main.rs)
- macros: `todo!()` y `println!()`
#### [mut](../src/mut-key/src/main.rs)
#### shadowing
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
#### [tipos](../src/tipos-intro/src/main.rs)
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


#### [colecciones de datos](../src/tuplas/src/main.rs)

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

#### [variantes `enum` para datos compuestos]()