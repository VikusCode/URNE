use crate::Candidat;

#[derive(Clone)]
pub struct Match(i32, i32, i32);

pub fn schulze(candidat: &Vec<Candidat>, duels: &Vec<Vec<i32>>) -> String {
    // 1) Construire l'ensemble de Smith
    let en_lice = crate::smith::ensemble_smith(duels);
    let mut candidat_smith: Vec<Candidat> = Vec::new();
    let mut les_matchs: Vec<Match> = Vec::new();
    let indices_smith: Vec<usize> = (0..en_lice.len()).filter(|&i| en_lice[i]).collect();

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

    for i1 in 0..n {
        for j1 in (i1 + 1)..n {
            let i = indices_smith[i1];
            let j = indices_smith[j1];
            if duels[i][j] > duels[j][i] {
                les_matchs.push(Match(j1 as i32, duels[i][j] - duels[j][i], i1 as i32));
                scores[i1] += 1; // i bat j
            } else if duels[i][j] < duels[j][i] {
                scores[j1] += 1; // j bat i
                les_matchs.push(Match(i1 as i32, duels[j][i] - duels[i][j], j1 as i32));
            } else {
                les_matchs.push(Match(-1, 0, -1)) // si nul, on ne fait rien
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
        let mut gagnants: Vec<usize> = Vec::new();
        for confront in &match_restants {
            let indice = confront.0 as usize;
            let part = confront.2 as usize;
            if !perdants.contains(&indice) {
                perdants.push(indice);
            }
            if !gagnants.contains(&part) {
                gagnants.push(part);
            }
        }

        for win in 0..n {
            if !perdants.contains(&win) {
                return candidat_smith[win].0.clone();
            }
        }

        les_matchs = match_restants;
    }
}