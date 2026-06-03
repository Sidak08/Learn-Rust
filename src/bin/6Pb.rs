use std::collections::HashMap;
use std::collections::HashSet;

pub fn freq_vec_hash_map(word_list: &[&str]) -> Vec<HashMap<char, i32>> {
    let mut word_Freq_Vec: Vec<HashMap<char, i32>> = Vec::new();
    for i in 0..word_list.len() {
        let curr_Word: Vec<char> = word_list[i].chars().collect();
        let mut curr_Hash_map: HashMap<char, i32> = HashMap::new();
        for j in 0..curr_Word.len() {
            //println!("{}", curr_Word[j]);
            let char_count = curr_Hash_map
                .entry(curr_Word[j].to_lowercase().next().unwrap())
                .or_insert(0);
            *char_count += 1;
        }
        word_Freq_Vec.push(curr_Hash_map);
    }
    return word_Freq_Vec;
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let vec_to_match: Vec<HashMap<char, i32>> = freq_vec_hash_map(&[word]);
    let possible_options: Vec<HashMap<char, i32>> = freq_vec_hash_map(possible_anagrams);
    let mut answer: HashSet<&'a str> = HashSet::new();

    for i in 0..possible_options.len() {
        if vec_to_match[0] == possible_options[i]
            && possible_anagrams[i].to_lowercase() != word.to_lowercase()
        {
            answer.insert(possible_anagrams[i]);
        }
    }
    println!("{:?}", answer);

    return answer;
}

pub fn main() {
    let test_set = ["stone", "tones", "banana", "tons", "notes", "Seton"];
    let test_word: &str = "stone";

    let result = anagrams_for(&test_word, &test_set);
}
