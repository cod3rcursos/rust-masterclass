pub fn exemplo_range() {
    for a in 1..5 {
        println!("a => {}", a);
    }

    for b in 1..=5 {
        println!("b => {}", b);
    }

    for c in (1..5).rev() {
        println!("c => {}", c);
    }

    for d in (1..=10).step_by(2) {
        println!("d => {}", d);
    }
}
