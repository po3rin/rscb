#[macro_use(s)]
extern crate ndarray;
extern crate rscb;

use rscb::word;

fn main() {
    let st = String::from("you say goodbye and i say hello .");
    println!("=====target text=====\n'{}'\n", st);

    let (word_to_id, id_to_word, corpus) = word::preprocess(&st);

    println!("=====id_to_word=====\n{:#?}\n", &id_to_word);
    println!("=====word_to_id=====\n{:#?}\n", &word_to_id);
    println!("=====corpus=====\n{:#?}\n", &corpus);

    let a = word::create_to_matrix(corpus, &word_to_id);

    println!("======Co-occurrence matrix=====\n{:#?}\n", a);

    let c0 = a.slice(s![word_to_id["you"], ..]);
    let c1 = a.slice(s![word_to_id["i"], ..]);

    let vocab_len = word_to_id.len();
    let sim = word::cos_simirality(c0, c1, vocab_len);

    println!("====== Result =====");
    println!("'you' & 'I' : {}", sim)
}
