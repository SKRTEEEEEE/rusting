# Rust
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
