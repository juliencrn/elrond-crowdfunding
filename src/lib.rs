#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, initial_value: BigInt) {
        let my_address: ManagedAddress = self.blockchain().get_caller();
        self.set_owner(&my_address);
        self.sum().set(&initial_value);
    }

    #[storage_set("owner")]
    fn set_owner(&self, address: &ManagedAddress);

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> ManagedAddress;

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigInt) -> SCResult<()> {
        self.sum().update(|sum| *sum += value);

        Ok(())
    }

    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigInt>;
}
