extern crate ndarray;

use ndarray::prelude::*;
use std::collections::HashMap;
use std::ops::Mul;

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

    println!("=====id_to_word=====\n{:#?}\n", &id_to_word);
    println!("=====word_to_id=====\n{:#?}\n", &word_to_id);
    println!("=====corpus=====\n{:#?}\n", &corpus);

    let corpus_len = corpus.len();
    let vocab_len = word_to_id.len();
    let mut a = Array2::<f64>::zeros((vocab_len, vocab_len));

    let mut i = 0;
    for word_id in &corpus {
        if i > 0 {
            let left_id = i - 1;
            let left_word_id = corpus[left_id];
            a[[*word_id, left_word_id]] += 1.;
        }

        if *word_id < (corpus_len - 1) && i < corpus_len - 1 {
            let right_id = i + 1;
            let right_word_id = corpus[right_id];
            a[[*word_id, right_word_id]] += 1.;
        }
        i += 1
    }
    println!("======Co-occurrence matrix=====\n{:#?}\n", a);

    let c0 = a.slice(s![word_to_id["you"], ..]);
    let c1 = a.slice(s![word_to_id["i"], ..]);

    let mut sum0: f64 = 0.;
    let mut sum1: f64 = 0.;

    for item in c0 {
        sum0 += item.powi(2);
    }
    for item in c1 {
        sum1 += item.powi(2);
    }

    let m0 = sum0.sqrt();
    let m1 = sum1.sqrt();

    let mut cm0 = Array::<f64, _>::zeros(vocab_len);
    for mut row in cm0.genrows_mut() {
        row.fill(1. / m0);
    }
    let mut cm1 = Array::<f64, _>::zeros(vocab_len);
    for mut row in cm1.genrows_mut() {
        row.fill(1. / m1);
    }

    let nx = c0.mul(&cm0);
    let ny = c1.mul(&cm1);

    println!("{}", nx.dot(&ny))
}
