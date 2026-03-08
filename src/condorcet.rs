use crate::Candidat;

// Calcule la matrice des duels : duels[i][j] = nombre de bulletins qui préfèrent i à j
// Un candidat i est préféré à j si i apparaît avant j dans le bulletin
pub fn matrice_duels(candidat: &Vec<Candidat>, bdd: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
    let n = candidat.len();

    // a[i][j] = nombre de bulletins où i est classé avant j (les deux présents)
    let mut a: Vec<Vec<i32>> = Vec::new();
    for _i in 0..n {
        let mut ligne: Vec<i32> = Vec::new();
        for _j in 0..n {
            ligne.push(0);
        }
        a.push(ligne);
    }

    // has[i] = nombre de bulletins contenant le candidat i
    let mut has: Vec<i32> = Vec::new();
    for _i in 0..n {
        has.push(0);
    }

    for bulletin in bdd {
        // Compter les apparitions
        for c in bulletin {
            has[*c] += 1;
        }
        // Pour chaque paire ordonnée dans le bulletin
        for pos_i in 0..bulletin.len() {
            for pos_j in (pos_i + 1)..bulletin.len() {
                a[bulletin[pos_i]][bulletin[pos_j]] += 1;
            }
        }
    }

    // duels[i][j] = has[i] - a[j][i]
    // = (bulletins où i présent et j classé avant i sont soustraits)
    // = (bulletins où i avant j) + (bulletins où i présent et j absent)
    let mut duels: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        let mut ligne: Vec<i32> = Vec::new();
        for j in 0..n {
            if i == j {
                ligne.push(0);
            } else {
                ligne.push(has[i] - a[j][i]);
            }
        }
        duels.push(ligne);
    }

    duels
}
