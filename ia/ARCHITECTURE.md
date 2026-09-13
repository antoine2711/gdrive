# Architecture des tests unitaires pour gdrive

Ce document décrit l'architecture et la stratégie retenue pour la mise en place des tests unitaires dans le projet `gdrive`.

---

## 1. Contexte et contraintes

- **Nature du projet** : `gdrive` est une application console (binaire Rust pur) définie dans `src/main.rs`, sans bibliothèque séparée (`src/lib.rs`).
- **Dépendances externes** : La plupart des commandes réelles effectuent des appels vers l'API Google Drive via `google_drive3`, nécessitant des identifiants OAuth et un accès réseau.
- **Objectif des tests unitaires** : Les tests unitaires doivent s'exécuter **hors ligne**, de manière **déterministe**, **instantanée** et sans avoir besoin de compte Google configuré.

---

## 2. Stratégie d'implantation

### A. Tests unitaires intégrés aux modules (`#[cfg(test)]`)
Comme les structures et enums internes ne sont pas exportées via une bibliothèque publique, les tests unitaires sont placés directement dans des sous-modules `#[cfg(test)] mod tests` au sein des fichiers sources respectifs :
- Accès direct aux types privés et semi-privés.
- Compilation des tests uniquement lors de l'exécution de `cargo test` (aucun impact sur la taille du binaire final de production).

### B. Périmètre testable sans réseau
La logique métier pure, le parsing et le formatage sont isolés des appels API :

```
┌─────────────────────────────────────────────────────────────┐
│                       gdrive CLI                            │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ┌───────────────────────┼───────────────────────┐
       ▼                       ▼                       ▼
┌──────────────┐       ┌──────────────┐       ┌──────────────┐
│     CLI      │       │    Logic     │       │  Formatting  │
│ (src/main.rs)│       │(common/files)│       │ (table.rs)   │
└──────┬───────┘       └──────┬───────┘       └──────┬───────┘
       │                      │                      │
       ▼                      ▼                      ▼
• Clap validation      • ChunkSize            • Header rendering
• Flag parsing         • Permission rules     • Row alignment
• Subcommand routing   • Query generation     • Separators
```

---

## 3. Composants couverts

### 1. Interface CLI (`src/main.rs`)
- **Validation clap (`Cli::command().debug_assert()`)** : Vérification statique et dynamique par `clap` qu'il n'y a aucun conflit d'argument, aucun type manquant et que les arbres de sous-commandes sont valides.
- **Parsing des commandes** : Utilisation de `Cli::try_parse_from` pour valider que les arguments obligatoires et optionnels sont correctement routés vers les variantes correspondantes de l'enum `Command`.

### 2. Gestion des blocs (`src/common/delegate.rs`)
- **`ChunkSize`** :
  - Parsing (`FromStr`) : puissances de 2 autorisées de 1 à 8192 MB.
  - Conversion binaire (`in_bytes()`) : calcul exact en octets ($2^{20}$ à $2^{33}$).
  - Affichage (`Display`).
  - Détection des erreurs : rejet strict de toute valeur non puissance de 2 ou hors limites.

### 3. Permissions (`src/common/permission.rs`)
- **`Role`** :
  - Conversion chaîne $\leftrightarrow$ enum (`owner`, `organizer`, `fileOrganizer`, `writer`, `commenter`, `reader`).
  - Rejet des rôles inconnus.
- **`Type`** :
  - Conversion chaîne $\leftrightarrow$ enum (`user`, `group`, `domain`, `anyone`).
  - Règles métier : validation de `requires_email()`, `requires_domain()` et `supports_file_discovery()`.

### 4. Requêtes et tris de fichiers (`src/files/list.rs`)
- **`ListQuery`** :
  - Chaîne vide convertie en `ListQuery::None`.
  - Chaîne non vide convertie en `ListQuery::Custom`.
  - Formatage `Display` conforme à la syntaxe de l'API Drive (`'root' in parents and trashed = false`, `'{id}' in parents...`).
- **`ListSortOrder`** :
  - Formatage du tri par défaut (`folder,modifiedTime desc,name`).
  - Tri personnalisé (`Custom`) et rejet des chaînes vides.

### 5. Formatage tabulaire (`src/common/table.rs`)
- **`Table::write`** :
  - Écriture dans un buffer en mémoire (`Vec<u8>`) avec `TabWriter`.
  - Prise en compte du drapeau `skip_header`.
  - Alignement des colonnes et gestion de séparateurs personnalisés.

---

## 4. Exécution et intégration

- **Lancement local** :
  ```bash
  cargo test
  ```
- **Évolution future (CI)** :
  Ces tests peuvent être intégrés dans un workflow GitHub Actions (`.github/workflows/test.yaml`) pour s'exécuter automatiquement à chaque `push` ou `pull_request`.

