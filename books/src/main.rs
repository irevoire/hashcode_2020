#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let (nb_book, nb_library, nb_days, mut scores, mut library) = books::parse().unwrap();

    dbg!(nb_book);
    dbg!(library);

    /*
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
    */
}
