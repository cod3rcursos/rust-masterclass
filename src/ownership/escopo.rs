// 1. Todo valor em Rust tem um dono.
// 2. Só pode ter um dono por vez.
// 3. Quando dono sair do escopo, valor será descartado.

pub fn exemplo_basico() {
    {
        let s = String::from("Olá");
        println!("{}", s);
    }
    // println!("{}", s);
}

pub fn exemplo_tempo_vida() {
    let x;
    {
        let y = String::from("Escopo interno");
        x = &y;
        println!("{} {}", x, y);
        println!("{}", x);
    }
}

pub fn exemplo_mover() {
    // Valor é alocado na stack
    let num1 = 10;
    let num2 = num1;
    println!("{} {}", num1, num2);

    // Valor é alocado na heap
    let s1 = String::from("Olá");

    // s1 foi movido para s2
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}

pub fn exemplo_clone() {
    let s1 = String::from("Olá");

    // Clone precisa ser explicitamente chamado
    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}

pub fn exemplo_copia() {
    // Valores fixos são armazenados na stack e são copiados
    // Precisa implementar a trait Copy
    // i32, f64, bool, char, tuplas com tipos Copy
    let numeros_a = [1, 2, 3, 4, 5];
    let numeros_b = numeros_a;

    println!("{:?} {:?}", numeros_a, numeros_b);
}
