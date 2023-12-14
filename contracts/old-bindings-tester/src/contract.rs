#[cfg(not(feature = "imported"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, ContractInfoResponse
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(feature = "old"))]
use classic_bindings::{
    TerraMsg, TerraQuerier, TerraQuery, ExchangeRatesResponse,
    SwapResponse, TaxCapResponse, TaxRateResponse,
};
#[cfg(feature = "old")]
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ContractInfoResponse, ExchangeRatesResponse,
    SwapResponse, TaxCapResponse, TaxRateResponse, TerraMsgWrapper, TerraQuerier, TerraQueryWrapper
};


#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<TerraMsg>> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TerraMsg>, StdError> {
    match msg {
        ExecuteMsg::Swap {
            offer_coin,
            ask_denom,
        } => execute_msg_swap(deps, env, info, offer_coin, ask_denom, None),
        ExecuteMsg::SwapSend {
            to_address,
            offer_coin,
            ask_denom,
        } => execute_msg_swap(deps, env, info, offer_coin, ask_denom, Some(to_address)),
    }
}

pub fn execute_msg_swap(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    offer_coin: Coin,
    ask_denom: String,
    recipient: Option<Addr>,
) -> StdResult<Response<TerraMsg>> {
    let msg = if let Some(recipient) = recipient {
        TerraMsg::create_swap_send_msg(recipient.to_string(), offer_coin, ask_denom)
    } else {
        TerraMsg::create_swap_msg(offer_coin, ask_denom)
    };

    Ok(Response::new().add_message(msg))
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn query(deps: Deps<TerraQuery>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Swap {
            offer_coin,
            ask_denom,
        } => to_json_binary(&query_swap(deps, offer_coin, ask_denom)?),
        QueryMsg::TaxRate {} => to_json_binary(&query_tax_rate(deps)?),
        QueryMsg::TaxCap { denom } => to_json_binary(&query_tax_cap(deps, denom)?),
        QueryMsg::ExchangeRates {
            base_denom,
            quote_denoms,
        } => to_json_binary(&query_exchange_rates(deps, base_denom, quote_denoms)?),
        QueryMsg::ContractInfo { contract_address } => {
            to_json_binary(&query_contract_info(deps, contract_address)?)
        }
    }
}

pub fn query_swap(deps: Deps<TerraQuery>, offer_coin: Coin, ask_denom: String) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate(deps: Deps<TerraQuery>) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap(deps: Deps<TerraQuery>, denom: String) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxCapResponse = querier.query_tax_cap(denom)?;

    Ok(res)
}

pub fn query_exchange_rates(
    deps: Deps<TerraQuery>,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;

    Ok(res)
}

pub fn query_contract_info(
    deps: Deps<TerraQuery>,
    contract_address: String,
) -> StdResult<ContractInfoResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ContractInfoResponse = querier.query_contract_info(contract_address)?;

    Ok(res)
}
