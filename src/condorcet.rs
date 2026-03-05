use crate::Candidat;

// Calcule la matrice des duels : duels[i][j] = nombre de bulletins qui préfèrent i à j
// Un candidat i est préféré à j si i apparaît avant j dans le bulletin
pub fn matrice_duels(candidat: &Vec<Candidat>, bdd: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
    let n = candidat.len();
    // Créer la matrice n x n initialisée à 0
    let mut duels: Vec<Vec<i32>> = Vec::new();
    for _i in 0..n {
        let mut ligne: Vec<i32> = Vec::new();
        for _j in 0..n {
            ligne.push(0);
        }
        duels.push(ligne);
    }

    // Pour chaque bulletin, tous les candidats qui apparaissent avant un autre
    // sont préférés à ce dernier
    // Les candidats absents du bulletin sont considérés comme classés après tous ceux présents
    for bulletin in bdd {
        // Pour chaque paire (i, j) dans le bulletin : i est préféré à j
        for pos_i in 0..bulletin.len() {
            for pos_j in (pos_i + 1)..bulletin.len() {
                duels[bulletin[pos_i]][bulletin[pos_j]] += 1;
            }
        }
        // Les candidats présents dans le bulletin sont aussi préférés
        // à ceux qui n'y sont pas du tout
        // On utilise un tableau "present" pour savoir qui est dans le bulletin
        let mut present: Vec<bool> = Vec::new();
        for _i in 0..n {
            present.push(false);
        }
        for c in bulletin {
            present[*c] = true;
        }
        for c in bulletin {
            for j in 0..n {
                if !present[j] {
                    duels[*c][j] += 1;
                }
            }
        }
    }

    duels
}
