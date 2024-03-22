pub fn executar() {
    variavel_imutavel();
    variavel_mutavel();

    // Constante
    const Z: i32 = 20;
    println!("O valor de Z é: {}", Z);

    // Shadowing
    let a = 25;
    println!("O valor de a é: {}", a);

    let a = 30;
    println!("O valor de a é: {}", a);

    let a = "Texto";
    println!("O valor de a é: {}", a);

    let a = 'A';
    println!("O valor de a é: {}", a);

    let a = 3.14;
    println!("O valor de a é: {}", a);

    let a = true;
    println!("O valor de a é: {}", a);

    let a = [1, 2, 3, 4, 5];
    println!("O valor de a é: {:?}", a);

    let a = (1, 2, 3, 4, 5);
    println!("O valor de a é: {:?}", a);


}

fn variavel_imutavel() {
    let x = 5;
    // x = 10;
    println!("x => {}", x);
}

fn variavel_mutavel() {
    let mut x = 10;
    let y = x;
    println!("x, y => {} {}", x, y);
    
    x = 15;
    println!("x, y => {} {}", x, y);
}