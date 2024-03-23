pub fn exemplo() {
    funcao_basica();
    funcao_com_parametros(10, 20);

    let r = funcao_com_retorno();
    println!("Retorno #1 => {}", r);

    let r = funcao_com_parametros_e_retorno(10, 20);
    println!("Retorno #2 => {}", r);
}

fn funcao_basica() {
    println!("Função básica");
}

fn funcao_com_parametros(a: i32, b: i32) {
    println!("a => {}", a);
    println!("b => {}", b);
}

fn funcao_com_retorno() -> i32 {
    10
}

fn funcao_com_parametros_e_retorno(a: i32, b: i32) -> i32 {
    a + b
}
