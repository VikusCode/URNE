// Gestion des erreurs du programme
// Format : [TYPE]: description

pub fn erreur_fichier(fichier: &str, detail: &str) {
    eprintln!("[ERREUR FICHIER]: {} - {}", fichier, detail);
}

pub fn erreur_parsing(fichier: &str, ligne: usize, detail: &str) {
    eprintln!("[ERREUR PARSING]: {} ligne {} - {}", fichier, ligne, detail);
}

pub fn erreur_donnees(detail: &str) {
    eprintln!("[ERREUR DONNEES]: {}", detail);
}
