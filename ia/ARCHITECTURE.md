# Architecture and Features Documentation for gdrive

This document provides a comprehensive technical overview of the `gdrive` application architecture, its complete Command Line Interface (CLI) feature set, internal subsystem design, and unit testing strategy.

---

## 1. Overview & System Context

`gdrive` is a command-line utility written in Rust for interacting with the Google Drive API (v3). It serves as the modern successor to `gdrive2`, built on an asynchronous runtime (`tokio`), HTTP streaming (`hyper`), and type-safe CLI parsing (`clap`).

### Key Characteristics
- **Binary-Only Crate**: Packaged as a standalone binary (`src/main.rs`).
- **Asynchronous I/O**: High-throughput file downloads, uploads, and recursive directory traversals powered by `tokio` and `futures`.
- **Resumable Uploads**: Robust chunked uploads with exponential backoff and custom retry logic.
- **Multi-Account Support**: Profile management with local token storage and cross-machine configuration export/import.

---

## 2. Complete CLI Feature Matrix & Command Reference

The command structure is organized into 5 primary functional domains plus root information commands:

```
gdrive
├── about
├── version
├── account
│   ├── add
│   ├── list
│   ├── current
│   ├── switch <NAME>
│   ├── remove <NAME>
│   ├── export <NAME>
│   └── import <FILE>
├── drives
│   └── list
├── files
│   ├── info <ID>
│   ├── list
│   ├── download <ID>
│   ├── upload [PATH]
│   ├── update <ID> [PATH]
│   ├── delete <ID>
│   ├── mkdir <NAME>
│   ├── rename <ID> <NAME>
│   ├── move <ID> <FOLDER_ID>
│   ├── copy <ID> <FOLDER_ID>
│   ├── import <PATH>
│   └── export <ID> <PATH>
└── permissions
    ├── share <ID>
    ├── list <ID>
    └── revoke <ID>
```

---

### A. Root Information Commands

| Command | Description |
| :--- | :--- |
| `gdrive about` | Prints information about the gdrive application and project homepage. |
| `gdrive version` | Prints the application version, build metadata, and runtime environment. |

---

### B. Account Management (`gdrive account`)

Credentials and tokens are isolated per account and persisted in `$HOME/.config/gdrive3/`.

| Subcommand | Arguments & Flags | Description |
| :--- | :--- | :--- |
| `account add` | *(Interactive)* | Adds a new Google account. Prompts for OAuth Client ID & Secret, opens browser authorization, and spins up a temporary local HTTP server (`http://localhost:8085`) to capture the OAuth redirect. |
| `account list` | *(None)* | Lists all configured accounts. |
| `account current` | *(None)* | Displays the currently active account name. |
| `account switch` | `<ACCOUNT_NAME>` | Switches the active profile to the specified account. |
| `account remove` | `<ACCOUNT_NAME>` | Deletes the account credentials and tokens from local storage. |
| `account export` | `<ACCOUNT_NAME>` | Exports account configuration into an encrypted/zipped archive for headless server migration. |
| `account import` | `<FILE_PATH>` | Imports an account archive created with `account export` (ideal for remote servers without a local web browser). |

---

### C. Shared Drives (`gdrive drives`)

| Subcommand | Arguments & Flags | Description |
| :--- | :--- | :--- |
| `drives list` | `--skip-header`<br>`--field-separator <SEP>` | Lists all Google Workspace Shared Drives (formerly Team Drives) accessible by the authenticated account. |

---

### D. File Operations (`gdrive files`)

The core domain handling Google Drive items, metadata, synchronization, and transfers.

#### 1. `files info`
Retrieves detailed metadata for a file or directory.
- Arguments: `<FILE_ID>`
- Flags: `--size-in-bytes` (display raw bytes instead of human-readable units).
- Output attributes: ID, Name, Size, MIME type, Created/Modified timestamps, MD5 checksum, Parent IDs, Shared status, Description, Web Links, Shortcut target details.

#### 2. `files list`
Lists files with flexible filtering, sorting, and formatting.
- Flags:
  - `--max <N>`: Maximum number of files to return (default: `30`).
  - `--query <QUERY>`: Custom Google Drive search parameter (e.g. `'name contains "report"'`).
  - `--order-by <ORDER>`: Sort order parameter (default: `folder,modifiedTime desc,name`).
  - `--parent <DIRECTORY_ID>`: Restrict listing to files inside a specific folder.
  - `--drive <DRIVE_ID>`: Scope listing to a specific Shared Drive.
  - `--full-name`: Prevent truncation of long file names in terminal tables.
  - `--skip-header`: Omit table column headers.
  - `--field-separator <SEP>`: Column separator (default: tab `\t`).

