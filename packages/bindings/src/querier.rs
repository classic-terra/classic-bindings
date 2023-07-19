use std::str::FromStr;
use std::convert::TryFrom;

use classic_rust::types::terra::{market::v1beta1::QuerySwapRequest, treasury::v1beta1::{QueryTaxCapRequest, QueryTaxRateRequest}, oracle::v1beta1::QueryExchangeRateRequest};
use cosmwasm_std::{Coin, QuerierWrapper, QueryRequest, StdResult, ContractInfoResponse, Uint128, Decimal};

use crate::{query::{
    SwapResponse, TaxRateResponse, TaxCapResponse, ExchangeRatesResponse, TerraQuery,
}, ExchangeRateItem};

/// This is a helper wrapper to easily use our custom queries
pub struct TerraQuerier<'a> {
    querier: &'a QuerierWrapper<'a, TerraQuery>,
}

impl<'a> TerraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<TerraQuery>) -> Self {
        TerraQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let request = TerraQuery::Swap {
            offer_coin,
            ask_denom: ask_denom.into(),
        };

        let request: QueryRequest<TerraQuery> = TerraQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = TerraQuery::TaxCap {
            denom: denom.into(),
        };

        let request: QueryRequest<TerraQuery> = TerraQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = TerraQuery::TaxRate {};

        let request: QueryRequest<TerraQuery> = TerraQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let request = TerraQuery::ExchangeRates {
            base_denom: base_denom.into(),
            quote_denoms: quote_denoms.into_iter().map(|x| x.into()).collect(),
        };

        let request: QueryRequest<TerraQuery> = TerraQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        self.querier.query_wasm_contract_info(contract_address.into())
    }

}

#[cfg(feature = "stargate")]
/// This is a helper wrapper to easily use our custom queries through stargate query
pub struct TerraStargateQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

#[cfg(feature = "stargate")]
impl<'a> TerraStargateQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        TerraStargateQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let response = QuerySwapRequest{
            offer_coin: offer_coin.to_string(),
            ask_denom: ask_denom.into()
        }.query(self.querier).unwrap();

        Ok(SwapResponse { receive: Coin::try_from(response.return_coin.unwrap()).unwrap() })
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let response = QueryTaxCapRequest {
            denom: denom.into(),
        }.query(self.querier).unwrap();

        Ok(TaxCapResponse { cap: Uint128::from_str(&response.tax_cap).unwrap() })
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let response = QueryTaxRateRequest {}.query(self.querier).unwrap();

        Ok(TaxRateResponse { rate: Decimal::from_str(&response.tax_rate).unwrap() })
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let base_denom_str: String = base_denom.into();

        // LUNA / BASE_DENOM
        let base_denom_rate = Decimal::from_str(
            &QueryExchangeRateRequest {
                denom: base_denom_str.clone()
            }.query(self.querier).unwrap().exchange_rate
        ).unwrap();

        let exchange_rates = quote_denoms
            .into_iter()
            .map(|quote_denom| {
                let quote_denom_str: String = quote_denom.into();

                // LUNA / QUOTE_DENOM
                let quote_denom_rate = Decimal::from_str(
                    &QueryExchangeRateRequest {
                        denom: quote_denom_str.clone()
                    }.query(self.querier).unwrap().exchange_rate
                ).unwrap();

                ExchangeRateItem {
                    quote_denom: quote_denom_str,
                    exchange_rate: quote_denom_rate.checked_div(base_denom_rate).unwrap()
                }
            })
            .collect();

        Ok(ExchangeRatesResponse { base_denom: base_denom_str, exchange_rates })
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        self.querier.query_wasm_contract_info(contract_address.into())
    }
}