use cosmwasm_schema::write_api;

use stargate_tester::msg::{ExecuteMsg, InstantiateMsg};
use classic_bindings::TerraQuery;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: TerraQuery,
    }
}
