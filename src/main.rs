/*
test*/

extern crate regex;

use std::env;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;


fn word_count(file_name: &str) -> Result<HashMap<String, u32>, io::Error > {
    let mut dict = HashMap::new();
    let re = Regex::new(r"\W").unwrap();
    let file = File::open(file_name).expect("cant open file");
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        for word in line.unwrap().split_whitespace() {
            let tword = word.to_string().to_lowercase();
            let tword = re.replace_all(&tword, "").into_owned();
            let q = dict.entry(tword).or_insert(1);
            *q += 1
        }
    }
    Ok(dict)
}

fn main() {
    let files = env::args().skip(1);

    let mut total_dict = HashMap::new();
    for arg in files{
        let dict = word_count(&arg);
        for (word, count) in dict.unwrap().iter() {
            let q = total_dict.entry(word.clone()).or_insert(*count);
            *q += *count;
        }
    }

    let mut count_vec: Vec<(&String, &u32)> = total_dict.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for vec_val in count_vec[0..10].iter() {
        println!("{:?}: {:?}", &vec_val.0, &vec_val.1);
    }
//    println!("Most frequent character in text: {}", count_vec[0].0);
//    println!("{:?}", total_dict);
}
