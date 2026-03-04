use std::env;
use std::fs::read_to_string;

mod plurality;
mod trs;

use plurality::plurality;
use trs::trs;

struct Candidat(String);

fn parseur(list : &mut [String]) -> (Vec<Candidat>, Vec<Vec<String>>) {
    let mut candidat: Vec<Candidat> = Vec::new();
    let mut bdd: Vec<Vec<String>> = Vec::new();

    // En-tête : on split par ';' et on trim les espaces
    let morceaux: Vec<&str> = list[0].split(';').collect();
    for m in morceaux {
        let nom = m.trim().to_string();
        if !nom.is_empty() {
            let t = Candidat(nom);
            candidat.push(t);
        }
    }

    // Bulletins:on split par '>' et on trim les espaces (Syntaxe formelle de l'énoncé)
    for l in &list[1..list.len()] {
        let mut bulletin: Vec<String> = Vec::new();
        if l.trim().is_empty() {
            continue;
        }
        let morceaux: Vec<&str> = l.split('>').collect();
        for m in morceaux {
            let nom = m.trim().to_string();
            if !nom.is_empty() {
                bulletin.push(nom);
            }
        }
        bdd.push(bulletin);
    }

    return (candidat, bdd);
}

// Petit rappel à moi-même : regarder quand mettre des & et des mut pour itérer, ect...

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
     .unwrap()  // déclenche une panique en cas d'erreur possible lors de la lecture du fichier
     .lines()  // divise la chaîne en un itérateur de fragments de chaîne
     .map(String::from)  // transforme chaque fragment en une chaîne
     .collect()  // rassemble tout dans un vecteur
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Erreur : veuillez fournir un fichier scrutin en argument.");
        return;
    }
    let mut lignes = read_lines(&args[1]);
    let (candidat, bdd) = parseur(&mut lignes);
    // On passe des références (&) : pas besoin de copier les candidats
    println!("Plurality: {}", plurality(&candidat, &bdd));
    println!("TRS: {}", trs(&candidat, &bdd));
}