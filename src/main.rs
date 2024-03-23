mod fundamentos;
mod utils;

use std::process::exit;
use utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens = vec!["Fundamentos", "Sair"];
        let opcao = exibir_menu(itens);

        limpar_tela();

        match opcao {
            1 => fundamentos::executar(),
            2 => exit(0),
            _ => println!("Opção inválida!"),
        }

        esperar_enter();
    }
}
