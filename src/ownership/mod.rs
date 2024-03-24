mod escopo;
mod mecanismo;
mod referencia;
mod slice;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Básico",
            "Tempo de Vida",
            "Mover",
            "Clone",
            "Cópia",
            "Movendo a Posse #1",
            "Movendo a Posse #2",
            "Referência Imutável",
            "Referência Mutável #1",
            "Referência Mutável #2",
            "Referência Mutável #3",
            "Referência Pendente",
            "Slice",
        ];
        let selecionado = exibir_menu("Ownership", &itens, false);

        limpar_tela();

        match selecionado {
            1 => escopo::exemplo_basico(),
            2 => escopo::exemplo_tempo_vida(),
            3 => escopo::exemplo_mover(),
            4 => escopo::exemplo_clone(),
            5 => escopo::exemplo_copia(),
            6 => mecanismo::exemplo_a(),
            7 => mecanismo::exemplo_b(),
            8 => referencia::exemplo_ref_imutavel(),
            9 => referencia::exemplo_ref_mutavel_a(),
            10 => referencia::exemplo_ref_mutavel_b(),
            11 => referencia::exemplo_ref_mutavel_c(),
            12 => referencia::exemplo_ref_pendente(),
            13 => slice::exemplo(),
            _ => break,
        }

        esperar_enter();
    }
}
