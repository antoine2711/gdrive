# TODO - Unit Test Suite Implementation

This document tracks all tasks for implementing and maintaining unit tests in `gdrive`.

---

## 1. Common Components (`src/common/`)

### `src/common/delegate.rs` (`ChunkSize`)
- [x] Test valid parsing for all powers of 2 (`"1"`, `"2"`, `"4"`, `"8"`, `"16"`, `"32"`, `"64"`, `"128"`, `"256"`, `"512"`, `"1024"`, `"2048"`, `"4096"`, `"8192"`).
- [x] Test byte calculation `in_bytes()` for each size (e.g., $1\text{ MB} = 1048576$, $32\text{ MB} = 33554432$).
- [x] Test `Display` output (must format as numeric string).
- [x] Test rejection of invalid inputs (`"0"`, `"3"`, `"5"`, `"100"`, `"8193"`, `"abc"`, `""`).

### `src/common/permission.rs` (`Role` & `Type`)
- [x] Test parsing and display of `Role` for all valid roles (`owner`, `organizer`, `fileOrganizer`, `writer`, `commenter`, `reader`).
- [x] Test rejection of invalid roles (e.g., `"admin"`, `""`).
- [x] Test parsing and display of `Type` (`user`, `group`, `domain`, `anyone`).
- [x] Test rejection of invalid types.
- [x] Test `requires_email()`: `true` for `User` and `Group`, `false` for others.
- [x] Test `requires_domain()`: `true` for `Domain`, `false` for others.
- [x] Test `supports_file_discovery()`: `true` for `Domain` and `Anyone`, `false` for others.

### `src/common/table.rs` (`Table`)
- [x] Test writing a table with default header.
- [x] Test writing a table with `skip_header: true`.
- [x] Test writing a table with a custom separator (e.g., `","` or `" | "`).

---

## 2. File Management (`src/files/`)

### `src/files/list.rs` (`ListQuery` & `ListSortOrder`)
- [x] Test `ListQuery::from_str`:
  - Empty string `""` $\rightarrow$ `ListQuery::None`.
  - String `"name = 'test'"` $\rightarrow$ `ListQuery::Custom("name = 'test'")`.
- [x] Test `Display` for `ListQuery` variants:
  - `RootNotTrashed` $\rightarrow$ `"'root' in parents and trashed = false"`.
  - `FilesInFolder { folder_id: "xyz" }` $\rightarrow$ `"'xyz' in parents and trashed = false"`.
  - `FilesOnDrive { drive_id: "drive123" }` $\rightarrow$ `"'drive123' in parents and trashed = false"`.
  - `None` $\rightarrow$ `""`.
- [x] Test `ListSortOrder::from_str`:
  - Empty string $\rightarrow$ error.
  - Valid string $\rightarrow$ `ListSortOrder::Custom(...)`.
- [x] Test `Display` for `ListSortOrder::FolderModifiedName` (`"folder,modifiedTime desc,name"`).
- [x] Test `truncate_middle` for long string truncation with `…`.

---

## 3. CLI Interface (`src/main.rs`)

### Clap Validation
- [x] Test `Cli::command().debug_assert()` to verify clap CLI grammar and consistency.
- [x] Test parsing of representative commands:
  - [x] `gdrive version`
  - [x] `gdrive about`
  - [x] `gdrive files list` (default options)
  - [x] `gdrive files download FILE_ID`
  - [x] `gdrive files upload file.txt --recursive --parent folder_abc`
  - [x] `gdrive files mkdir new_folder --print-only-id`
- [x] Test rejection of unknown commands or missing required arguments.

---

## 4. Verification and Validation

- [x] Run test compilation and execution via `cargo test`.
- [x] Verify that 100% of tests pass without warnings or failures (28 passed).
- [ ] Commit changes on the `add-unit-tests` branch.
