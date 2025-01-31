use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Item};

use super::fetch_ident;

#[derive(Debug)]
pub struct AssetAPIFields {
    pub transaction_payment: Ident,
    pub balance: Ident,
    pub call: Ident,
    pub oracle_key: Ident,
    pub oracle: Ident,
}

impl TryFrom<&[Item]> for AssetAPIFields {
    type Error = &'static str;
    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut transaction_payment = None;
        let mut call = None;
        let mut balance = None;
        let mut oracle_key = None;
        let mut oracle = None;

        for item in value {
            if let Item::Type(ty) = item {
                if ty.ident == "TransactionPayment" {
                    transaction_payment = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RuntimeCall" {
                    call = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Balance" {
                    balance = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "OracleKey" {
                    oracle_key = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Oracle" {
                    oracle = Some(fetch_ident(&ty.ty))
                }
            }
        }

        let transaction_payment =
            transaction_payment.ok_or("`type TransactionPayment` not specified, but required")?;
        let balance = balance.ok_or("`type Balance` not specified, but required")?;
        let call = call.ok_or("`type RuntimeCall` not specified, but required")?;
        let oracle_key = oracle_key.ok_or("`type OracleKey` not specified, but required")?;
        let oracle = oracle.ok_or("`type Oracle` not specified, but required")?;
        Ok(AssetAPIFields {
            transaction_payment,
            balance,
            call,
            oracle_key,
            oracle
        })
    }
}

pub fn assets_apis(
    runtime: &Ident,
    block: &Ident,
    transaction_payment: &Ident,
    balance: &Ident,
    call: &Ident,
    oracle_key: &Ident,
    oracle: &Ident
) -> TokenStream {
    quote! {
        impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<#block, #balance>
            for #runtime
        {
            fn query_info(
                uxt: <#block as sp_runtime::traits::Block>::Extrinsic,
                len: u32,
            ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<#balance> {
                #transaction_payment::query_info(uxt, len)
            }
            fn query_fee_details(
                uxt: <#block as sp_runtime::traits::Block>::Extrinsic,
                len: u32,
            ) -> pallet_transaction_payment::FeeDetails<#balance> {
                #transaction_payment::query_fee_details(uxt, len)
            }
            fn query_weight_to_fee(weight: frame_support::weights::Weight) -> #balance {
                #transaction_payment::weight_to_fee(weight)
            }
            fn query_length_to_fee(length: u32) -> #balance {
                #transaction_payment::length_to_fee(length)
            }
        }

        impl
            pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<
                #block,
                #balance,
                #call,
            > for #runtime
        {
            fn query_call_info(
                call: #call,
                len: u32,
            ) -> pallet_transaction_payment::RuntimeDispatchInfo<#balance> {
                #transaction_payment::query_call_info(call, len)
            }
            fn query_call_fee_details(
                call: #call,
                len: u32,
            ) -> pallet_transaction_payment::FeeDetails<#balance> {
                #transaction_payment::query_call_fee_details(call, len)
            }
            fn query_weight_to_fee(weight: frame_support::weights::Weight) -> #balance {
                #transaction_payment::weight_to_fee(weight)
            }
            fn query_length_to_fee(length: u32) -> #balance {
                #transaction_payment::length_to_fee(length)
            }
        }

        impl orml_oracle_runtime_api::OracleApi<
            #block,
            (),
            #oracle_key,
            orml_oracle::TimestampedValue<sp_runtime::FixedU128, u64>,
        > for Runtime {
            fn get_value(_: (), key: #oracle_key) -> Option<orml_oracle::TimestampedValue<sp_runtime::FixedU128, u64>> {
                #oracle::get(&key)
            }

            fn get_all_values(_: ()) -> sp_std::prelude::Vec<(#oracle_key, Option<orml_oracle::TimestampedValue<sp_runtime::FixedU128, u64>>)> {
                #oracle::get_all_values()
            }
        }
    }
}
