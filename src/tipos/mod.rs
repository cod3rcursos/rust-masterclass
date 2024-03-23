mod basicos;
mod custom;
mod sequencias;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Básicos",
            "Sequências",
            "Custom - Structs",
            "Custom - Enums",
        ];
        let selecionado = exibir_menu("Tipos", &itens, false);

        limpar_tela();

        match selecionado {
            1 => basicos::exemplo(),
            2 => sequencias::exemplo(),
            3 => custom::exemplo_struct(),
            4 => custom::exemplo_enum(),
            _ => break,
        }

        esperar_enter();
    }
}
