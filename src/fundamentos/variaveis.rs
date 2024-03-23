pub fn imutaveis() {
    let x = 5;
    // x = 10;
    println!("x => {}", x);
}

pub fn mutaveis() {
    let mut x = 10;
    let y = x; // primitivo
    println!("x, y => {} {}", x, y);

    x = 15;
    println!("x, y => {} {}", x, y);
}

pub fn constantes() {
    const Z: i32 = 20;
    println!("O valor de Z é: {}", Z);
}

pub fn shadowing() {
    let a = 25;
    println!("O valor de a é: {}", a);

    let a = "Texto";
    println!("O valor de a é: {}", a);

    let a = [1, 2, 3, 4, 5];
    println!("O valor de a é: {:?}", a);
}
