use crate::Candidat;

// Copeland : 2 points par victoire, 1 point par nul, 0 par défaite
pub fn copeland(candidat: &Vec<Candidat>, duels: &Vec<Vec<i32>>) -> String {
    let n = candidat.len();
    let mut scores: Vec<i32> = Vec::new();
    for _i in 0..n {
        scores.push(0);
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if duels[i][j] > duels[j][i] {
                scores[i] += 2; // i bat j
            } else if duels[i][j] < duels[j][i] {
                scores[j] += 2; // j bat i
            } else {
                scores[i] += 1; // nul
                scores[j] += 1;
            }
        }
    }

    // Trouver le max 
    let mut max_score = -1;
    let mut max_cand = String::new();
    for i in 0..n {
        if scores[i] > max_score || (scores[i] == max_score && candidat[i].0 < max_cand) {
            max_score = scores[i];
            max_cand = candidat[i].0.clone();
        }
    }
    max_cand
}
