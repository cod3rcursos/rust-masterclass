use rpassword::read_password;
use std::io::Write;

pub fn exibir_menu(itens: Vec<&str>) -> u32 {
    loop {
        limpar_tela();
        exibir_itens(&itens);

        print!("Escolha uma opção: ");
        std::io::stdout().flush().unwrap();

        let mut opcao = String::new();
        std::io::stdin().read_line(&mut opcao).unwrap();

        let opcao: u32 = opcao.trim().parse().unwrap();

        if opcao > 0 && opcao <= itens.len() as u32 {
            return opcao;
        } else if opcao == itens.len() as u32 + 1 {
            return 0;
        } else {
            println!("Opção inválida!");
        }
    }
}

fn exibir_itens(itens: &Vec<&str>) {
    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn esperar_enter() {
    print!("Pressione ENTER para continuar...");
    std::io::stdout().flush().unwrap();

    read_password().unwrap();
}

pub fn limpar_tela() {
    print!("{esc}c", esc = 27 as char);
}
