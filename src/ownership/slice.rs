// Os Slice permitem fazer referência a uma sequência contígua
// de elementos em uma coleção, em vez de à coleção inteira.
// O Slice é uma espécie de referência, portanto não possui ownership.

pub fn exemplo() {
    let texto = String::from("Rust é uma linguagem de programação moderna");

    let palavra = primeira_palavra_com('l', &texto);
    println!("A palavra é '{}'", palavra);
}

// Primeira palavra iniciada pela letra informada
fn primeira_palavra_com(letra: char, slice: &str) -> &str {
    let bytes = slice.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == letra as u8 {
            return slice[i..].split_whitespace().next().unwrap();
        }
    }
    slice
}