#### 3. `files download`
Downloads files or entire folder trees from Google Drive to the local machine.
- Arguments: `<FILE_ID>`
- Flags:
  - `--destination <PATH>`: Local target directory where files should be placed.
  - `--recursive`: Recursively download all files and subdirectories.
  - `--overwrite`: Overwrite existing local files.
  - `--follow-shortcuts`: Resolve and download the target of shortcut files.
  - `--stdout`: Stream file binary directly to standard output.

#### 4. `files upload`
Uploads local files or directory hierarchies to Google Drive.
- Arguments: `[FILE_PATH]` (optional, reads from stdin if omitted where supported).
- Flags:
  - `--parent <DIRECTORY_ID>`: Google Drive folder ID to upload into (accepts multiple parents).
  - `--recursive`: Recursively upload directories and reconstruct remote directory trees.
  - `--mime <MIME_TYPE>`: Override automatic MIME-type detection.
  - `--chunk-size <SIZE>`: Set upload chunk size in MB (power of 2 from `1` to `8192`, default: `32`).
  - `--print-chunk-info`: Output debug details for each chunk uploaded.
  - `--print-chunk-errors`: Output chunk upload failure and retry warnings.
  - `--print-only-id`: Print only the resulting file/folder ID on completion.

#### 5. `files update`
Replaces the content of an existing Google Drive file with a new revision (retaining file ID).
- Arguments: `<FILE_ID>`, `[FILE_PATH]`
- Flags: `--mime <MIME_TYPE>`, `--chunk-size <SIZE>`, `--print-chunk-info`, `--print-chunk-errors`.

#### 6. `files delete`
Permanently deletes a file or directory.
- Arguments: `<FILE_ID>`
- Flags: `--recursive` (required to delete non-empty directories and their children).

#### 7. `files mkdir`
Creates a new directory on Google Drive.
- Arguments: `<NAME>`
- Flags: `--parent <DIRECTORY_ID>`, `--print-only-id`.

#### 8. `files rename`
Renames an existing file or directory on Google Drive.
- Arguments: `<FILE_ID>`, `<NAME>`.

#### 9. `files move`
Moves a file or directory to a different remote folder.
- Arguments: `<FILE_ID>`, `<FOLDER_ID>`.

#### 10. `files copy`
Creates a copy of an existing file into a specified destination folder.
- Arguments: `<FILE_ID>`, `<FOLDER_ID>`.

#### 11. `files import`
Uploads and converts supported document formats (docx, xlsx, csv, pdf, html, etc.) directly into editable Google Docs, Sheets, or Slides.
- Arguments: `<FILE_PATH>`
- Flags: `--parent <DIRECTORY_ID>`, `--print-only-id`.

#### 12. `files export`
Exports native Google Docs/Sheets/Slides to local standard file formats (PDF, DOCX, XLSX, etc.) based on the target file extension.
- Arguments: `<FILE_ID>`, `<FILE_PATH>`
- Flags: `--overwrite`.

---

### E. Permission Management (`gdrive permissions`)

Controls Access Control Lists (ACLs) and sharing settings on files and folders.

| Subcommand | Arguments & Flags | Description |
| :--- | :--- | :--- |
| `permissions share` | `<FILE_ID>`<br>`--role <ROLE>`<br>`--type <TYPE>`<br>`--email <EMAIL>`<br>`--domain <DOMAIN>`<br>`--discoverable` | Grants permissions on a file:<br>• **Roles**: `owner`, `organizer`, `fileOrganizer`, `writer`, `commenter`, `reader`<br>• **Types**: `user`, `group`, `domain`, `anyone`<br>• `--email`: Required for `user` and `group`<br>• `--domain`: Required for `domain`<br>• `--discoverable`: Searchable file discovery for domain/anyone. |
| `permissions list` | `<FILE_ID>`<br>`--skip-header`<br>`--field-separator <SEP>` | Lists all active permissions (Role, Type, Email, Domain, ID) for a file. |
| `permissions revoke` | `<FILE_ID>`<br>`--id <PERMISSION_ID>`<br>`--all` | Revokes access permissions:<br>• `--all`: Removes all permissions except owner<br>• `--id`: Revokes a specific permission by ID<br>• Default: Revokes the public `'anyone'` permission. |

---

## 3. Internal Architecture & Subsystems

