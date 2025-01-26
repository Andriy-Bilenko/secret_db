# Contacts Table dApp

This project is a decentralized application that allows users to manage contact lists, storing personal information such as names, common interests, meeting history, and closeness. The data is stored on the Secret Network blockchain for privacy and security.

## Structure and Features

### Relayer Contract

- **Relayer contract** manages the storage of user-specific personal tables.
- It keeps track of all deployed personal smart contracts via a `Keymap`.
- Functions:
    - `getMyContractAddrs()`: Returns the address of the user's personal table contract.
    - `AddDeployedContract()`: Registers the contract address of the user's personal storage.

### Personal Table Contract

- Each user has a personal table contract that holds their contact list.
- The table is represented by `Keymap<CellCoords, string, Bincode2>`, where each cell stores a piece of information about a person.
- Functions:
    - `getTableInfo()`: Returns information about the table, such as its owner and name.
    - `addOrUpdateCells()`: Adds or updates rows in the table.
    - `removeCells()`: Removes rows from the table.
    - `getCellStr()`: Retrieves data from a specific cell.
    - `getAllCells()`: Returns all cells in the table.

### Contact List Structure

- Each contact is represented by the following structure:
```rust
pub struct Contact {
    pub name: String,
    pub commons: String,
    pub firstMeet: String,
    pub other: String,
    pub closeness: i8,
}
```
- Closeness is ranked from 1 (most close) to -8 (most negative).

### Frontend Features

- **Connect Wallet**: Users can connect their wallet to access their contact list and personal tables.
- **Create Personal Table**: Users can create a personal table to store contact data.
- **Add/Update/Delete Contacts**: Users can add, update, or delete contact entries in their list.
- **Save Changes**: Changes are saved in a single transaction to the blockchain.
- **Closeness & Status**: Each contact has a "closeness" field and other attributes like meeting history and personal notes.

## Setup

### Initial Setup

1. Clone the repository and move `.env.example` to `.env`, then populate with the necessary environment variables (mnemonics, etc.).
2. Install dependencies:
```bash
npm install
cd scrt_lottery_react_app/
npm install
```
3. If using localnet, start a local instance of the Secret Network:
```bash
docker run -it -p 9091:9091 -p 26657:26657 -p 1317:1317 -p 5000:5000 --name localsecret -v ~/.secretd:/root/.secretd ghcr.io/scrtlabs/localsecret:v1.15.0-beta.7
```
### Contract Setup

1. **Build the contract**:
```bash
make build
```
2. **Deploy and store the contract**:
```bash
make store
make instantiate
```
### Interact with the Contract

1. Query and interact with the contact list via the Secret CLI or frontend.
2. Execute functions like `addOrUpdateCells()`, `removeCells()`, and `getCellStr()` to manipulate contact data.

### Frontend

- Run the frontend:
```bash
cd scrt_lottery_react_app
npm start
```
## Database Operations

- The personal contact list is stored in a `Keymap` with `CellCoords` (x, y) as the key. The frontend will map the fields from the `Contact` struct to the table rows.
- **Contacts Table**: A simple list of contacts, with customizable columns for `name`, `commons`, `firstMeet`, `other`, and `closeness`.

### Relayer Contract

- **Keymap**: Stores the mapping between user addresses and their personal table contracts.
```rust
pub const TABLE_ORGANISER: Keymap<TableInfo, Addr, Bincode2> = KeymapBuilder::new(b"secrets").build();
```
### Personal Table Contract

- **Keymap**: Stores contact data in the form of `CellCoords` and string values.
```rust
pub const TABLE: Keymap<CellCoords, string, Bincode2> = KeymapBuilder::new(b"secrets").build();
```
