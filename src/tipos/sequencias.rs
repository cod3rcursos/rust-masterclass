#[allow(clippy::vec_init_then_push)]

pub fn exemplo() {
    let tupla = (1, 2, 3);
    println!("tupla => {:?}", tupla);
    println!("tupla.0 => {}", tupla.0);

    let (a, b, c) = tupla;
    println!("elementos da tupla => {} {} {}", a, b, c);

    let mut array: [i32; 10] = [0; 10];
    println!("array => {:?}", array);

    array[0] = 1;
    array[3] = 4;
    array[9] = 10;
    println!("array => {:?}", array);

    println!("array[0] => {}", array[0]);

    // dynamically sized type (DST)
    let mut slice: &[i32] = &array[1..4];
    println!("slice => {:?}", slice);

    slice = &array[2..5];
    slice.iter().for_each(|x| print!("{} ", x));
    println!();

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("vec => {:?}", vec);

    let mut vec = vec![7, 8, 9];
    vec.push(10);
    println!("vec[0] => {}", vec[0]);
    println!("vec.pop() => {}", vec.pop().unwrap());
}
