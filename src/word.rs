extern crate ndarray;

use ndarray::prelude::*;
use std::collections::HashMap;
use std::ops::Mul;

pub fn preprocess<'a>(
    s: &'a str,
) -> (HashMap<&'a str, usize>, HashMap<usize, &'a str>, Vec<usize>) {
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

    (word_to_id, id_to_word, corpus)
}

pub fn create_to_matrix(corpus: Vec<usize>, word_to_id: &HashMap<&str, usize>) -> Array2<f64> {
    let corpus_len = corpus.len();
    let vocab_len = word_to_id.len();
    let mut a = Array2::<f64>::zeros((vocab_len, vocab_len));

    for (i, word_id) in corpus.iter().enumerate() {
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
    }

    a
}

pub fn cos_simirality(
    c0: ArrayBase<ndarray::ViewRepr<&f64>, Dim<[usize; 1]>>,
    c1: ArrayBase<ndarray::ViewRepr<&f64>, Dim<[usize; 1]>>,
    vocab_len: usize,
) -> f64 {
    let mut sum0: f64 = 0.;
    let mut sum1: f64 = 0.;

    for item in c0 {
        sum0 += item.powi(2);
    }
    for item in c1 {
        sum1 += item.powi(2);
    }

    let mut cm0 = Array::<f64, _>::zeros(vocab_len);
    let mut cm1 = Array::<f64, _>::zeros(vocab_len);

    for mut row in cm0.genrows_mut() {
        row.fill(1. / sum0.sqrt())
    }
    for mut row in cm1.genrows_mut() {
        row.fill(1. / sum1.sqrt())
    }

    let nx = c0.mul(&cm0);
    let ny = c1.mul(&cm1);

    nx.dot(&ny)
}
