use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use terra_cosmwasm::{
    ExchangeRatesResponse, SwapResponse, TaxCapResponse, TaxRateResponse, TerraMsg,
    TerraMsgWrapper, TerraQuery, TerraQueryWrapper, TerraRoute,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(TerraMsgWrapper), &out_dir);
    export_schema(&schema_for!(TerraMsg), &out_dir);
    export_schema(&schema_for!(TerraQueryWrapper), &out_dir);
    export_schema(&schema_for!(TerraQuery), &out_dir);
    export_schema(&schema_for!(TerraRoute), &out_dir);
    export_schema(&schema_for!(SwapResponse), &out_dir);
    export_schema(&schema_for!(TaxCapResponse), &out_dir);
    export_schema(&schema_for!(TaxRateResponse), &out_dir);
    export_schema(&schema_for!(ExchangeRatesResponse), &out_dir);
}
