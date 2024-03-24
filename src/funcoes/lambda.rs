// pub fn exemplo() {}

pub fn exemplo_map() {
    let base_2: Vec<i32> = (1..=10).map(|x| i32::pow(2, x)).collect();
    println!("{:?}", base_2);
}

pub fn exemplo_filter() {
    let e_primo = |n: i32| -> bool {
        if n < 2 {
            return false;
        }

        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }

        true
    };

    let primos: Vec<i32> = (1..=10000)
        .filter(|&x| e_primo(x))
        .filter(|&x| x.to_string().ends_with('7'))
        .collect();
    println!("{:?}", primos);
}
