fn main() {
    let (max_slice, _, mut pizza) = pizza::parse().unwrap();
    // pizza.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    pizza.reverse();

    // ALGO
    let mut cpt = 0;
    let mut used = Vec::new();

    for p in pizza {
        if cpt + p.nb < max_slice {
            cpt += p.nb;
            used.push(p.clone());
        }
    }

    used.reverse();

    println!("{}", used.len());
    for p in used {
        print!("{} ", p.id);
    }
    println!();
}
