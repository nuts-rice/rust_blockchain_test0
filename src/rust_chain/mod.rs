extern crate blake2;

use std::vec::Vec;
use std::collections::HashMap;
use std::convert::Into;
use std::time::SystemTime;
use blake2::{Blake2b, Digest};
use std::string::String;
use std::convert::From;

#[derive(Debug, Clone)]

pub struct Blockchain {
    //Stores all the blocks which are in the blockchain
    pub blocks: Vec<Block>,

    //Lookup from AccountID to account
    pub accounts: HashMap<String, Account>

    pub pending_transactions: Vec<Transaction>
}

//Repersents current state of blockchain after all the blocks are executed

trait WorldState {
    //bring us all regestered user ids

    fn get_user_ids(&self) -> Vec<String>;

    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>
}

#[derive(Clone, Debug)]
pub struct Transaction {

    nonce: u128,

    from: String,

    created_at: SystemTIme,

    pub(crate) record: TransactionData,

    signature: Option<String>,

}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionData {

    CreateUserAccount(String),

    ChangeStoreValue{key: String, value: String},

    TransferTokens{to: String, amount: u128},

    CreateTokens{reciever: String, amount: u128},
}

#[derive(Clone, Debug)]

//repersents an account on the blockchain
pub struct Account {

    store: HashMap<String, String>,

    acc_type: AccountType,

    tokens: u128,
}

#[derive (Clone, Debug)]
pub enum AccountType {

    User,

    Contract,

    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,

    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        //This block can create a user
        let is_genesis = self.len() == 0;

        if !block.verify_own_hash() {
            return ("The block hash is mismatching! (Code: 93820394)".into());
        }

        if !(block.prev_hash == self.get_last_block_hash()) {
            return Err("The new block has to point to the previous block".into());
        }

        if block.get_transactions_count() == 0 {
            return Err("There has to be at least one tranbsactions inside the block".into());
        }

        let old_state = self.accounts.clone();

        for (i, transaction) in block.transactions.iter().enumerate() {

            if let Err(err) = transaction.execute(self, &is_genesis){
                self. accounts = old_state;

                return Err(format!("Could not execute transaction {} due to {}, Rolling back", i+1, err));
            }
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    //Calculate hash of the whole block including transactions, Blake2 Hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calulate_hash())
        }

        let block_as_string = format!("{:?}", (&self.prev_hash, &self.nonce));
        hasher.update(&block_as_bytes);

        return Vec::from(hasher.finalize().as_ref());
    }


}

impl Account {
    //Constructor

    pub fn new(account_type: AccountType) -> Self {
        return Self {
            tokens: 0,
            acc_type: account_type,
            store: HashMap::new()
        }
    }
}
