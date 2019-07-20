use std::collections::HashMap;

pub mod word;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hash {
        ( $( $t:expr),* ) => {
            {
                let mut temp_hash = HashMap::new();
                $(
                    temp_hash.insert($t.0, $t.1);
                )*
                temp_hash
            }
        };
    }

    #[test]
    fn test_preprocess() {
        let expect_id_to_word = hash![
            (0, "you"),
            (1, "say"),
            (2, "goodbye"),
            (3, "and"),
            (4, "i"),
            (5, "hello"),
            (6, ".")
        ];

        let expect_word_to_id = hash![
            ("you", 0),
            ("say", 1),
            ("goodbye", 2),
            ("and", 3),
            ("i", 4),
            ("hello", 5),
            (".", 6)
        ];

        let expect_corpus = vec![0, 1, 2, 3, 4, 1, 5, 6];

        let st = String::from("you say goodbye and i say hello .");
        let (word_to_id, id_to_word, corpus) = word::preprocess(&st);

        assert_eq!(expect_corpus, corpus);
        assert_eq!(expect_word_to_id, word_to_id);
        assert_eq!(expect_id_to_word, id_to_word);
    }
}
