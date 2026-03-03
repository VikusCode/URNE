use std::fs;

fn parseur(list : &mut [String]) {
    println!("{:?}", list);
}

// fn plurality() {

// }

fn lecture() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}

fn main() {    
    let mut participants: Vec<String> = Vec::new();
    participants.push(String::from("Bonjour"));
    parseur(&mut participants);
    lecture();
}