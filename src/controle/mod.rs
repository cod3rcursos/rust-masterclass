mod condicionais;
mod repeticoes;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = ["Condicionais", "Repetições - Range"];
        let selecionado = exibir_menu("Controle", &itens, false);

        limpar_tela();

        match selecionado {
            1 => condicionais::exemplo(),
            2 => repeticoes::exemplo_range(),
            _ => break,
        }

        esperar_enter();
    }
}
