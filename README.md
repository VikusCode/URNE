# Projet URNE

## Utilisation

```
cargo run -- chemin/vers/nom_du_fichier.txt

OU 

.\target\debug\mon_prog.exe chemin/vers/nom_du_fichier.txt


(A LA FIN FAIRE: cargo install --path pour pouvoir lancer mon_prog nom_du_fichier.txt comme demandé sur le sujet)


(AJOUTER --timing à la fin de la ligne pour lancer le mode avec timing activés)
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

- **Plurality** — On regarde qui est classé 1er sur chaque bulletin. Celui qui apparaît le plus souvent en 1er gagne.
- **TRS** — Tour 1 : on compte les 1ers choix. Si quelqu'un a plus de la moitié des voix, il gagne. Sinon, on garde les deux meilleurs et on refait un vote entre eux.
- **IRV** — On compte les 1ers choix. Si personne n'a plus de 50%, on élimine le candidat avec le moins de voix (un seul par tour). Les bulletins qui votaient pour lui passent au candidat suivant encore en lice. On répète jusqu'à ce que quelqu'un dépasse 50%.
- **Borda** — Chaque position dans le classement donne des points : le 1er choix reçoit le max de points, le 2e un peu moins, etc. Celui qui a le plus de points au total gagne.
- **Bucklin** — Tour 1 : on compte les 1ers choix. Si personne n'a >50%, tour 2 : on ajoute les 2es choix aux scores. Tour 3 : on ajoute les 3es, etc. Dès que quelqu'un dépasse 50%, il gagne.
- **Copeland** — On fait tous les duels possibles entre candidats. Victoire = 2 pts, nul = 1 pt, défaite = 0 pt. Le plus de points gagne.
- **Copeland+Borda** — Même chose que Copeland, mais en cas d'égalité au score Copeland, on départage avec le score Borda (points par position dans le classement).
- **Smith+IRV** — On calcule l'ensemble de Smith (le plus petit groupe de candidats qui battent tous les autres). Puis on applique IRV à l'intérieur de ce groupe : on élimine un par un celui avec le moins de 1ers choix jusqu'à trouver un gagnant de Condorcet.

## Règle de départage

En cas d'égalité, le candidat dont le nom est le plus petit en ordre lexicographique est élu.
