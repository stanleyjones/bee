use std::collections::BTreeMap;

pub fn get_words<'a>(words: Vec<&'a str>, letters: &String) -> BTreeMap<u32, Vec<&'a str>> {
    let scored = words
        .into_iter()
        .filter(|&word| is_valid(word, &letters))
        .map(|word| (score_word(word, &letters), word))
        .collect();

    group_words(scored)
}

fn is_valid(word: &str, letters: &String) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let center_letter: char = letters.chars().nth(0).unwrap();
    return chars.len() > 3
        && chars.contains(&center_letter)
        && chars.iter().all(|&c| letters.contains(c));
}

fn score_word(word: &str, letters: &String) -> u32 {
    let mut score = word.len() as u32;
    if letters.chars().all(|c| word.contains(c)) {
        score += 5;
    }
    score
}

fn group_words(words: Vec<(u32, &str)>) -> BTreeMap<u32, Vec<&str>> {
    let mut words_by_score: BTreeMap<u32, Vec<&str>> = BTreeMap::new();

    for (score, word) in words {
        words_by_score.entry(score).or_insert(Vec::new()).push(word);
    }

    return words_by_score;
}

#[test]
fn should_be_at_least_4_letters_long() {
    let letters = "fod".to_string();
    assert_eq!(is_valid("foo", &letters), false);
    assert_eq!(is_valid("food", &letters), true);
}

#[test]
fn should_contain_the_center_letter() {
    let letters = "fod".to_string();
    assert_eq!(is_valid("dood", &letters), false);
    assert_eq!(is_valid("food", &letters), true);
}

#[test]
fn should_only_contain_the_given_letters() {
    let letters = "fod".to_string();
    assert_eq!(is_valid("foob", &letters), false);
    assert_eq!(is_valid("food", &letters), true);
}

#[test]
fn should_score_1_point_per_letter() {
    let letters = "fobar".to_string();
    assert_eq!(score_word("foob", &letters), 4);
    assert_eq!(score_word("fooboo", &letters), 6);
}

#[test]
fn should_score_5_extra_points_for_pangrams() {
    let letters = "fobar".to_string();
    assert_eq!(score_word("fooboo", &letters), 6);
    assert_eq!(score_word("foobar", &letters), 11);
}

#[test]
fn should_group_words_by_score() {
    let scored_words = vec![(4, "food"), (6, "fooboo"), (6, "booboo"), (11, "foobar")];
    assert_eq!(group_words(scored_words).get(&6).unwrap().len(), 2);
}
