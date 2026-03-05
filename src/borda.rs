use crate::Candidat;

// bdd contient des indices de candidats (usize) au lieu de noms (String)
pub fn borda(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if bulletin.is_empty() {
            continue;
        }
        // Le premier reçoit bulletin.len() points, le 2ème bulletin.len()-1, etc.
        let mut point = bulletin.len() as i32;
        for elt in bulletin {
            scores[*elt] += point;
            point -= 1;
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
