use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery, Decimal, Uint128};

impl CustomQuery for TerraQuery {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum TerraQuery {
    #[returns(SwapResponse)]
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    #[returns(TaxRateResponse)]
    TaxRate {},
    #[returns(TaxCapResponse)]
    TaxCap {
        denom: String,
    },
    #[returns(ExchangeRatesResponse)]
    ExchangeRates {
        base_denom: String,
        quote_denoms: Vec<String>,
    },
}

/// SwapResponse is data format returned from SwapRequest::Simulate query
#[cw_serde]
pub struct SwapResponse {
    pub receive: Coin,
}

/// TaxRateResponse is data format returned from TreasuryRequest::TaxRate query
#[cw_serde]
pub struct TaxRateResponse {
    pub rate: Decimal,
}

/// TaxCapResponse is data format returned from TreasuryRequest::TaxCap query
#[cw_serde]
pub struct TaxCapResponse {
    pub cap: Uint128,
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[cw_serde]
pub struct ExchangeRateItem {
    pub quote_denom: String,
    pub exchange_rate: Decimal,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[cw_serde]
pub struct ExchangeRatesResponse {
    pub base_denom: String,
    pub exchange_rates: Vec<ExchangeRateItem>,
}