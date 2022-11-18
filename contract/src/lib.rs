mod models;

// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, require};
use near_sdk::{
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, Timestamp,
};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    d_list: LookupMap<String, Vector<ItemInfo>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemInfo {
    creator: AccountId,
    post_id: u64,
    date: Timestamp,
    category: String,
    title: String,
    description: String,
    image: Option<Vec<String>>,
    location: Option<String>,
    price: Option<u32>,
    details: Option<Details>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Details {
    for_sale: Option<models::ForSale>,
    community: Option<models::Community>,
    housing: Option<models::Housing>,
    jobs: Option<models::Job>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            d_list: LookupMap::new(b"dlist".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_item(&self, group: String, post_id: u64) -> Option<ItemInfo> {
        match self.d_list.get(&group) {
            Some(items) => items
                .iter()
                .find(|item| -> bool { item.post_id == post_id }),
            None => None,
        }
    }

    pub fn get_items(&self, group: String) -> Vec<ItemInfo> {
        match self.d_list.get(&group) {
            Some(items) => items.to_vec(),
            None => vec![],
        }
    }

    #[payable]
    pub fn set_items(&mut self, group: String, item_info_in: ItemInfo) {
        let mut temp_list = match self.d_list.get(&group) {
            Some(items) => items,
            None => Vector::new(group.as_bytes()),
        };

        temp_list.push(&item_info_in);

        self.d_list.insert(&group, &temp_list);

    }

    pub fn remove_items(
        &mut self,
        group: String,
        account_id: String,
        post_id: u64,
    ) -> Option<ItemInfo> {
        require!(
            env::signer_account_id().to_string() == account_id,
            "can not delete item check that account id is equal to yours!"
        );

        if let Some(mut items) = self.d_list.get(&group) {
            if let Some(index) = items.iter().position(|item| item.post_id == post_id) {
                let r_item = items.swap_remove(index.try_into().unwrap());
                self.d_list.insert(&group, &items);

                Some(r_item)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::env;

    use super::*;

    #[test]
    fn get_one_item_post_id() {
        let mut contract = Contract::default();
        let item_info = ItemInfo {
            creator: env::current_account_id(),
            post_id: 365416516352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info);

        let item_info_2 = ItemInfo {
            creator: env::current_account_id(),
            post_id: 654612352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info_2);

        let x = contract.get_item("community".to_string(), 65612352);

        assert_eq!(x, None);
    }

    #[test]
    fn set_items_check() {
        let mut contract = Contract::default();
        let item_info = ItemInfo {
            creator: env::current_account_id(),
            post_id: 365416516352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info);

        let item_info_2 = ItemInfo {
            creator: env::current_account_id(),
            post_id: 654612352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info_2);
        let check_len = contract.get_items("community".to_string());
        assert_eq!(check_len.len(), 2);
    }

    #[test]
    fn remove_item_post_id() {
        let mut contract = Contract::default();
        let item_info = ItemInfo {
            creator: env::current_account_id(),
            post_id: 365416516352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info);

        let item_info_2 = ItemInfo {
            creator: env::current_account_id(),
            post_id: 654612352,
            date: env::block_timestamp(),
            category: "finance".to_string(),
            title: "Looking for a financal tech".to_string(),
            description:
                "This job is looking for a person to do all the money making I sit back and watch"
                    .to_string(),
            image: None,
            location: Some("remote".to_string()),
            price: None,
            details: None,
        };

        contract.set_items("community".to_string(), item_info_2);

        let x = contract.get_item("community".to_string(), 654612352);
        let y = contract.remove_items(
            "community".to_string(),
            env::signer_account_id().to_string(),
            654612352,
        );
        let check_len = contract.get_items("community".to_string());
        let y_2 = contract.remove_items(
            "community".to_string(),
            env::signer_account_id().to_string(),
            654612352,
        );

        assert_ne!(x, None);
        assert_ne!(y, None);
        assert_eq!(check_len.len(), 1);
        assert_eq!(y_2, None)
    }
}
