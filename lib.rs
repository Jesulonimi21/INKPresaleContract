#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod bugbite_presale {
    use ink::contract_ref;
    use ink::prelude::vec::Vec;
    use psp22::PSP22;
 

    #[ink(storage)]
    // #[derive(Default)]X/
    pub struct Token{
        owner: AccountId,
        price_per_token: Balance,
        presale_asset: AccountId
    }

    #[ink(event)]
    pub struct Purchased {
        from: AccountId,
        value: Balance,
        price: Balance
    }



    impl Token{
        #[ink(constructor)]
        pub fn new(price_per_token: Balance, presale_token: AccountId) -> Self{
            assert!(price_per_token > 0, "price per token must be greater than zero");
            let caller = Self::env().caller();
            Self { price_per_token,owner: caller,  presale_asset: presale_token }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId{
            self.owner
        }

        #[ink(message)]
        pub fn get_price(&self) -> u128{
            self.price_per_token
        }

        #[ink(message, payable)]
        pub fn buy_token(&mut self, amount_to_purchase: Balance) -> Balance{
            assert!(amount_to_purchase > 0, "amount to purchase must be greater than zero");
            let from = self.env().caller();
            let price: Balance = (amount_to_purchase * self.price_per_token)/1000_000_000_000;
            let transferred_value = self.env().transferred_value();
            assert!(transferred_value >= price, "value lesser than price");
            Self::env().transfer(self.owner, price).expect("Could not send native tokens to the owner");
            let mut token: contract_ref!(PSP22) = self.presale_asset.into();
            let to_balance_before = token.balance_of(from);
            // let _ = token.transfer_from(self.owner, Self::env().account_id(), price, Vec::<u8>::new());
            let _ = token.transfer(from, price,  Vec::<u8>::new());
            let to_balance = token.balance_of(from);
            let new_balance = to_balance - to_balance_before;
            assert_eq!(new_balance, price, "the senders balance does not change");
            if transferred_value > price {
                Self::env().transfer(from, transferred_value - price).expect("could not transfer excess price sent by user");
            };
            Self::env().emit_event(Purchased{
                from, value: amount_to_purchase, price
            });
            new_balance
        }

        #[ink(message)]
        pub fn claim_tokens(&mut self, amount_tokens: Balance) -> Balance{
            assert_eq!(self.env().caller(), self.owner,  "only admin can call this function");
            let mut token: contract_ref!(PSP22) = self.presale_asset.into();
            let _ = token.transfer(self.owner, amount_tokens,  Vec::<u8>::new());
            amount_tokens
        }

        #[ink(message, payable)]
        pub fn claim_native_token(&mut self)  -> Balance {
            let contract_balance = Self::env().balance();
            assert_eq!(self.env().caller(), self.owner);
            Self::env().transfer(self.owner, contract_balance).expect("could not send tokens to the owner");
            contract_balance
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) {
            assert_eq!(self.env().caller(), self.owner, "only admin can call this function");
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
    }

    
}