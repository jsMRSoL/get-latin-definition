use reqwest;
use select::document::Document;
use select::predicate::Class;
use std::env;
use std::error::Error;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_slice = &args[1..];
    for arg in args_slice.iter() {
        match latin_definition(arg) {
            Ok(answer) => print!("{}", answer),
            Err(e) => println!("Something went wrong {}", e),
        }
    }
}

fn latin_definition(s: &str) -> Result<String, Box<dyn Error>> {
    // construct query
    let query = format!("http://www.perseus.tufts.edu/hopper/morph?l={}&la=la", s);

    // get page
    let res = reqwest::blocking::get(&query)?;

    // parsing
    let doc = Document::from_read(res)?;
    let mut answer = String::new();
    for node in doc.find(Class("analysis")) {
        // get latin headword
        let mut entry = match node.find(Class("la")).next() {
            Some(w) => w.text(),
            None => "None".to_string(),
        };
        entry = entry.trim().to_string();
        // get lemmas
        let mut def = match node.find(Class("lemma_definition")).next() {
            Some(l) => l.text(),
            None => "None".to_string(),
        };
        def = def.trim().to_string();
        answer.push_str(&format!("{}: {}\n", entry, def));
    }
    Ok(answer)
}
