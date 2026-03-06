// Ensemble de Smith : plus petit sous-ensemble tel qu'aucun candidat hors de l'ensemble
// ne bat un candidat de l'ensemble.
// Algorithme :
// 1. Trouver les vainqueurs Copeland (score max) -> les mettre dans l'ensemble
// 2. Ajouter tout candidat hors de l'ensemble qui bat un candidat de l'ensemble
// 3. Répéter jusqu'à stabilisation
pub fn ensemble_smith(duels: &Vec<Vec<i32>>) -> Vec<bool> {
    let n = duels.len();

    // 1) Calculer les scores Copeland
    let mut scores: Vec<i32> = Vec::new();
    for _i in 0..n {
        scores.push(0);
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if duels[i][j] > duels[j][i] {
                scores[i] += 2;
            } else if duels[i][j] < duels[j][i] {
                scores[j] += 2;
            } else {
                scores[i] += 1;
                scores[j] += 1;
            }
        }
    }

    // Trouver le score max
    let mut max_score = -1;
    for i in 0..n {
        if scores[i] > max_score {
            max_score = scores[i];
        }
    }

    // 2) Initialiser l'ensemble avec les vainqueurs Copeland
    let mut smith: Vec<bool> = Vec::new();
    for i in 0..n {
        smith.push(scores[i] == max_score);
    }

    // 3) Ajouter itérativement tous les candidats qui battent un membre de l'ensemble
    loop {
        let mut change = false;
        for i in 0..n {
            if smith[i] {
                continue; // déjà dans l'ensemble
            }
            // Est-ce que i n'est pas strictement battu par un membre de l'ensemble ?
            // (si i bat ou fait égalité avec un membre, il faut l'ajouter)
            for j in 0..n {
                if smith[j] && duels[i][j] >= duels[j][i] {
                    smith[i] = true;
                    change = true;
                    break;
                }
            }
        }
        if !change {
            break;
        }
    }

    smith
}
