use crate::Candidat;

// bdd contient des indices de candidats (usize) au lieu de noms (String)
pub fn trs(candidat : &Vec<Candidat>, bdd : &Vec<Vec<usize>>) -> String {
    // Tour 1 
    let mut scores: Vec<i32> = Vec::new();
    for _c in candidat {
        scores.push(0);
    }

    for bulletin in bdd {
        if !bulletin.is_empty() {
            // bulletin[0] est directement l'indice du candidat en première position
            let premier = bulletin[0];
            scores[premier] += 1;
        }
    }

    let mut total = 0;
    for i in 0..scores.len() {
        total += scores[i];
    }
    let majorite = total / 2 + 1;

    // Trouver les deux meilleurs indices (tri par score desc, lexico asc)
    // On crée un tableau d'indices qu'on trie
    let mut indices: Vec<usize> = Vec::new();
    for i in 0..candidat.len() {
        indices.push(i);
    }
    indices.sort_by(|a, b| scores[*b].cmp(&scores[*a]).then_with(|| candidat[*a].0.cmp(&candidat[*b].0)));

    // Si le premier a la majorité absolue = élu
    if scores[indices[0]] >= majorite {
        return candidat[indices[0]].0.clone();
    }

    // Sinon second tour entre les deux premiers
    let i1 = indices[0]; // indice du premier finaliste
    let i2 = indices[1]; // indice du deuxième finaliste
    let mut s1 = 0;
    let mut s2 = 0;
    for bulletin in bdd {
        // Parcourir le bulletin : le premier finaliste rencontré reçoit le vote
        for c in bulletin {
            if *c == i1 {
                s1 += 1;
                break;
            } else if *c == i2 {
                s2 += 1;
                break;
            }
        }
    }
    if s1 > s2 {
        candidat[i1].0.clone()
    } else if s2 > s1 {
        candidat[i2].0.clone()
    } else {
        // Egalité = ordre lexicographique
        if candidat[i1].0 <= candidat[i2].0 {
            candidat[i1].0.clone()
        } else {
            candidat[i2].0.clone()
        }
    }
}
