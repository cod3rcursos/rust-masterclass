pub fn exemplo_a() {
    let nome = String::from("João");
    exibir_nome(nome);

    // println!("{}", nome);

    let idade = 30;
    exibir_idade(idade);

    println!("{}", idade);
}

// A referência é movida para o argumento
// que toma posse do valor
fn exibir_nome(nome: String) {
    println!("{}", nome);
} // nome é descartado (drop é chamado)

// Recebe uma copia do valor (não toma posse)
fn exibir_idade(idade: i32) {
    println!("{}", idade);
}

pub fn exemplo_b() {
    let nome = novo_nome();
    println!("{}", nome);

    let (nome, t) = calcular_tamanho(nome);
    println!("{} tem tamanho {}", nome, t);
}

#[allow(clippy::let_and_return)]
fn novo_nome() -> String {
    let nome = String::from("Maria");
    nome // posse é transferida para a função chamadora
}

// Recebe a posse do valor (ownership)
fn calcular_tamanho(nome: String) -> (String, usize) {
    let tamanho = nome.len();
    (nome, tamanho) // Devolve a posse do valor para a função chamadora
}
