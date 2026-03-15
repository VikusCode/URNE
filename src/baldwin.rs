use crate::Candidat;

// Baldwin : Borda itératif, on élimine le candidat avec le plus faible score Borda à chaque tour.
// Les points sont calculés bulletin par bulletin sur les candidats encore en lice : k, k-1, ..., 1.
pub fn baldwin(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
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

        // scores[i] = score Borda au tour courant
        let mut min_score = i32::MAX;
        let mut elimine = usize::MAX;
        let mut scores: Vec<i32> = Vec::new();
        for _i in 0..n {
            scores.push(0);
        }

        for bulletin in bdd {
            // k = nombre de candidats encore en lice présents dans ce bulletin
            let mut k = 0;
            for c in bulletin {
                if en_lice[*c] {
                    k += 1;
                }
            }
            if k == 0 {
                continue;
            }
            let mut points = k as i32;
            for c in bulletin {
                if en_lice[*c] {
                    scores[*c] += points;
                    points -= 1;
                }
            }
        }

        for i in 0..n {
            if !en_lice[i] {
                continue;
            }
            let score = scores[i];
            if score < min_score || (score == min_score && candidat[i].0 > candidat[elimine].0) {
                min_score = score;
                elimine = i;
            }
        }

        en_lice[elimine] = false;
        nb_en_lice -= 1;
    }
}