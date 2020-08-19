pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();
    let mut proverb: Vec<String> = Vec::with_capacity(len - 1);
    let mut iter = list.iter();
    iter.next();

    for word in list {
        if let Some(next_word) = iter.next() {
            proverb.push(make_sentence(word, next_word));
        }
    }

    proverb.push(make_last_sentence(list[0]));
    proverb.join("\n")
}

fn make_sentence(word1: &str, word2: &str) -> String {
    format!("For want of a {} the {} was lost.", word1, word2)
}

fn make_last_sentence(word: &str) -> String {
    format!("And all for the want of a {}.", word)
}
