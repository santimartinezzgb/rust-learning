fn main() {
    let x = String::from("hello world");

    let word = first_word(&x); 

    println!("La primera palabra es: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convertimos el string a bytes

    for (i, &b) in bytes.iter().enumerate() { // iteramos sobre los bytes con su índice

        if b == b' ' { // buscamos el byte que representa un espacio
            return &s[0..i]; // devolvemos el slice desde el inicio hasta el índice del espacio
        }
    }

    &s[..] // si no hay espacios, devolvemos el string completo como slice
}


// Explicación de first_word. Esta función toma una referencia a un string (&str) y devuelve una referencia a la primera palabra en ese string.
// 1. Convierte el string en un array de bytes para poder iterar sobre cada byte.
// 2. Usa un bucle for con enumerate() para obtener tanto el índice como el byte en cada iteración.
// 3. Compara cada byte con el byte que representa un espacio (b' ').
// 4. Si encuentra un espacio, devuelve un slice del string desde el inicio hasta el índice del espacio.
// 5. Si no encuentra ningún espacio, devuelve un slice que representa todo el string.


// &str. Un &str es una referencia a una cadena de texto en Rust.
// Posee y administra su propia memoria
// No posee la cadena de texto a la que hace referencia, sino que apunta a una parte de una cadena de texto que ya existe en algún lugar de la memoria.
// Comúnmente utilizados para pasar cadenas de texto como argumentos a funciones sin transferir la propiedad de la cadena completa.
// Por ejemplo, en la función first_word, se pasa una referencia &str para que la función pueda leer la cadena sin tomar posesión de ella.
// Esto permite que la cadena original siga siendo válida después de que la función haya terminado de ejecutarse.