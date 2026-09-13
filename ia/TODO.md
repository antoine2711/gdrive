# TODO - Implantation des tests unitaires

Ce document liste l'ensemble des tâches à accomplir pour implanter la première suite de tests unitaires sur le projet `gdrive`.

---

## 1. Composants communs (`src/common/`)

### `src/common/delegate.rs` (`ChunkSize`)
- [x] Tester le parsing valide de toutes les puissances de 2 (`"1"`, `"2"`, `"4"`, `"8"`, `"16"`, `"32"`, `"64"`, `"128"`, `"256"`, `"512"`, `"1024"`, `"2048"`, `"4096"`, `"8192"`).
- [x] Tester le calcul en octets `in_bytes()` pour chaque taille (ex. $1\text{ MB} = 1048576$, $32\text{ MB} = 33554432$).
- [x] Tester l'affichage `Display` (doit retourner la valeur sous forme de chaîne numérique).
- [x] Tester le rejet des entrées invalides (`"0"`, `"3"`, `"5"`, `"100"`, `"8193"`, `"abc"`, `""`).

### `src/common/permission.rs` (`Role` & `Type`)
- [x] Tester le parsing et l'affichage de `Role` pour tous les rôles valides (`owner`, `organizer`, `fileOrganizer`, `writer`, `commenter`, `reader`).
- [x] Tester le rejet d'un rôle invalide (ex. `"admin"`, `""`).
- [x] Tester le parsing et l'affichage de `Type` (`user`, `group`, `domain`, `anyone`).
- [x] Tester le rejet d'un type invalide.
- [x] Tester `requires_email()` : `true` pour `User` et `Group`, `false` pour les autres.
- [x] Tester `requires_domain()` : `true` pour `Domain`, `false` pour les autres.
- [x] Tester `supports_file_discovery()` : `true` pour `Domain` et `Anyone`, `false` pour les autres.

### `src/common/table.rs` (`Table`)
- [x] Tester l'écriture d'un tableau avec en-tête par défaut.
- [x] Tester l'écriture d'un tableau avec `skip_header: true`.
- [x] Tester l'écriture d'un tableau avec un séparateur personnalisé (ex. `","` ou `" | "`).

---

## 2. Gestion des fichiers (`src/files/`)

### `src/files/list.rs` (`ListQuery` & `ListSortOrder`)
- [x] Tester `ListQuery::from_str` :
  - Chaîne vide `""` $\rightarrow$ `ListQuery::None`.
  - Chaîne `"name = 'test'"` $\rightarrow$ `ListQuery::Custom("name = 'test'")`.
- [x] Tester `Display` pour les différentes variantes de `ListQuery` :
  - `RootNotTrashed` $\rightarrow$ `"'root' in parents and trashed = false"`.
  - `FilesInFolder { folder_id: "xyz" }` $\rightarrow$ `"'xyz' in parents and trashed = false"`.
  - `FilesOnDrive { drive_id: "drive123" }` $\rightarrow$ `"'drive123' in parents and trashed = false"`.
  - `None` $\rightarrow$ `""`.
- [x] Tester `ListSortOrder::from_str` :
  - Chaîne vide $\rightarrow$ erreur.
  - Chaîne valide $\rightarrow$ `ListSortOrder::Custom(...)`.
- [x] Tester `Display` pour `ListSortOrder::FolderModifiedName` (`"folder,modifiedTime desc,name"`).
- [x] Tester `truncate_middle` pour la troncature de noms longs.

---

## 3. Interface CLI (`src/main.rs`)

### Validation Clap
- [x] Tester `Cli::command().debug_assert()` pour vérifier la validité de la grammaire CLI clap.
- [x] Tester le parsing de commandes représentatives :
  - [x] `gdrive version`
  - [x] `gdrive about`
  - [x] `gdrive files list` (options par défaut)
  - [x] `gdrive files download FILE_ID`
  - [x] `gdrive files upload file.txt --recursive --parent folder_abc`
  - [x] `gdrive files mkdir new_folder --print-only-id`
- [x] Tester le rejet de commandes invalides ou d'arguments manquants.

---

## 4. Vérification et validation

- [x] Lancer la compilation et l'exécution de l'ensemble des tests via `cargo test`.
- [x] Vérifier que 100% des tests passent sans avertissement ni échec (28 tests passés avec succès).
- [ ] Committer les modifications sur la branche `add-unit-tests`.
