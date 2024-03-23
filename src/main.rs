mod controle;
mod fundamentos;
mod utils;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens = ["Fundamentos", "Controle"];
        let selecionado = exibir_menu("Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::executar(),
            2 => controle::executar(),
            _ => exit(0),
        }
    }
}
