mod funcs;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = ["Básicos"];
        let selecionado = exibir_menu("Funções", &itens, false);

        limpar_tela();

        match selecionado {
            1 => funcs::exemplo(),
            _ => break,
        }

        esperar_enter();
    }
}
