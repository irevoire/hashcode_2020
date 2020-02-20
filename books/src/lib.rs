use std::io::prelude::*;
use std::io::{stdin, BufReader};

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone)]
pub struct Library {
    pub books: Vec<usize>,
    pub nb_book: usize,
    pub signup_duration: usize,
    pub book_throughput: usize,
}

impl Library {
    pub fn from(
        books: Vec<usize>,
        nb_book: usize,
        signup_duration: usize,
        book_throughput: usize,
    ) -> Self {
        Self {
            books,
            nb_book,
            signup_duration,
            book_throughput,
        }
    }

    pub fn parse(
        reader: &mut std::io::Lines<std::io::BufReader<std::io::Stdin>>,
    ) -> Result<Self, Error> {
        let tmp = reader.next().unwrap()?;

        // start parsing line per line
        let mut tmp = tmp.split(" ");

        let nb_book = tmp.next().unwrap().parse()?;
        let signup_duration = tmp.next().unwrap().parse()?;
        let book_throughput = tmp.next().unwrap().parse()?;

        let tmp = reader.next().unwrap()?;
        let books = tmp
            .split(" ")
            .map(|idx| idx.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self::from(books, nb_book, signup_duration, book_throughput))
    }
}

/// number of books, number of library, number of days, book score
pub fn parse() -> Result<(usize, usize, usize, Vec<usize>, Vec<Library>), Error> {
    // get the reader
    let f = stdin();
    let reader = BufReader::new(f);

    let mut reader = reader.lines();
    let tmp = reader.next().unwrap()?;

    // start parsing line per line
    let mut tmp = tmp.split(" ");

    let nb_book = tmp.next().unwrap().parse()?;
    let nb_library = tmp.next().unwrap().parse()?;
    let nb_days = tmp.next().unwrap().parse()?;

    let tmp = reader.next().unwrap()?;

    // parse the score line
    let tmp = tmp.split(" ");
    let scores = tmp.map(|id| id.parse()).collect::<Result<_, _>>()?;

    let lib = (0..nb_library)
        .map(|_| Library::parse(&mut reader))
        .collect::<Result<_, _>>()?;

    Ok((nb_book, nb_library, nb_days, scores, lib))
}
