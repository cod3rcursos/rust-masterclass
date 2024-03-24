// Os Slice permitem fazer referência a uma sequência contígua
// de elementos em uma coleção, em vez de à coleção inteira.
// O Slice é uma espécie de referência, portanto não possui ownership.

pub fn exemplo() {
    let texto = String::from("Rust é uma linguagem de programação moderna");

    let palavra = com_slice(&texto, 'l');
    println!("A palavra é '{}'", palavra);

    let palavra = sem_slice(&texto, 'l');
    println!("A palavra é '{}'", palavra);
}

// Primeira palavra iniciada pela letra informada
fn com_slice(slice: &str, letra: char) -> &str {
    let bytes = slice.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == letra as u8 {
            return slice[i..].split_whitespace().next().unwrap();
        }
    }
    slice
}

// Primeira palavra iniciada pela letra informada
fn sem_slice(texto: &String, letra: char) -> String {
    let bytes = texto.as_bytes();

    let mut encontrou = false;
    let mut inicio = 0;
    let mut fim = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == letra as u8 {
            encontrou = true;
            inicio = i;
        }

        if encontrou && item == b' ' {
            fim = i;
            break;
        }
    }

    String::from(if encontrou { &texto[inicio..fim] } else { "" })
}
