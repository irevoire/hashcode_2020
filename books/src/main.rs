#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let (nb_book, nb_library, mut nb_days, mut scores, mut library) = books::parse().unwrap();
    let mut used_book = vec![false; nb_book];
    let mut used_lib = vec![false; nb_library];
    let mut lib_order = Vec::new();

    for lib in library.iter_mut() {
        lib.books.sort_unstable_by_key(|el| scores[*el]);
    }

    while nb_days != 0 && !library.is_empty() {
        let mut max = 0;
        let mut books = Vec::new();
        let mut id = 0;

        for lib in &library {
            if used_lib[lib.id] {
                continue;
            }
            let (s, b) = lib.scan_book(nb_days, &used_book, &scores);
            if s > max {
                max = s;
                books = b;
                id = lib.id;
            }
        }

        if max == 0 {
            break;
        }

        books.iter().for_each(|b| used_book[*b] = true);
        lib_order.push((id, books));
        nb_days -= library[id].signup_duration;
        used_lib[id] = true;
    }

    println!("{}", lib_order.len());

    for (id, books) in lib_order {
        println!("{} {}", id, books.len());
        for b in books {
            print!("{} ", b);
        }
        println!();
    }
}
