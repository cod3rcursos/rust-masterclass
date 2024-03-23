use rpassword::prompt_password;
use std::io::Write;

pub fn exibir_menu(titulo: &str, itens: &[&str], sair: bool) -> u32 {
    limpar_tela();

    let completo = String::from("Masterclass Rust :: ") + titulo;
    println!("{}", completo);
    println!("{}", String::from("=").repeat(completo.len()));

    exibir_itens(itens);

    println!("{}", if sair { "* - Sair" } else { "* - Voltar" });
    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut linha = String::new();
    std::io::stdin().read_line(&mut linha).unwrap();

    let opcao: Result<u32, _> = linha.trim().parse();

    match opcao {
        Ok(opcao) => opcao,
        _ => 0,
    }
}

fn exibir_itens(itens: &[&str]) {
    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn esperar_enter() {
    prompt_password("Pressione ENTER para continuar...").unwrap();
}

pub fn limpar_tela() {
    print!("{esc}c", esc = 27 as char);
}
