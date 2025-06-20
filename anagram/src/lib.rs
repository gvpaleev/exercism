use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = lowercase_word.chars().collect();
    sorted_word.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let lowercase_candidate = candidate.to_lowercase();
            // Пропускаем точные совпадения (без учета регистра)
            lowercase_candidate != lowercase_word
        })
        .filter(|&&candidate| {
            let lowercase_candidate = candidate.to_lowercase();
            let mut sorted_candidate: Vec<char> = lowercase_candidate.chars().collect();
            sorted_candidate.sort_unstable();
            sorted_candidate == sorted_word
        })
        .copied()
        .collect()
}
