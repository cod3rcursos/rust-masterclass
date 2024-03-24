// 1. Em um dado momento, você pode ter uma referência mutável
// ou qualquer número de referências imutáveis.
// 2. As referências devem sempre ser válidas.

pub fn exemplo_ref_imutavel() {
    let texto = String::from(
        "Rust é uma linguagem de programação de sistemas 
        que roda incrivelmente rápido, previne falhas de 
        segmentação e garante segurança concorrente.",
    );

    let qtde_palavras = calcular_qtde_palavras(&texto);
    println!("O texto {} tem {} palavras", texto, qtde_palavras);
}

#[allow(clippy::ptr_arg)]
// Recebe uma referência para o texto (não toma posse)
// Borrows (empresta) o texto
fn calcular_qtde_palavras(texto: &String) -> usize {
    texto.split_whitespace().count()
    // texto é descartado (drop não é chamado porque não tem a posse - ownership)
}

pub fn exemplo_ref_mutavel_a() {
    let nome = String::from("João");
    sobrenome(&nome);
    println!("Nome: {}", nome);
}

#[allow(clippy::ptr_arg)]
// Não pode ser emprestado como mutável
fn sobrenome(texto: &String) {
    // texto.push_str(" da Silva");
    println!("{}", texto.len());
}

pub fn exemplo_ref_mutavel_b() {
    let mut nome = String::from("João");
    sobrenome_mutavel(&mut nome);
    println!("Nome: {}", nome);

    let n1 = &mut nome;
    println!("{}", n1);

    let n2 = &mut nome;
    println!("{}", n2);
}

// Pode ser emprestado como mutável
fn sobrenome_mutavel(texto: &mut String) {
    texto.push_str(" da Silva");
}

pub fn exemplo_ref_mutavel_c() {
    let mut nome = String::from("Gabriel");

    // imutável - sem problema
    let n1 = &nome;
    let n2 = &nome;

    println!("{} {}", n1, n2);

    // mutável - problema se trocar a ordem
    let n3 = &mut nome;

    println!("{}", n3);
}

pub fn exemplo_ref_pendente() {
    // let aponta_para_nada = gerar_pendencia();
    // println!("Referência Pendente {}", aponta_para_nada);

    let sem_pendencia = sem_pendencia();
    println!("Sem Pendência {}", sem_pendencia);
}

// fn gerar_pendencia() -> &String {
//     let texto = String::from("Referência Pendente");
//     &texto // retorna uma referência que vai ser descartada
// }

#[allow(clippy::let_and_return)]
fn sem_pendencia() -> String {
    let texto = String::from("Sem pendência");
    texto // retorna uma referência que será movida para a função chamadora
}
