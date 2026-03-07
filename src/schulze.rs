use crate::Candidat;

#[derive(Clone)]
pub struct Match(i32, i32);

pub fn schulze(candidat: &Vec<Candidat>, duels: &Vec<Vec<i32>>) -> String {
    // 1) Construire l'ensemble de Smith
    let en_lice = crate::smith::ensemble_smith(duels);
    let mut candidat_smith: Vec<Candidat> = Vec::new();
    let mut les_matchs: Vec<Match> = Vec::new();

    let mut n = 0;
    for boolean in 0..en_lice.len() {
        let cand = Candidat(candidat[boolean].0.clone());
        if en_lice[boolean] {
            n += 1;
            candidat_smith.push(cand);
        }
    }

    let mut scores: Vec<i32> = Vec::new();
    for _i in 0..n {
        scores.push(0);
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if duels[i][j] > duels[j][i] {
                les_matchs.push(Match(j as i32, duels[i][j] - duels[j][i]));
                scores[i] += 1; // i bat j
            } else if duels[i][j] < duels[j][i] {
                scores[j] += 1; // j bat i
                les_matchs.push(Match(i as i32, duels[j][i] - duels[i][j]));
            } else {
                les_matchs.push(Match(-1, 0)) // si nul, on ne fait rien
            }
        }   
    }

    loop {
        let mut min = i32::MAX;
        let mut match_restants: Vec<Match> = Vec::new();
        for confront in &les_matchs {
            if confront.1 >= 0 && min > confront.1 {
                min = confront.1;
            }
        }
        if min == i32::MAX {
            return candidat_smith[0].0.clone();
        }
        for confront in &les_matchs {
            if confront.1 != min {
                match_restants.push(confront.clone());
            }
        }
        let mut perdants: Vec<usize> = Vec::new();
        for confront in &match_restants {
            let indice = confront.0 as usize;
            if !perdants.contains(&indice) {
                perdants.push(indice);
            }
        }
        if perdants.len() == candidat_smith.len() - 1 {
            for compteur in 0..candidat_smith.len() {
                if !perdants.contains(&compteur) {
                    return candidat_smith[compteur].0.clone();
                }
            }
        }
        les_matchs = match_restants;
    }
}