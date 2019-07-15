use std::collections::HashMap;
extern crate ndarray;
use ndarray::prelude::*;

pub fn words(s: String) {
    let mut word_to_id = HashMap::new();
    let mut id_to_word = HashMap::new();
    let mut corpus = Vec::new();

    let mut i = 0;
    for item in s.split_whitespace() {
        if word_to_id.get(item) != None {
            continue;
        }
        id_to_word.insert(i, item);
        word_to_id.insert(item, i);
        i += 1;
    }

    for item in s.split_whitespace() {
        corpus.push(word_to_id[item]);
    }

    println!("{:#?}", &id_to_word);
    println!("{:#?}", &word_to_id);
    println!("{:#?}", &corpus);

    let corpus_len = corpus.len();
    let vocab_len = word_to_id.len();
    let window_size = 1;

    let mut a = Array2::<i64>::zeros((vocab_len, vocab_len));

    for word_id in &corpus {
        if *word_id > 1 {
            let left_id = word_id - 1;
            let left_word_id = corpus[left_id];
            a[[*word_id, left_word_id]] += 1;
        }

        if *word_id < (corpus_len - 1) {
            let right_id = word_id + 1;
            let right_word_id = corpus[right_id];
            a[[*word_id, right_word_id]] += 1;
        }
    }
    println!("{:#?}", a);
}
