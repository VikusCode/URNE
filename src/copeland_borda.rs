use crate::Candidat;

// Copeland+Borda : Copeland pour le classement principal, Borda pour départager les ex-aequo
pub fn copeland_borda(candidat: &Vec<Candidat>, bdd: &Vec<Vec<usize>>, duels: &Vec<Vec<i32>>) -> String {
    let n = candidat.len();

    // 1) Calculer les scores Copeland
    let mut scores_cop: Vec<i32> = Vec::new();
    for _i in 0..n {
        scores_cop.push(0);
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if duels[i][j] > duels[j][i] {
                scores_cop[i] += 2;
            } else if duels[i][j] < duels[j][i] {
                scores_cop[j] += 2;
            } else {
                scores_cop[i] += 1;
                scores_cop[j] += 1;
            }
        }
    }

    // 2) Calculer les scores Borda
    let mut scores_borda: Vec<i32> = Vec::new();
    for _i in 0..n {
        scores_borda.push(0);
    }
    for bulletin in bdd {
        if bulletin.is_empty() {
            continue;
        }
        let mut point = bulletin.len() as i32;
        for elt in bulletin {
            scores_borda[*elt] += point;
            point -= 1;
        }
    }

    // 3) Trouver le gagnant : max Copeland, puis Borda pour départager, puis lexico
    let mut max_cop = -1;
    let mut max_borda = -1;
    let mut max_cand = String::new();
    for i in 0..n {
        if scores_cop[i] > max_cop
            || (scores_cop[i] == max_cop && scores_borda[i] > max_borda)
            || (scores_cop[i] == max_cop && scores_borda[i] == max_borda && candidat[i].0 < max_cand)
        {
            max_cop = scores_cop[i];
            max_borda = scores_borda[i];
            max_cand = candidat[i].0.clone();
        }
    }
    max_cand
}
