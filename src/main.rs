use std::fs::read_to_string;

fn parseur(list : &mut [String]) {
    println!("{:?}", list);
    for l in list {
        for elt in l.chars() {
            println!("{}", elt);
        }
        println!("Séparation");
    }
}

// fn plurality() {

// }

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
     .unwrap()  // déclenche une panique en cas d'erreur possible lors de la lecture du fichier
     .lines()  // divise la chaîne en un itérateur de fragments de chaîne
     .map(String::from)  // transforme chaque fragment en une chaîne
     .collect()  // rassemble tout dans un vecteur
}

fn main() {    
    let mut participants: Vec<String> = Vec::new();
    participants.push(String::from("Bonjour"));
    let mut lignes = read_lines("./src/fichier.txt");
    for ligne in &lignes {
        println!("{}", ligne);
    }
    parseur(&mut lignes);
}