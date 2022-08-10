#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod students {
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_storage::traits::SpreadAllocate;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Students {
        // Store a mapping from AccountIds to a u32
        map: ink_storage::Mapping<AccountId, u32>,
    }

    impl Students {
        #[ink(constructor)]
        pub fn new(student_old:  u32) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.map.insert(&caller, &student_old);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn check_old(&self) ->  String {
            let caller = Self::env().caller();
            let student_old = self.map.get(&caller).unwrap_or_default();
            let mut mess = String::from(student_old.to_string());
            if student_old >= 18 {
                mess.push_str(" is old enough");
            } else {
                mess.push_str(" is not old enough");
            }
            mess
        }
    }
}
