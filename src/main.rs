use std::io::{self, Write};
use unidecode::unidecode;
use reqwest;
use serde_json::Value;

async fn fetch_word() -> String {
    let client = reqwest::Client::new();
    let response = client.get("https://trouve-mot.fr/api/random")
        .send()
        .await
        .expect("Failed to fetch word");

    let words: Vec<Value> = response.json().await.expect("Failed to parse JSON");

    let word = words[0]["name"].as_str().expect("Failed to extract name");
    word.to_string()
}

fn check_inside(letter: char, word: &String) -> bool {
    let unaccented_word = unidecode(word);
    unaccented_word.contains(letter)
}

fn uncovered_letters(word: &String, letters: &Vec<char>) -> String {
    let mut result: String = String::from("");
    for c in word.chars() {
        if letters.contains(&unidecode(c.to_string().as_str()).chars().next().unwrap()) {
            result += c.to_string().as_str();
        } else {
            result += "_";
        }
        result += " ";
    }
    result
}

fn input_letter(word: &String, letters: &mut Vec<char>) -> bool {
    print!("Veuillez entrer une lettre : \n>>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Some(c) = input.trim().chars().next() {
        let c = unidecode(&c.to_string()).chars().next().unwrap().to_lowercase().next().unwrap();
        if check_inside(c, word) && !letters.contains(&c) {
            letters.push(c);
            return true;
        }
    }
    false
}

fn found(word: &String, letters: &Vec<char>) -> bool {
    let unaccented_word = unidecode(word);
    for c in unaccented_word.chars() {
        if !letters.contains(&c) {
            return false;
        }
    }
    true
}

async fn play(mut tries: i8) {
    let word = fetch_word().await;
    let mut letters: Vec<char> = Vec::new();
    while !found(&word, &letters) && tries > 0 {
        println!("{}", uncovered_letters(&word, &letters));
        println!("Essais restants : {tries}");
        if input_letter(&word, &mut letters) {
            println!("Lettre correcte!");
        } else {
            println!("Lettre incorrecte ou déjà entrée.");
            tries -= 1;
        }
    }
    if !found(&word, &letters) {
        println!("Ah… vous avez perdu !\nLe mot était {}", word);
    } else {
        println!("{}\n Bravo !", word);
    }
}

#[tokio::main]
async fn main() {
    play(10).await;
}

