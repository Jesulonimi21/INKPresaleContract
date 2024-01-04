#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod bugbite_presale {
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use ink::prelude::vec;

    use ink::{
        codegen::EmitEvent,
        prelude::{format, string::String, vec::Vec},
        reflect::ContractEventBase,
        storage::Mapping,
        ToAccountId,
    };

    use::first_aleph_my_token::MytokenRef;
    use::first_aleph_my_token::Error;
    use ink::{
        env::{
            CallFlags,
            call::{build_call, ExecutionInput, FromAccountId, Selector},
            DefaultEnvironment, Error as InkEnvError,
        },
        LangError,
    };

    #[ink(storage)]
    // #[derive(Default)]X/
    pub struct Token{
        owner: AccountId,
        price_per_token: Balance,
        supply_remaining: Balance,
        PresaleAsset: MytokenRef
    }



    impl Token{
        #[ink(constructor)]
        pub fn new(supply_remaining: Balance, price_per_token: Balance, presale_token: AccountId, presale_asset_hash: Hash) -> Self{
            //supply_remaining is supposed to be the number of tokens received
            // let _ = PSP22Ref::balance_of(&presale_token, Self::env().account_id());
            let caller = Self::env().caller();
            // let version: u8 = 1;

            // let presale_asset_ref = MytokenRef::new(0)
            // .endowment(0) /* Amount of value transferred as part of the call. 
            //                * It should not be required but the API of `*Ref` pattern 
            //                * does not allow for calling `instantiate()` 
            //                * on a builder where `endowment` is not set.*/
            // .code_hash(presale_asset_hash)
            // .salt_bytes(&[version.to_le_bytes().as_ref(), Self::env().caller().as_ref()].concat()[..4])
            // .instantiate();

            // let presale_asset =
            //     <MytokenRef as ToAccountId<
            //         super::bugbite_presale::Environment,
            //     >>::to_account_id(&presale_asset_ref);

            let other_contract = MytokenRef::from_account_id(presale_token);
            Self { price_per_token, supply_remaining, owner: caller,  PresaleAsset: other_contract}
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
        pub fn buy_token(&mut self, amount_to_purchase: u128, to: AccountId, hash: Hash) -> Balance{
            let from = self.env().caller();
            let price: Balance = amount_to_purchase * self.price_per_token;
            let transferred_value = self.env().transferred_value();
            // assert_eq!(transferred_value, price);
            Self::env().transfer(self.owner, transferred_value);
            let current_contract_id = Self::env().account_id();
            // let _ =  PSP22Ref::transfer_from(&self.presale_token, from, self.get_owner(), price.into(), Vec::<u8>::new());
            self.supply_remaining = self.supply_remaining - amount_to_purchase;
            let res: Result<(), Error> = self.PresaleAsset.transfer_from(from, to, price);
            let to_balance = self.PresaleAsset.balance_of(to);

            // let selector = ink::selector_bytes!("transfer");

            // let _ = build_call::<DefaultEnvironment>()
            // .delegate(self.PresaleAsset)
            // .call_flags(CallFlags::default().set_tail_call(true))
            // .exec_input(ExecutionInput::new(Selector::new(selector)).push_arg(to)
            // .push_arg(price))
            // .returns::<Result<(), Error>>()
            // .invoke();
            // let sec = build_call::<DefaultEnvironment>()
            //         .delegate(hash)
            //         // .call(self.PresaleAsset)
            //         .exec_input(
            //             ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
            //                 .push_arg(to)
            //                 .push_arg(price)
            //         )
            //         .returns::<Result<(), Error>>()
            //         .invoke();

            
            // let my_return_value = <MytokenRef as FromAccountId<
            //         super::bugbite_presale::Environment,
            //     >>::from_account_id( self.PresaleAsset)
            //     // change to from
            //     .transfer(to, price);
            

            // let to_balance = <MytokenRef as FromAccountId<
            //     super::bugbite_presale::Environment,
            // >>::from_account_id( self.PresaleAsset)
            // // change to from
            // .balance_of(to);
                
            //  assert_eq!(to_balance, price);
            to_balance
        }
    }

    
}
//1000000000000

// 4538138c0f25518cd6855b3075e900cb6a806fce623d7bb5fb5ba1aab741c741