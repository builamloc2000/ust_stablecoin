use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    near_bindgen,
    Promise,
    AccountId, 
    collections::{LookupMap},
};
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
	total_supply: u128,
	total_market: u128,
}
impl Default for Account {
    fn default() -> Self {
        Account {
            total_supply: 0,
            total_market: 0,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenStorage {
    pub balances: LookupMap<AccountId, Account>,
   
}

impl Default for TokenStorage {
    fn default() -> Self {
        TokenStorage {
            balances: LookupMap::new(b"b".to_vec()),
           
        }
    }
}

#[near_bindgen]
#[derive(Default,BorshDeserialize, BorshSerialize)]
pub struct Token {
    // token name
    pub name: String,
    // token symbol
    pub symbol: String,
    // token decimals
    pub decimals: u8,
    // total supply of tokens
    pub account: Account,
    
    // storage for account balances and allowances
    pub storage: TokenStorage,
}

#[near_bindgen]
impl Token {
    // Initializes the contract with the given name, symbol, decimals and total supply
    #[init]
    pub fn new(name: String, symbol: String, decimals: u8, initial_balance: u128) -> Self {
        let mut storage = TokenStorage::default();
        // this is the id calling this function
        let account_id = env::predecessor_account_id();
        let account = Account {
        	total_supply: initial_balance,
        	total_market: initial_balance,
    	};
        storage.balances.insert(&account_id, &account);
        Self {
            name,
            symbol,
            decimals,
            account,
            storage,
        }
    }
    // Get the total supply of tokens for the given account
    pub fn balance_of(&self, account_id: AccountId) -> Option<u128> {
        self.storage.balances.get(&account_id).map(|account| account.total_supply)
    }

    // Get the total market value of tokens for the given account
    pub fn total_market_of(&self, account_id: AccountId) -> Option<u128> {
        self.storage.balances.get(&account_id).map(|account| account.total_market)
    }
    
     pub fn mint_ust(&mut self, account_id: AccountId, amount: u128) {
        let mut account = self.storage.balances.get(&account_id).unwrap_or(Account::default());
        account.total_supply += amount;
        account.total_market += amount;
        self.storage.balances.insert(&account_id, &account);
        self.account.total_supply += amount;
        self.account.total_market += amount;
    }
     pub fn burn_ust(&mut self, account_id: AccountId, amount: u128) {
        let mut account = self.storage.balances.get(&account_id).unwrap_or(Account::default());
        account.total_supply -= amount;
        account.total_market -= amount;
        self.storage.balances.insert(&account_id, &account);
        self.account.total_supply -= amount;
        self.account.total_market -= amount;
    }
    
    
      
}


