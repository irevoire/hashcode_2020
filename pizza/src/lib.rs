use std::io::prelude::*;
use std::io::{stdin, BufReader};

type Error = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct Pizza {
    pub id: usize, // initial position
    pub nb: u64,   // number of slice
}

impl Pizza {
    pub fn from(id: usize, nb: u64) -> Self {
        Pizza { id, nb }
    }
}

/// max slice, number of pizza type, [pizza]
pub fn parse() -> Result<(u64, u64, Vec<Pizza>), Error> {
    // get the reader
    let f = stdin();
    let reader = BufReader::new(f);

    let mut reader = reader.lines();
    let tmp = reader.next().unwrap()?;

    // start parsing line per line
    let mut tmp = tmp.split(" ");

    let max_slice = tmp.next().unwrap().parse()?;
    let nb_type = tmp.next().unwrap().parse()?;

    let pizzas = reader
        .next()
        .unwrap()?
        .split(" ")
        .enumerate()
        .map(|(id, nb)| Pizza::from(id, nb.parse().unwrap()))
        .collect();

    Ok((max_slice, nb_type, pizzas))
}
