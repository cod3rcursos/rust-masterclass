mod controle;
mod funcoes;
mod fundamentos;
mod tipos;
mod utils;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens = ["Fundamentos", "Tipos", "Controle", "Funções"];
        let selecionado = exibir_menu("Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::executar(),
            2 => tipos::executar(),
            3 => controle::executar(),
            4 => funcoes::executar(),
            _ => exit(0),
        }
    }
}
