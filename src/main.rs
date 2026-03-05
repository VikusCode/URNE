use std::env;
use std::fs::read_to_string;

mod plurality;
mod trs;
mod irv;
mod borda;

use plurality::plurality;
use trs::trs;
use irv::irv;
use borda::borda;

struct Candidat(String);

fn parseur(list : &mut [String]) -> (Vec<Candidat>, Vec<Vec<usize>>) {
    let mut candidat: Vec<Candidat> = Vec::new();
    let mut bdd: Vec<Vec<usize>> = Vec::new();

    // En-tête : on split par ';' et on trim les espaces
    let morceaux: Vec<&str> = list[0].split(';').collect();
    for m in morceaux {
        let nom = m.trim().to_string();
        if !nom.is_empty() {
            let t = Candidat(nom);
            candidat.push(t);
        }
    }

    // Bulletins : on split par '>' et on trim les espaces (Syntaxe formelle de l'énoncé)
    // On convertit chaque nom en indice du candidat pour éviter de chercher à chaque fois
    for l in &list[1..list.len()] {
        let mut bulletin: Vec<usize> = Vec::new();
        if l.trim().is_empty() {
            continue;
        }
        let morceaux: Vec<&str> = l.split('>').collect();
        for m in morceaux {
            let nom = m.trim();
            if !nom.is_empty() {
                // Chercher l'indice du candidat
                for i in 0..candidat.len() {
                    if nom == candidat[i].0 {
                        bulletin.push(i);
                        break;
                    }
                }
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
    // Faudra m'expliquer comment tu utilises les args
    let mut lignes = read_lines(&args[1]);
    let (candidat, bdd) = parseur(&mut lignes);
    // On passe des références (&) : pas besoin de copier les candidats
    println!("Plurality: {}", plurality(&candidat, &bdd));
    println!("TRS: {}", trs(&candidat, &bdd));
    println!("IRV: {}", irv(&candidat, &bdd));
    println!("Borda: {}", borda(&candidat, &bdd));
}