```
┌─────────────────────────────────────────────────────────────┐
│                    Command Line (clap)                      │
└──────────────────────────────┬──────────────────────────────┘
                               │
            ┌──────────────────┴──────────────────┐
            ▼                                     ▼
┌───────────────────────┐             ┌───────────────────────┐
│     Hub & Auth        │             │    Common Utilities   │
│ • Token storage       │             │ • Table / TabWriter   │
│ • OAuth server (8085) │             │ • Md5Writer           │
│ • google_drive3 client│             │ • FileInfo            │
└───────────┬───────────┘             └───────────┬───────────┘
            │                                     │
            └──────────────────┬──────────────────┘
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   File Transfer Pipeline                    │
│ • UploadDelegate (Chunk upload, Backoff retry)              │
│ • Local FileTree (IdGen pre-generation, fs traversal)       │
│ • Remote FileTreeDrive (async_recursion, folder mapping)    │
└─────────────────────────────────────────────────────────────┘
```

### A. Authentication & Hub Management (`src/hub.rs`, `src/common/hub_helper.rs`)
- Uses `google_drive3` with `yup-oauth2` authenticator.
- Local temporary web server on port `8085` captures the OAuth code upon user consent.
- Tokens are automatically refreshed on expiry and persisted in `$HOME/.config/gdrive3/`.

### B. Upload Pipeline & Resumable Chunks (`src/common/delegate.rs`)
- Implements `google_drive3::client::Delegate`.
- Uploads large files in discrete binary blocks configured via `ChunkSize` ($2^{20}$ to $2^{33}$ bytes).
- Wrapped in `exponential_backoff::Backoff` to handle cloud network throttling (`429 Too Many Requests`, `500`, `502`, `503`).
- Tracks and prints chunk transfer info and failure warnings in real-time.

### C. Tree Abstractions & Recursive Synchronization
- **Local Filesystem (`src/common/file_tree.rs`)**:
  - Recursively maps directories and files.
  - Leverages `IdGen` (`src/common/id_gen.rs`) to batch-request up to 1000 Google Drive IDs in advance, ensuring deterministic folder creation order before file streaming begins.
- **Drive Remote Hierarchy (`src/common/file_tree_drive.rs`)**:
  - Employs `async_recursion` to traverse nested Google Drive folders into flat, sorted queues.
  - Computes path depths and ancestor counts for consistent hierarchical reconstruction.

### D. Checksums & Integrity (`src/common/md5_writer.rs`)
- Wraps `std::io::Write` to compute MD5 digests during download streaming.
- Compares computed hashes against Google Drive's `md5Checksum` metadata attribute to guarantee file integrity.

### E. Output Formatting (`src/common/table.rs`)
- Universal tabular output formatter backed by `tabwriter::TabWriter`.
- Handles dynamic column width calculations, padding, header suppression, and custom field delimiters.

---

## 4. Unit Testing Strategy

### A. Rationale
Because `gdrive` is compiled as an executable binary without an exported library (`src/lib.rs`), unit tests are organized directly within **in-module test modules** (`#[cfg(test)] mod tests`):
- Direct access to internal data structures and non-public methods.
- Tests compile only during `cargo test` and introduce zero runtime overhead or binary bloat.
- Eliminates any external dependency on active Google accounts, OAuth secrets, or live network endpoints.

### B. Test Suite Coverage

| Component | File | Tested Capabilities |
| :--- | :--- | :--- |
| **CLI Validation** | `src/main.rs` | • Full `clap` command graph verification (`debug_assert()`)<br>• Subcommand argument parsing (`version`, `about`, `list`, `upload`, `download`, `mkdir`)<br>• Error detection on invalid commands or missing mandatory positional arguments |
| **Chunk Sizes** | `src/common/delegate.rs` | • Valid string parsing for powers of 2 from 1 to 8192<br>• Byte calculation accuracy ($2^{20}$ to $2^{33}$)<br>• String serialization (`Display`)<br>• Strict rejection of invalid, zero, or non-power-of-two inputs |
| **Permissions** | `src/common/permission.rs` | • String parsing and display for all valid `Role` and `Type` values<br>• Rejection of unknown roles/types<br>• Business logic rules: `requires_email()`, `requires_domain()`, and `supports_file_discovery()` |
| **Drive Queries** | `src/files/list.rs` | • Query string parsing (`None` vs `Custom`)<br>• Formatted query strings matching Google Drive API search parameters<br>• Sort order parsing and default format strings<br>• Unicode middle truncation (`truncate_middle`) |
| **Table Formatter**| `src/common/table.rs` | • Header rendering with tab padding<br>• Header suppression (`skip_header = true`)<br>• Delimiter customization |

### C. Execution
Run the full test suite locally:
```bash
cargo test
```
All 28 tests run deterministically in $\le 0.02$ seconds.
