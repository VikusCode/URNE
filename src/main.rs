use std::env;
use std::fs::read_to_string;

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

// Maybe vérifier plus tard si une meilleure complexité existe
fn plurality(candidat : &Vec<Candidat>, bdd : &Vec<Vec<String>>) -> String {
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if bulletin.is_empty() {
            continue;
        }
        let premier = &bulletin[0];
        // On cherche quel candidat correspond au premier du bulletin
        for i in 0..candidat.len() {
            if *premier == candidat[i].0 {
                scores[i] += 1;
            }
        }
    }

    // Trouver max égalité 
    let mut max = -1;
    let mut max_cand = String::new();
    for i in 0..candidat.len() {
        if scores[i] > max || (scores[i] == max && candidat[i].0 < max_cand) {
            max = scores[i];
            max_cand = candidat[i].0.clone(); // clone juste pour le résultat final
        }
    }
    max_cand
}

fn trs(candidat : &Vec<Candidat>, bdd : &Vec<Vec<String>>) -> String {
    // Tour 1 
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if !bulletin.is_empty() {
            let premier = &bulletin[0];
            for i in 0..candidat.len() {
                if *premier == candidat[i].0 {
                    scores[i] += 1;
                }
            }
        }
    }

    let mut total = 0;
    for i in 0..scores.len() {
        total += scores[i];
    }
    let majorite = total / 2 + 1;

    // Trouver les deux meilleurs indices (tri par score desc, lexico asc)
    // On crée un tableau d'indices qu'on trie
    let mut indices: Vec<usize> = Vec::new();
    for i in 0..candidat.len() {
        indices.push(i);
    }
    indices.sort_by(|a, b| scores[*b].cmp(&scores[*a]).then_with(|| candidat[*a].0.cmp(&candidat[*b].0)));

    // Si le premier a la majorité absolue = élu
    if scores[indices[0]] >= majorite {
        return candidat[indices[0]].0.clone();
    }

    // Sinon second tour entre les deux premiers
    let i1 = indices[0]; // indice du premier finaliste
    let i2 = indices[1]; // indice du deuxième finaliste
    let mut s1 = 0;
    let mut s2 = 0;
    for bulletin in bdd {
        for c in bulletin {
            if *c == candidat[i1].0 {
                s1 += 1;
                break;
            } else if *c == candidat[i2].0 {
                s2 += 1;
                break;
            }
        }
    }
    if s1 > s2 {
        candidat[i1].0.clone()
    } else if s2 > s1 {
        candidat[i2].0.clone()
    } else {
        // Egalité = ordre lexicographique
        if candidat[i1].0 <= candidat[i2].0 {
            candidat[i1].0.clone()
        } else {
            candidat[i2].0.clone()
        }
    }
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



// let mut participants: Vec<String> = Vec::new();
// participants.push(String::from("Bonjour"));