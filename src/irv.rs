use crate::Candidat;

// bdd contient des indices de candidats (usize) au lieu de noms (String)
pub fn irv(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
    // Liste des candidats encore en lice (true = encore présent)
    let mut en_lice: Vec<bool> = Vec::new();
    for _c in candidat {
        en_lice.push(true);
    }
    // Position courante dans chaque bulletin (pour ne pas re-parcourir les éliminés)
    let mut pos: Vec<usize> = Vec::new();
    for _b in bdd {
        pos.push(0);
    }
    loop {
        // Compter les voix en première position (parmi les candidats encore en lice)
        let mut scores: Vec<i32> = Vec::new();
        for _c in candidat {
            scores.push(0);
        }
        for b in 0..bdd.len() {
            // Avancer tant que le candidat pointé est éliminé
            while pos[b] < bdd[b].len() && !en_lice[bdd[b][pos[b]]] {
                pos[b] += 1;
            }
            // Si on a trouvé un candidat encore en lice, compter sa voix
            if pos[b] < bdd[b].len() {
                scores[bdd[b][pos[b]]] += 1;
            }
        }
        // Compter le total des suffrages exprimés (bulletins qui ont au moins un candidat en lice)
        let mut total = 0;
        for i in 0..candidat.len() {
            if en_lice[i] {
                total += scores[i];
            }
        }
        let majorite = total / 2 + 1;
        // Vérifier si un candidat a la majorité absolue
        for i in 0..candidat.len() {
            if en_lice[i] && scores[i] >= majorite {
                return candidat[i].0.clone();
            }
        }
        // Compter combien de candidats sont encore en lice
        let mut nb_en_lice = 0;
        for i in 0..candidat.len() {
            if en_lice[i] {
                nb_en_lice += 1;
            }
        }
        // S'il ne reste qu'un seul candidat, il gagne
        if nb_en_lice <= 1 {
            for i in 0..candidat.len() {
                if en_lice[i] {
                    return candidat[i].0.clone();
                }
            }
        }
        // Trouver le score minimum parmi les candidats encore en lice
        let mut min_score = i32::MAX;
        for i in 0..candidat.len() {
            if en_lice[i] && scores[i] < min_score {
                min_score = scores[i];
            }
        }
        // Parmi ceux qui ont le score minimum, éliminer celui avec le plus grand nom (lexico)
        // Car en cas d'égalité on garde le plus petit en lexico = on élimine le plus grand
        let mut elimine = usize::MAX;
        for i in 0..candidat.len() {
            if en_lice[i] && scores[i] == min_score {
                if elimine == usize::MAX || candidat[i].0 > candidat[elimine].0 {
                    elimine = i;
                }
            }
        }
        // Éliminer ce candidat
        en_lice[elimine] = false;
    }
}
