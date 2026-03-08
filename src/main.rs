use std::env;
use std::time::Instant;

mod erreur;
mod parseur;
mod plurality;
mod trs;
mod irv;
mod borda;
mod bucklin;
mod condorcet;
mod copeland;
mod copeland_borda;
mod smith;
mod smith_irv;
mod baldwin;
mod schulze;

use plurality::plurality;
use trs::trs;
use irv::irv;
use borda::borda;
use bucklin::bucklin;

pub struct Candidat(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[ERREUR ARGS]: veuillez fournir un fichier scrutin en argument.");
        eprintln!("Usage : mon-prog <fichier> [--timing]");
        return;
    }

    // Vérifier si le flag --timing est présent
    let mut timing = false;
    for a in &args {
        if a == "--timing" {
            timing = true;
        }
    }

    let debut_total = Instant::now();

    // Lecture + parsing du fichier (tout en un seul appel)
    let debut_parsing = Instant::now();
    let resultat = parseur::parseur(&args[1]);
    let (candidat, bdd) = match resultat {
        Ok(r) => r,
        Err(()) => return,
    };
    let duree_parsing = debut_parsing.elapsed();

    if timing {
        eprintln!("[TIMING] Parsing : {:.6}s", duree_parsing.as_secs_f64());
    }

    // On passe des références (&) : pas besoin de copier les candidats
    let debut = Instant::now();
    println!("Plurality: {}", plurality(&candidat, &bdd));
    if timing { eprintln!("[TIMING] Plurality : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("TRS: {}", trs(&candidat, &bdd));
    if timing { eprintln!("[TIMING] TRS : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("IRV: {}", irv(&candidat, &bdd));
    if timing { eprintln!("[TIMING] IRV : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Borda: {}", borda(&candidat, &bdd));
    if timing { eprintln!("[TIMING] Borda : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Bucklin: {}", bucklin(&candidat, &bdd));
    if timing { eprintln!("[TIMING] Bucklin : {:.6}s", debut.elapsed().as_secs_f64()); }

    // Calcul de la matrice des duels (utilisée par Baldwin et toutes les méthodes Condorcet)
    let debut = Instant::now();
    let duels = condorcet::matrice_duels(&candidat, &bdd);
    if timing { eprintln!("[TIMING] Matrice duels : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Baldwin: {}", baldwin::baldwin(&candidat, &duels));
    if timing { eprintln!("[TIMING] Baldwin : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Copeland: {}", copeland::copeland(&candidat, &duels));
    if timing { eprintln!("[TIMING] Copeland : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Copeland+Borda: {}", copeland_borda::copeland_borda(&candidat, &bdd, &duels));
    if timing { eprintln!("[TIMING] Copeland+Borda : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Schulze: {}", schulze::schulze(&candidat, &duels));
    if timing { eprintln!("[TIMING] Schulze : {:.6}s", debut.elapsed().as_secs_f64()); }

    let debut = Instant::now();
    println!("Smith+IRV: {}", smith_irv::smith_irv(&candidat, &bdd, &duels));
    if timing { eprintln!("[TIMING] Smith+IRV : {:.6}s", debut.elapsed().as_secs_f64()); }

    if timing {
        eprintln!("[TIMING] Total : {:.6}s", debut_total.elapsed().as_secs_f64());
    }
}
