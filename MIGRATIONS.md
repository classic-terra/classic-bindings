# Migration guides
1. Replace package `terra-cosmwasm` with `classic-bindings`
2. Replace `terra_cosmwasm::TerraMsgWrapper` to `classic_bindings::TerraMsg`
3. Replace `terra_cosmwasm::TerraQueryWrapper` to `classic_bindings::TerraQuery`
4. Replace `terra_cosmwasm::ContractInfoResponse` to `cosmwasm_std::ContractInfoResponse`
5. Replace `terra_cosmwasm::create_swap_send_msg` to `classic_bindings::TerraMsg::create_swap_send_msg`
6. Replace `terra_cosmwasm::create_swap_msg` to `classic_bindings::TerraMsg::create_swap_msg`

Example of migrated terra bindings: [here](contracts/old-bindings-tester/README.md)

Cargo for classic-bindings: https://crates.io/crates/classic-bindings
