#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Crowdfunding campagne status
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[elrond_wasm::contract]
pub trait Crowdfunding {
    // constructor
    #[init]
    fn init(
        &self,
        initial_value: BigInt, // Test basic increment mapper
        target: &BigUint,      // crowdfunding target in wei
        deadline: u64,         // deadline (block nonce)
    ) {
        let my_address: ManagedAddress = self.blockchain().get_caller();
        self.set_owner(&my_address);
        self.sum().set(&initial_value);
        self.set_target(target);
        self.set_deadline(deadline);
    }

    // Let user fund the current campagne
    #[payable("*")]
    #[endpoint]
    fn fund(&self, #[payment] payment: BigUint) -> SCResult<()> {
        if self.blockchain().get_block_nonce() > self.get_deadline() {
            return sc_error!("cannot fund after deadline");
        }
        let caller = self.blockchain().get_caller();
        let mut deposit = self.get_deposit(&caller);
        deposit += payment;
        self.set_deposit(&caller, &deposit);
        Ok(())
    }

    // Get the campagne status
    #[view]
    fn status(&self) -> Status {
        if self.blockchain().get_block_nonce() <= self.get_deadline() {
            Status::FundingPeriod
        } else if self
            .blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
            >= self.get_target()
        {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    // Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigInt) -> SCResult<()> {
        self.sum().update(|sum| *sum += value);

        Ok(())
    }

    ///////////////////////////////////
    /// Storage
    ///////////////////////////////////

    // Crowdfunding target
    #[storage_set("target")]
    fn set_target(&self, target: &BigUint);

    #[view]
    #[storage_get("target")]
    fn get_target(&self) -> BigUint;

    // Crowdfunding deadline
    #[storage_set("deadline")]
    fn set_deadline(&self, deadline: u64);

    #[view]
    #[storage_get("deadline")]
    fn get_deadline(&self) -> u64;

    // campagne deposit
    #[storage_set("deposit")]
    fn set_deposit(&self, donor: &ManagedAddress, amount: &BigUint);

    #[view]
    #[storage_get("deposit")]
    fn get_deposit(&self, donor: &ManagedAddress) -> BigUint;

    // Contract owner (or crowdfunding owner??)
    #[storage_set("owner")]
    fn set_owner(&self, address: &ManagedAddress);

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> ManagedAddress;

    // Test mapper
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigInt>;
}
