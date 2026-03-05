use crate::Candidat;

// bdd contient des indices de candidats (usize) au lieu de noms (String)
pub fn bucklin(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
    // Scores cumulés pour chaque candidat
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    // Compter les votes pour la majorité absolue
    let mut total = 0;
    for bulletin in bdd {
        if !bulletin.is_empty() {
            total += 1;
        }
    }
    let majorite = total / 2 + 1;

    // Trouver le rang max parmi tous les bulletins
    let mut rang_max = 0;
    for bulletin in bdd {
        if bulletin.len() > rang_max {
            rang_max = bulletin.len();
        }
    }

    // boucle sur les rangs (k = 0, 1, 2, ...)
    for k in 0..rang_max {
        // Ajouter les voix du rang k aux scores cumulés
        for bulletin in bdd {
            if k < bulletin.len() {
                scores[bulletin[k]] += 1;
            }
        }

        // Vérifier si un candidat a la majorité absolue
        let mut quelquun_a_majorite = false;
        for i in 0..candidat.len() {
            if scores[i] >= majorite {
                quelquun_a_majorite = true;
                break;
            }
        }

        if quelquun_a_majorite {
            // Élire celui qui a le plus de voix
            let mut max_score = -1;
            let mut max_cand = String::new();
            for i in 0..candidat.len() {
                if scores[i] > max_score || (scores[i] == max_score && candidat[i].0 < max_cand) {
                    max_score = scores[i];
                    max_cand = candidat[i].0.clone();
                }
            }
            return max_cand;
        }
    }

    //  celui avec le plus de voix gagne (pas de majorité abs possible)
    let mut max_score = -1;
    let mut max_cand = String::new();
    for i in 0..candidat.len() {
        if scores[i] > max_score || (scores[i] == max_score && candidat[i].0 < max_cand) {
            max_score = scores[i];
            max_cand = candidat[i].0.clone();
        }
    }
    max_cand
}
