use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use tracing::info;

use crate::Vocab;

pub fn load_vocab(path: &str) -> anyhow::Result<Vocab> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|l| l.ok());
    let vocab = Vocab::from_words(lines);

    info!("{} words loaded!", vocab.len());
    Ok(vocab)
}
