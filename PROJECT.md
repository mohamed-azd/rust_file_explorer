# PROJECT.md - Explorateur de Fichiers TUI en Rust

## 1. Vue d'ensemble
Ce projet est un explorateur de fichiers en ligne de commande (TUI - Text User Interface) écrit en Rust. Il s'inspire d'outils comme `ranger`. L'objectif est de fournir une navigation rapide au clavier avec une prévisualisation immédiate du contenu des fichiers (texte coloré ou images ASCII) dans un panneau latéral.

**Plateforme cible :** Windows (compatible Cross-platform).

## 2. Stack Technique
Le projet utilise les crates suivantes :

* **`ratatui`** : Moteur de rendu TUI pour gérer le layout (panneaux) et l'affichage des widgets.
* **`crossterm`** : Gestion des événements (clavier/souris), manipulation brute du terminal et compatibilité Windows.
* **`syntect`** : Moteur de coloration syntaxique pour la prévisualisation des fichiers de code.
* **`image`** : Traitement d'images pour la conversion en ASCII art dans la prévisualisation.

## 3. Architecture & Design
L'application suit une boucle d'exécution standard pour les TUI :
1.  **Poll Events :** Écoute des entrées utilisateur (non-bloquant).
2.  **Update State :** Mise à jour de l'état (changement de dossier, sélection de fichier).
3.  **Draw :** Rendu de l'interface complète à chaque changement.

### Layout de l'interface
L'écran est divisé en 3 colonnes verticales (via `ratatui` Constraints) :
1.  **Gauche (Parent) :** Liste du dossier parent (contexte). *[Optionnel au début]*
2.  **Milieu (Actif) :** Liste des fichiers du dossier courant avec surbrillance de la sélection.
3.  **Droite (Preview) :** Contenu du fichier sélectionné.

## 4. Fonctionnalités (Roadmap)

### Phase 1 : MVP (Navigation)
- [ ] Lister les fichiers et dossiers du répertoire courant (`std::fs`).
- [ ] Gérer la navigation verticale (Haut/Bas) pour changer la sélection.
- [ ] Gérer la navigation hiérarchique :
    -   `Entrée` : Ouvrir le dossier sélectionné.
    -   `Retour` / `Backspace` : Remonter au dossier parent.
- [ ] Gestion robuste des chemins sous Windows (`PathBuf`, séparateurs).

### Phase 2 : Prévisualisation (Preview Pane)
- [ ] Détection du type de fichier (extension/mime).
- [ ] **Texte :** Afficher les N premières lignes.
- [ ] **Code :** Appliquer la coloration syntaxique via `syntect`.
- [ ] **Image :** Redimensionner et convertir en ASCII via `image`.
- [ ] **Binaire/Autre :** Afficher les métadonnées (taille, date) si illisible.

### Phase 3 : UX & Polish
- [ ] Gestion des erreurs (Permission denied) sans crash.
- [ ] Scroll dans la fenêtre de prévisualisation.

## 5. Contraintes Spécifiques
* Utilisation stricte de `PathBuf` pour la compatibilité des chemins Windows.
* Gestion des disques multiples (C:\, D:\) pour la racine.
* Performance critique : La boucle de rendu ne doit pas laguer lors du parcours de gros dossiers.