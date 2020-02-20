use std::io::prelude::*;
use std::io::{stdin, BufReader};

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone)]
pub struct Library {
    pub books: Vec<usize>,
    pub nb_book: usize,
    pub signup_duration: usize,
    pub book_throughput: usize,
    pub id: usize,
}

impl Library {
    pub fn from(
        books: Vec<usize>,
        nb_book: usize,
        signup_duration: usize,
        book_throughput: usize,
        id: usize,
    ) -> Self {
        Self {
            books,
            nb_book,
            signup_duration,
            book_throughput,
            id,
        }
    }

    pub fn parse(
        reader: &mut std::io::Lines<std::io::BufReader<std::io::Stdin>>,
        id: usize,
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

        Ok(Self::from(
            books,
            nb_book,
            signup_duration,
            book_throughput,
            id,
        ))
    }

    pub fn score(&self) -> usize {
        self.books.iter().sum()
    }

    pub fn scan_book(&self, days: usize, used: &[bool], scores: &[usize]) -> (usize, Vec<usize>) {
        if self.signup_duration > days {
            return (0, Vec::new());
        }
        let mut rem = days - self.signup_duration;
        let mut score = 0;
        let mut book = Vec::new();
        let mut book_iter = self.books.iter();

        while rem > 0 {
            let mut done = 0;
            for &b in &mut book_iter {
                if used[b] {
                    continue;
                }
                book.push(b);
                score += scores[b];
                done += 1;
                if done == self.book_throughput {
                    break;
                }
            }
            rem -= 1;
        }

        (score, book)
    }
}

impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.id, self.nb_book)?;
        for book in &self.books {
            write!(f, "{} ", book)?;
        }
        writeln!(f)
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
        .map(|id| Library::parse(&mut reader, id))
        .collect::<Result<_, _>>()?;

    Ok((nb_book, nb_library, nb_days, scores, lib))
}
