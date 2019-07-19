use std::collections::HashMap;

pub mod word;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess() {
        let mut expect_word_to_id = HashMap::new();
        let mut expect_id_to_word = HashMap::new();
        let mut expect_corpus = Vec::new();

        expect_id_to_word.insert(0, "you");
        expect_id_to_word.insert(1, "say");
        expect_id_to_word.insert(2, "goodbye");
        expect_id_to_word.insert(3, "and");
        expect_id_to_word.insert(4, "i");
        expect_id_to_word.insert(5, "hello");
        expect_id_to_word.insert(6, ".");

        expect_word_to_id.insert("you", 0);
        expect_word_to_id.insert("say", 1);
        expect_word_to_id.insert("goodbye", 2);
        expect_word_to_id.insert("and", 3);
        expect_word_to_id.insert("i", 4);
        expect_word_to_id.insert("hello", 5);
        expect_word_to_id.insert(".", 6);

        expect_corpus.push(0);
        expect_corpus.push(1);
        expect_corpus.push(2);
        expect_corpus.push(3);
        expect_corpus.push(4);
        expect_corpus.push(1);
        expect_corpus.push(5);
        expect_corpus.push(6);

        let st = String::from("you say goodbye and i say hello .");
        let (word_to_id, id_to_word, corpus) = word::preprocess(&st);

        assert_eq!(expect_corpus, corpus);
        assert_eq!(expect_word_to_id, word_to_id);
        assert_eq!(expect_id_to_word, id_to_word);
    }
}
