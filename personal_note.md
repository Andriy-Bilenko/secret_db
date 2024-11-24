# verbal description
the storage, like contact table in SQL or CRM
functions: add, remove, modify person, get table
person: name(string), what do you have in common (string), how did you meed(string), other info, like birthdays(string), closeness(int, 1 - most close, 8 - least, -1 - a little negative, -8 - the most negative)


I'll be updating only 1 big field in a person and adding new people, for that I will use Keymap of person structures instead of several maps as fields of structures with O(1) complexity for all those operations

pub const CONTACTS_COUNT: Item<u32> = Item::new("contacts_count"); // used for indexes in CONTACTS_LIST, new index for new entry = contacts_count

pub const CONTACT_LIST: Keymap<u32, Contact, Bincode2> = KeymapBuilder::new(b"secrets").build(); // u32 is unique index
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Contact {
    pub name: String,
    pub commons: String,
    pub firstMeet: String,
    pub other: String,
	pub closeness: int8,
}

additinally I'll be not the only one to use the contract, so I'll have a map of those contact lists for each individual user their personal database noone would access

no maps of maps, every user would deploy a personal smart contract that is his map

and to keep track of all those we'd need another storage (a relayer), that would store keymap (user addr -> their deployed contract to launch)
smart contract for a user would autoadd (by triggering the relayer from Rust) itself to relayer's storage

I can restrict other users to query who's in the map if I change signature of pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> by adding info: MessageInfo, which contains address of a sender


so it's 2 contracts:
1. relayer:
	contains keymap<user address as string + number of personal table, their deployed personal contract>
	functions:
		1. queryMyContract() - returns the contract address of personal storage, and an error if it is not there (then it should be created). does return only for the user that queries it and not for other users.
		2. executeAddDeployedContract() - triggered by deployed contract, adds its own address to relayers map, the relayer triggers back a "are you ours" function of a personal smart contract to check if it is a smart contract, if it is deployed, and if it is part of the system (if it is not - returns an error)

2. personal_table
	functions:
		1. getTableNum() - get the number of a table, should be executed only by a relayer for a check
		2. getTableOwner() - get the addr of owner, should be executed only by a relayer for a check
		3. addOrUpdateCell() - adds or updates the cell
		4. removeCell() - removes cell
		5. getCell() - get certain cell at x and y
		6. getAllCells() - returns a Vec of all cells

if I wanted a database on a blockchain I'd want a hashmap of strings, where each key is like "row3-col4"
and the frontend would be the one managing which row is responsible for each field in the Person structure

struct Cell{ // with implemented features like eq, hash
	int x,
	int y,
}

pub const TABLE: Keymap<Cell, string, Bincode2> = KeymapBuilder::new(b"secrets").build();

and
pub const TABLE_INFO: Item<TableInfo> = Item::new("contacts_count");

# results of brainstorm and techy details

## relayer contract
pub const TABLES_COUNT: Item<u32> = Item::new("contacts_count"); // used for indexes in TableInfo in TABLE_ORGANISER, new index for new entry = tables_count

pub const TABLE_ORGANISER: Keymap<TableInfo, Addr(of deployed contract), Bincode2> = KeymapBuilder::new(b"secrets").build();
struct TableInfo{
	Addr owner, // owner of table
	String tableName, // custom name
	uint tableIndex,
}

fn getMyContractAddrs() -> Vec<Addr(of personal storage contract for signer.address)> or error if None (should be created then)
-- does return only for the user that queries it and not for other users

fn AddDeployedContract(Addr owner, String tableName) -> OK/ERR -
-- triggered by deployed contract, adds its own address (deployed contract addr) to relayers map (TABLE_ORGANISER),
-- the relayer triggers back a "are you ours" function of a personal smart contract to check if it is a smart contract, if it is deployed, and if it is part of the system (if it is not - returns an error)

## table contract
pub const TABLE_INFO: Item<TableInfo> = Item::new("contacts_count");
struct TableInfo{
	Addr owner, // owner of table
	String tableName, // custom name
	uint tableIndex,
}

pub const TABLE: Keymap<CellCoords, string, Bincode2> = KeymapBuilder::new(b"secrets").build();
struct CellCoords{ // with implemented features like eq, hash
	int x,
	int y,
}

fn getTableInfo() -> TableInfo , used ONLY by relayer contract to check that table contract exists, in correct format and is a contract and not a user

fn addOrUpdateCells(Vec<Cells>) -> (), triggered by user to create or update the Cell in the TABLE hashmap

fn removeCells(Vec<CellCoords>) -> (), triggered by user to remove the cell from hashmap

fn getCellStr(CellCoords xy) -> String, returns a value at a certain string, empty string "" if non-existent

fn getAllCells() -> Vec<Cell>, returns all cells as a vector of structs
struct Cell{
	CellCoords xy,
	String data,
}

# TABLES DB dApp
1. connect wallet button
	right after the connection query the relayer for my table
	if there's no table:
		put a field "name" and a big button "create table" that will deploy a personal table contract with a set name (no way to delete the table)
	if there is at least 1 table for a user:
		query the table for all cells
		put them into table *x*
		to the left there's a pane with each available table buttons, and a button "create another table"
		down there'd be a button "save changes" that writes all of them as a signle transaction
		can also delete certain cell or a row or a column and the colors of cells will change if the cell doesnt exist or was changed and not saved

# CONTACT LIST dApp
same as TABLES DB dApp but without a side pane and possibilities to create other tables and name of the only table is defaulted to "contact list"
 and fields are tweaked to support the person struct for each row:
name(string), what do you have in common (string), how did you meet(string), other info, like birthdays(string), closeness(int, 1 - most close, 8 - least, -1 - a little negative, -8 - the most negative)

