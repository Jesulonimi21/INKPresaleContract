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
        supply_remaining: Balance,
        PresaleAsset: AccountId
    }



    impl Token{
        #[ink(constructor)]
        pub fn new(supply_remaining: Balance, price_per_token: Balance, presale_token: AccountId) -> Self{
            let caller = Self::env().caller();
            Self { price_per_token, supply_remaining, owner: caller,  PresaleAsset: presale_token }
        }

        #[ink(message)]
        pub fn supply_remaining(&self) -> u128 {
            self.supply_remaining
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
            let mut token: contract_ref!(PSP22) = self.PresaleAsset.into();
            let _ = token.transfer_from(self.owner, Self::env().account_id(), price, Vec::<u8>::new());
            let _ = token.transfer(from, price,  Vec::<u8>::new());
            self.supply_remaining = self.supply_remaining - amount_to_purchase;
            let to_balance = token.balance_of(from);
            to_balance
        }
    }

    
}