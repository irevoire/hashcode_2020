#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let (nb_book, nb_library, nb_days, mut scores, mut library) = books::parse().unwrap();
    // let mut order = Vec::new();

    for lib in library.iter_mut() {
        lib.books.sort_unstable_by_key(|el| scores[*el]);
    }

    library.sort_unstable_by_key(|lib| lib.score());

    println!("{}", nb_library);
    for lib in library {
        print!("{}", lib);
    }
}
