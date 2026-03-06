use crate::Candidat;

// Smith+IRV : éliminer successivement de l'ensemble de Smith le candidat
// ayant le moins de premiers choix (IRV), jusqu'à trouver un gagnant de Condorcet.
pub fn smith_irv(candidat: &Vec<Candidat>, bdd: &Vec<Vec<usize>>, duels: &Vec<Vec<i32>>) -> String {
    let n = candidat.len();

    // 1) Construire l'ensemble de Smith
    let mut en_lice = crate::smith::ensemble_smith(duels);

    // Position courante dans chaque bulletin
    let mut pos: Vec<usize> = Vec::new();
    for _b in bdd {
        pos.push(0);
    }

    loop {
        // 2) Vérifier s'il y a un gagnant de Condorcet parmi les candidats en lice
        for i in 0..n {
            if !en_lice[i] {
                continue;
            }
            let mut bat_tous = true;
            for j in 0..n {
                if j == i || !en_lice[j] {
                    continue;
                }
                if duels[i][j] <= duels[j][i] {
                    bat_tous = false;
                    break;
                }
            }
            if bat_tous {
                return candidat[i].0.clone();
            }
        }

        // Compter combien restent en lice
        let mut nb_en_lice = 0;
        for i in 0..n {
            if en_lice[i] {
                nb_en_lice += 1;
            }
        }
        if nb_en_lice <= 1 {
            for i in 0..n {
                if en_lice[i] {
                    return candidat[i].0.clone();
                }
            }
        }

        // 3) Compter les premiers choix parmi les candidats en lice
        let mut scores: Vec<i32> = Vec::new();
        for _i in 0..n {
            scores.push(0);
        }
        for b in 0..bdd.len() {
            // Avancer tant que le candidat pointé est éliminé
            while pos[b] < bdd[b].len() && !en_lice[bdd[b][pos[b]]] {
                pos[b] += 1;
            }
            if pos[b] < bdd[b].len() {
                scores[bdd[b][pos[b]]] += 1;
            }
        }

        // 4) Trouver le min et éliminer (lexico : on élimine le plus grand nom)
        let mut min_score = i32::MAX;
        for i in 0..n {
            if en_lice[i] && scores[i] < min_score {
                min_score = scores[i];
            }
        }
        let mut elimine = usize::MAX;
        for i in 0..n {
            if en_lice[i] && scores[i] == min_score {
                if elimine == usize::MAX || candidat[i].0 > candidat[elimine].0 {
                    elimine = i;
                }
            }
        }
        en_lice[elimine] = false;
    }
}
