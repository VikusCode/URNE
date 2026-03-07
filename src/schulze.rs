use crate::Candidat;

pub fn schulze(candidat: &Vec<Candidat>, bdd: &Vec<Vec<usize>>, duels: &Vec<Vec<i32>>) -> String {
    let n = candidat.len();

    // 1) Construire l'ensemble de Smith
    let mut en_lice = crate::smith::ensemble_smith(duels);
}