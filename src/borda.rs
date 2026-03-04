use crate::Candidat;

pub fn borda(candidat : &Vec<Candidat>, bdd : &Vec<Vec<String>>) -> String {
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if bulletin.is_empty() {
            continue;
        }
        let mut point = bulletin.len() as i32;
        // On cherche quel candidat correspond au premier du bulletin
        for elt in bulletin {
            for i in 0..candidat.len() {
                if *elt == candidat[i].0 {
                    scores[i] += point;
                    point -= 1;
                }
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
