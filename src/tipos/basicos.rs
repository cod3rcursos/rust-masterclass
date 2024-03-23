pub fn exemplo() {
    let ativo: bool = true; // false
    println!("Booleano {}", ativo);

    let caractere: char = 'a';
    println!("Caractere {}", caractere);

    let nome: &str = "Maria Silva";
    println!("String {}", nome);

    let mut nome: String = String::from("João");
    nome.push_str(" da Silva");
    println!("String {}", nome);

    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    let quantidade: i32 = 10;
    println!("Inteiro {}", quantidade);

    // f32, f64
    let preco: f64 = 10.99;
    println!("Ponto flutuante {}", preco);
}
