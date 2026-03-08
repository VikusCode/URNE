use crate::Candidat;

// Baldwin : Borda itératif, on élimine le candidat avec le plus faible score Borda à chaque tour.
// Astuce : score Borda de i = somme de duels[i][j] pour tous j en lice (j != i)
pub fn baldwin(candidat : &Vec<Candidat>, duels: &Vec<Vec<i32>>) -> String {
    let n = candidat.len();
    let mut en_lice: Vec<bool> = Vec::new();
    for _c in candidat {
        en_lice.push(true);
    }
    let mut nb_en_lice = n;

    loop {
        if nb_en_lice <= 1 {
            for i in 0..n {
                if en_lice[i] {
                    return candidat[i].0.clone();
                }
            }
        }

        // Calculer les scores Borda via la matrice des duels
        // score_borda(i) = somme de duels[i][j] pour j en lice, j != i
        let mut min_score = i32::MAX;
        let mut elimine = usize::MAX;

        for i in 0..n {
            if !en_lice[i] {
                continue;
            }
            let mut score = 0;
            for j in 0..n {
                if j != i && en_lice[j] {
                    score += duels[i][j];
                }
            }
            if score < min_score || (score == min_score && candidat[i].0 > candidat[elimine].0) {
                min_score = score;
                elimine = i;
            }
        }

        en_lice[elimine] = false;
        nb_en_lice -= 1;
    }
}