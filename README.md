# Projet URNE

## Utilisation

```
cargo run -- chemin/vers/nom_du_fichier.txt

OU 

.\target\debug\mon_prog.exe chemin/vers/nom_du_fichier.txt


(A LA FIN FAIRE: cargo install --path pour pouvoir lancer mon_prog nom_du_fichier.txt comme demandé sur le sujet)
```

## Format du fichier scrutin

- **En-tête** : noms des candidats séparés par des `;`
- **Bulletins** : classements séparés par des `>`, un par ligne

Exemple :
```
A;B;C;D;E
A>B>C
A>D>E>B
B>A>D>E
C>D>B>A>E
C>D>B
D
```

## Méthodes implémentées

- **Plurality** — Le candidat avec le plus de voix en première position gagne.
- **TRS** — Vote à deux tours. Majorité absolue au tour 1 sinon second tour entre les deux premiers.
- **IRV** — Élimination successive du candidat avec le moins de voix jusqu'à majorité absolue.

## Règle de départage

En cas d'égalité, le candidat dont le nom est le plus petit en ordre lexicographique est élu.
