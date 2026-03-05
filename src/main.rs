use std::env;

mod parseur;
mod plurality;
mod trs;
mod irv;
mod borda;
mod bucklin;

use parseur::{parseur, read_lines};
use plurality::plurality;
use trs::trs;
use irv::irv;
use borda::borda;
use bucklin::bucklin;

pub struct Candidat(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Erreur : veuillez fournir un fichier scrutin en argument.");
        return;
    }
    // Faudra m'expliquer comment tu utilises les args
    let mut lignes = read_lines(&args[1]);
    let (candidat, bdd) = parseur(&mut lignes);
    // On passe des références (&) : pas besoin de copier les candidats
    println!("Plurality: {}", plurality(&candidat, &bdd));
    println!("TRS: {}", trs(&candidat, &bdd));
    println!("IRV: {}", irv(&candidat, &bdd));
    println!("Borda: {}", borda(&candidat, &bdd));
    println!("Bucklin: {}", bucklin(&candidat, &bdd));
}