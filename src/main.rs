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

    fn train(&mut self, input: &str) {
        let words: Vec<&str> = input.split_whitespace().collect();

        let mut word_occurence: HashMap<String, (u32, Vec<String>)> = HashMap::new();

        for word in words {
            let mut char_seq: Vec<String> = Vec::new();
            for char in word.chars() {
                char_seq.push(char.to_string());
            }
            //`or_insert` refers to "Insert this value if key doesn't exist then return a mutable reference to the value".
            //If the value exist then just increase the first value of the tuple which is the frequency!
            word_occurence.entry(word.to_string()).or_insert((0, char_seq)).0 += 1;
        }

        //Parameter to be changed
        let vocab_size = 20;

        for _ in 0..vocab_size {
            let mut pair_freq: HashMap<(String, String), u32> = HashMap::new();
            for (_, (freq, char_seq)) in &word_occurence {
                for i in 1..char_seq.len() {
                    let pair = (char_seq[i-1].clone(), char_seq[i].clone());
                    //In this case the value returns an &mut u32 mutable reference and as its an reference we need to dereference it.
                    *pair_freq.entry(pair).or_insert(0) += *freq;
                }
            }
            if pair_freq.is_empty() { break; }
            //to get the pair with max frequency
            let pair = pair_freq.iter().max_by_key(|(_, v)| *v).unwrap().0.clone();

            self.merge_rules.push(pair.clone());

            for (_, (_, char_seq)) in &mut word_occurence {
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
        let mut k: u32 = 1;
        for (_, (_, char_seq)) in &word_occurence {
            for seq in char_seq {
                self.vocab.entry(seq.clone()).or_insert_with(|| {
                    let id = k;
                    k+=1;
                    id
                }); //This function only runs when key is not found
                //so it inserts the new key and registers a new token id to it!
            }
        }
    }

    fn encode(&self, input: &str) -> Vec<u32> {
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut words_split: Vec<(String, Vec<String>)> = Vec::new();
        for word in words {
            let mut temp_vec: Vec<String> = Vec::new();
            for char in word.chars() {
                temp_vec.push(char.to_string());
            }
            words_split.push((word.to_string(), temp_vec));
        }

        for rules in &self.merge_rules {
            for (_, char_seq) in &mut words_split {
                let mut temp_vec: Vec<String> = Vec::new();
                let mut i = 0;
                while i < char_seq.len() {
                    if i+1 < char_seq.len() && rules.0 == char_seq[i] && rules.1 == char_seq[i+1] {
                        temp_vec.push(format!("{}{}", rules.0, rules.1));
                        i += 2;
                    }else {
                        temp_vec.push(char_seq[i].clone());
                        i += 1;
                    }
                }
                *char_seq = temp_vec;
            }
        }

        let mut ids: Vec<u32> = Vec::new();
        for (_, char_seq) in &words_split {
            for char in char_seq {
                if let Some(id) = self.vocab.get(char) {
                    ids.push(*id);
                }else {
                    ids.push(0);
                }
            }
        }
        ids
    }
}

fn main() {
    let mut input = String::new();
    println!("Input the text for training purpose: ");
    io::stdin().read_line(&mut input).unwrap();
    //read_line returns Result ={Ok | Err}
    //unwrap -> give the value of Ok() and panic if Err()
    let input = input.trim(); //Removes the whitespaces from both the end

    let mut tokenizer = BPETokenizer::new();
    tokenizer.train(input);

    println!("Merge rules: {:?}", tokenizer.merge_rules);
    println!("Vocab: {:?}", tokenizer.vocab);

    let mut encode_input = String::new();
    println!("Input the text for encoding purpose: ");
    io::stdin().read_line(&mut encode_input).unwrap();

    let encode_input = encode_input.trim();
    let ids: Vec<u32> = tokenizer.encode(encode_input);
    println!("Encoded: {:?}", ids);
}
