use crate::Candidat;
use crate::erreur;
use std::fs::read_to_string;
use std::collections::HashMap;

// Lit le fichier et parse directement sans créer de Vec<String> intermédiaire
pub fn parseur(fichier: &str) -> Result<(Vec<Candidat>, Vec<Vec<usize>>), ()> {
    let contenu = match read_to_string(fichier) {
        Ok(c) => c,
        Err(e) => {
            erreur::erreur_fichier(fichier, &format!("{}", e));
            return Err(());
        }
    };

    let mut candidat: Vec<Candidat> = Vec::new();

    // HashMap pour retrouver l'indice d'un candidat par son nom en O(1)
    let mut index_candidat: HashMap<String, usize> = HashMap::new();

    // Trouver la première ligne (en-tête)
    let mut lignes = contenu.lines();
    let entete = match lignes.next() {
        Some(l) => l,
        None => {
            erreur::erreur_fichier(fichier, "le fichier est vide");
            return Err(());
        }
    };

    // En-tête : on split par ';' et on trim les espaces
    for m in entete.split(';') {
        let nom = m.trim().to_string();
        if !nom.is_empty() {
            if index_candidat.contains_key(&nom) {
                erreur::erreur_parsing(fichier, 1, &format!("candidat '{}' en doublon dans l'en-tête", nom));
                return Err(());
            }
            index_candidat.insert(nom.clone(), candidat.len());
            candidat.push(Candidat(nom));
        }
    }

    if candidat.is_empty() {
        erreur::erreur_parsing(fichier, 1, "aucun candidat trouvé dans l'en-tête");
        return Err(());
    }

    // Pré-allouer bdd avec une estimation (on sait pas combien y'a de lignes, mais on réserve large)
    let nb_candidats = candidat.len();
    let mut bdd: Vec<Vec<usize>> = Vec::new();

    // Tableau pour détecter les doublons dans un bulletin en O(1)
    let mut vu: Vec<bool> = Vec::new();
    for _c in &candidat {
        vu.push(false);
    }

    // Bulletins : on itère directement sur les lignes restantes du contenu
    let mut nb_bulletins_ignores = 0;
    let mut num_ligne: usize = 1;
    for l in lignes {
        num_ligne += 1;
        if l.trim().is_empty() {
            continue;
        }
        // Pré-allouer le bulletin avec la taille max possible
        let mut bulletin: Vec<usize> = Vec::with_capacity(nb_candidats);
        let mut bulletin_valide = true;
        for m in l.split('>') {
            let nom = m.trim();
            if !nom.is_empty() {
                match index_candidat.get(nom) {
                    Some(idx) => {
                        let i = *idx;
                        if vu[i] {
                            erreur::erreur_parsing(fichier, num_ligne, &format!("candidat '{}' en doublon dans le bulletin", nom));
                            bulletin_valide = false;
                            break;
                        }
                        vu[i] = true;
                        bulletin.push(i);
                    }
                    None => {
                        erreur::erreur_parsing(fichier, num_ligne, &format!("candidat '{}' inconnu (absent de l'en-tête)", nom));
                        bulletin_valide = false;
                        break;
                    }
                }
            }
        }
        // Remettre vu[] à false pour le prochain bulletin
        for b in &bulletin {
            vu[*b] = false;
        }
        if bulletin_valide && !bulletin.is_empty() {
            bdd.push(bulletin);
        } else if !bulletin_valide {
            nb_bulletins_ignores += 1;
        }
    }

    if nb_bulletins_ignores > 0 {
        eprintln!("[ATTENTION]: {} bulletin(s) ignoré(s) dans {}", nb_bulletins_ignores, fichier);
    }

    if bdd.is_empty() {
        erreur::erreur_donnees(&format!("{} - aucun bulletin valide trouvé", fichier));
        return Err(());
    }

    return Ok((candidat, bdd));
}
