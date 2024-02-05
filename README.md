# Mood-Enhancing-Fragrance-Canister

## Overview

The Fragrance Canister is a sample Internet Computer (IC) canister that manages fragrances. It provides functionality for adding, updating, deleting, and querying fragrances. Fragrances have attributes such as name, description, mood-enhancing properties, creation time, and optional update time.

## Table of Contents

1. [Structs and Traits](#structs-and-traits)
2. [Thread-Local Storage](#thread-local-storage)
3. [Public Queries](#public-queries)
4. [Public Updates](#public-updates)
5. [Error Handling](#error-handling)

## Structs and Traits

### `Fragrance` Struct

- **Attributes:**
  - `id`: u64 - Unique identifier for each fragrance.
  - `name`: String - Name of the fragrance.
  - `description`: String - Description of the fragrance.
  - `mood_enhancing_properties`: Vec<String> - List of mood-enhancing properties associated with the fragrance.
  - `created_at`: u64 - Timestamp indicating when the fragrance was created.
  - `updated_at`: Option<u64> - Optional timestamp indicating when the fragrance was last updated.

### `FragrancePayload` Struct

- **Attributes:**
  - `name`: String - Name of the fragrance.
  - `description`: String - Description of the fragrance.
  - `mood_enhancing_properties`: Vec<String> - List of mood-enhancing properties associated with the fragrance.

### Traits

- `Storable`: Provides serialization and deserialization methods for the `Fragrance` struct.
- `BoundedStorable`: Defines constants related to storage size for the `Fragrance` struct.

## Thread-Local Storage

- `MEMORY_MANAGER`: Manages virtual memory using the `MemoryManager` from `ic_stable_structures`.
- `ID_COUNTER`: Provides a unique identifier for each fragrance.
- `FRAGRANCE_STORAGE`: Stores fragrances using a stable BTreeMap.

## Public Queries

1. **`get_fragrance(id: u64) -> Result<Fragrance, Error>`:**
   - Retrieves a fragrance by its unique identifier.

2. **`list_fragrances() -> Vec<Fragrance>`:**
   - Retrieves a list of all fragrances.

3. **`search_fragrance_names(keyword: String) -> Result<Vec<String>, Error>`:**
   - Searches for fragrances by name or description and returns their names.

4. **`get_recommendations(mood_keyword: String) -> Result<Vec<Fragrance>, Error>`:**
   - Retrieves fragrances with mood-enhancing properties related to the specified keyword.

5. **`filter_fragrances_by_mood(keyword: String) -> Result<Vec<Fragrance>, Error>`:**
   - Filters fragrances by mood-enhancing properties related to the specified keyword.

6. **`sort_fragrances_by_creation_date() -> Vec<Fragrance>`:**
   - Sorts fragrances by creation date in descending order.

## Public Updates

1. **`add_fragrance(fragrance: FragrancePayload) -> Option<Fragrance>`:**
   - Adds a new fragrance to the storage.

2. **`update_fragrance(id: u64, payload: FragrancePayload) -> Result<Fragrance, Error>`:**
   - Updates an existing fragrance with new data.

3. **`delete_fragrance(id: u64) -> Result<Fragrance, Error>`:**
   - Deletes a fragrance from the storage.

## Error Handling

- **`Error` Enum:**
  - `NotFound { msg: String }`: Indicates that a fragrance or certain criteria were not found.

---
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
