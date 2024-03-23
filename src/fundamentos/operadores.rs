pub fn aritmeticos() {
    let x = 10;
    let y = 5;

    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x % y = {}", x % y);
}

pub fn relacionais() {
    let x = 10;
    let y = 5;

    println!("x == y => {}", x == y);
    println!("x != y => {}", x != y);
    println!("x > y => {}", x > y);
    println!("x < y => {}", x < y);
    println!("x >= y => {}", x >= y);
    println!("x <= y => {}", x <= y);
}

pub fn logicos() {
    let x = true;
    let y = false;

    println!("x && y => {}", x && y);
    println!("x || y => {}", x || y);
    println!("!x => {}", !x);
}
