# Terra reflect

This is a simple contract modified from a
[simple cosmwasm test contract](https://github.com/CosmWasm/cosmwasm/tree/main/contracts/reflect)
but customized to use the Terra queries and messages.

**This should not be used in any production systems.**

It is intended for use in the CI to ensure the Terra bindings are working properly.
The goal is that the golang CI will download this contract, send encoded TerraMsg structs to this contract.
This contract will then execute the provided structs, allowing it to test the functionality of the messages from cosmwasm.
