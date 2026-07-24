use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vocab {
    words: Vec<String>,

    first_idx: HashMap<char, Vec<usize>>,
}

impl Vocab {
    pub fn new() -> Self {
        Vocab {
            words: Vec::new(),
            first_idx: HashMap::new(),
        }
    }

    pub fn from_words<I>(words: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut vocab = Vocab::new();
        for word in words {
            vocab.add(word);
        }
        vocab
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn add(&mut self, word: impl Into<String>) -> bool {
        let word = word.into();
        let word = word.trim().to_string();

        if word.is_empty() {
            return false;
        }

        let idx = self.words.len();
        let first = word.chars().next().unwrap();

        self.words.push(word);
        self.first_idx.entry(first).or_default().push(idx);

        true
    }

    pub fn filter(
        &self,
        first: Option<char>,
        min_len: Option<usize>,
        max_len: Option<usize>,
    ) -> Vec<&str> {
        let candidates: Vec<&str> = if let Some(f) = first {
            self.first_idx
                .get(&f)
                .map(|indices| indices.iter().map(|&i| self.words[i].as_str()).collect())
                .unwrap_or_default()
        } else {
            self.words.iter().map(|s| s.as_str()).collect()
        };

        candidates
            .into_iter()
            .filter(|w| min_len.map_or(true, |min| w.len() >= min))
            .filter(|w| max_len.map_or(true, |max| w.len() <= max))
            .collect()
    }
}
