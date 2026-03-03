use std::fs::read_to_string;

struct Candidat(char, i32);

fn parseur(list : &mut [String]) -> (Vec<Candidat>, Vec<Vec<char>>) {
    let mut candidat: Vec<Candidat> = Vec::new();
    let mut bulletin: Vec<char> = Vec::new();
    let mut bdd: Vec<Vec<char>> = Vec::new();

    for l in list[0].chars() {
        if l != ';' {
            let t = Candidat(l, 0);
            candidat.push(t);
        }
    }

    for l in &list[1..list.len()] {
        for elt in l.chars() {
            if elt != '>' {
                bulletin.push(elt);
            }
        }
        bdd.push(bulletin);
        bulletin = vec![];
    }

    return (candidat, bdd);
}

fn plurality(mut candidat : Vec<Candidat>, bdd : &mut Vec<Vec<char>>) -> char {
    for bulletin in bdd {
        let premier = bulletin[0];
        for cand in &mut candidat {
            if premier == cand.0 {
                cand.1 += 1;
            }
        }
    }
    let mut max = -1;
    let mut max_cand = '?';
    for cand in candidat {
        if cand.1 > max {
            max = cand.1;
            max_cand = cand.0;
        }
    }
    max_cand
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
    let mut lignes = read_lines("./src/fichier.txt");
    let (candidat, mut bdd) = parseur(&mut lignes);
    // for cand in candidat {
    //     println!("{} {}", cand.0, cand.1);
    // }
    println!("{:?}", bdd);
    println!("{}", plurality(candidat, &mut bdd));
}



// let mut participants: Vec<String> = Vec::new();
// participants.push(String::from("Bonjour"));

// for ligne in &lignes {
//     println!("{}", ligne);
// }