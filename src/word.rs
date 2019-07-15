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
        corpus.push(i);
        i = i + 1
    }

    println!("{:#?}", &id_to_word);
    println!("{:#?}", &word_to_id);
    println!("{:#?}", &corpus);

    let corpus_len = corpus.len();
    let vocab_len = corpus.len();
    let window_size = 1;

    let mut a = Array2::<i64>::zeros((vocab_len, vocab_len));
    println!("{:#?}", a);

    for word_id in &corpus {
        let left_id = word_id - 1;
        let right_id = word_id + 1;
        if left_id > 0 {
            let left_word_id = corpus[left_id as usize];
            a[[2, 3]] += 1;
        }

        if right_id < vocab_len as i32 {
            let right_word_id = corpus[right_id as usize];
            a[[2, 2]] += 1;
        }
    }
    println!("{:#?}", a);
}
