use crate::Candidat;

// Maybe vérifier plus tard si une meilleure complexité existe
// bdd contient des indices de candidats (usize) au lieu de noms (String)
pub fn plurality(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if bulletin.is_empty() {
            continue;
        }
        // bulletin[0] est directement l'indice du candidat en première position
        let premier = bulletin[0];
        scores[premier] += 1;
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
