use std::io;
use std::collections::HashMap;

struct BPETokenizer {
    vocab: HashMap<String, u32>,
    merge_rules: Vec<(String, String)>,
}

impl BPETokenizer {
    fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            merge_rules: Vec::new(),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    //read_line returns Result ={Ok | Err}
    //unwrap -> give the value of Ok() and panic if Err()
    let input = input.trim();
    let words: Vec<&str> = input.split_whitespace().collect();

    //Now the word sequencing;
    //like storing character sequencing of the word

    let mut tokenizer = BPETokenizer::new();

    let mut word_occurence: HashMap<String, (u32, Vec<String>)> = HashMap::new();

    for word in words {
        let mut char_seq: Vec<String> = Vec::new();
        for char in word.chars() {
            char_seq.push(char.to_string());
        }
        let entry = word_occurence.entry(word.to_string()).or_insert((0, char_seq));
        entry.0 += 1;
    }

    //Parameter to be changed
    let vocab_size = 20;

    for i in 0..vocab_size {
        let mut pair_freq: HashMap<(String, String), u32> = HashMap::new();
        for (word, (freq, char_seq)) in &word_occurence {
            for i in 1..char_seq.len() {
                let pair = (char_seq[i-1].clone(), char_seq[i].clone());
                *pair_freq.entry(pair).or_insert(0) += *freq;
            }
        }
        if pair_freq.is_empty() { break; }
        //to get the pair with max frequency
        let mut pair = pair_freq.iter().max_by_key(|(_, v)| *v).unwrap().0.clone();

        tokenizer.merge_rules.push(pair.clone());

        for (word, (freq, char_seq)) in &mut word_occurence {
            let mut new_vec: Vec<String> = Vec::new();
            let mut j = 0;
            while j < char_seq.len() {
                if j+1 < char_seq.len() && char_seq[j] == pair.0 && char_seq[j+1] == pair.1 {
                    new_vec.push(format!("{}{}", pair.0, pair.1));
                    j+=2;
                }else{
                    new_vec.push(char_seq[j].clone());
                    j+=1;
                }
            }
            *char_seq = new_vec;
        }
    }

    //building vocab
    let mut k: u32 = 0;
    for (word, (freq, char_seq)) in &word_occurence {
        for seq in char_seq {
            tokenizer.vocab.entry(seq.clone()).or_insert_with(|| {
                let id = k;
                k+=1;
                id
            }); //This function only runs when key is not found
            //so it inserts the new key and registers a new token id to it!
        }
    }

    println!("Merge rules: {:?}", tokenizer.merge_rules);
    println!("Vocab: {:?}", tokenizer.vocab);
}
