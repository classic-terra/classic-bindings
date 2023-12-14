use classic_bindings::{TerraQuery, TerraStargateQuerier};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stargate-tester";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: TerraQuery) -> StdResult<Binary> {
    let querier = TerraStargateQuerier::new(&deps.querier);

    match msg {
        TerraQuery::Swap { offer_coin, ask_denom } => to_json_binary(&querier.query_swap(offer_coin, ask_denom)?),
        TerraQuery::TaxCap { denom } => to_json_binary(&querier.query_tax_cap(denom)?),
        TerraQuery::TaxRate { } => to_json_binary(&querier.query_tax_rate()?),
        TerraQuery::ExchangeRates { base_denom, quote_denoms } => to_json_binary(&querier.query_exchange_rates(base_denom, quote_denoms)?),
    }
}

#[cfg(test)]
mod tests {}
