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



    impl Token{
        #[ink(constructor)]
        pub fn new(price_per_token: Balance, presale_token: AccountId) -> Self{
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
            let from = self.env().caller();
            let price: Balance = (amount_to_purchase * self.price_per_token)/1000000000000;
            let transferred_value = self.env().transferred_value();
            assert_eq!(transferred_value, price);
            Self::env().transfer(self.owner, transferred_value);
            let mut token: contract_ref!(PSP22) = self.presale_asset.into();
            let to_balance_before = token.balance_of(from);
            // let _ = token.transfer_from(self.owner, Self::env().account_id(), price, Vec::<u8>::new());
            let _ = token.transfer(from, price,  Vec::<u8>::new());
            let to_balance = token.balance_of(from);
            let new_balance = to_balance - to_balance_before;
            assert_eq!(new_balance, price);
            new_balance
        }

        #[ink(message)]
        pub fn claim_tokens(&mut self, amount_tokens: Balance) -> Balance{
            assert_eq!(self.env().caller(), self.owner);
            let mut token: contract_ref!(PSP22) = self.presale_asset.into();
            let _ = token.transfer(self.owner, amount_tokens,  Vec::<u8>::new());
            amount_tokens
        }
    }

    
}