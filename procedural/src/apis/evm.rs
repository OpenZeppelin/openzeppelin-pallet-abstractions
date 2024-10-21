use proc_macro2::TokenStream;
use syn::{Ident, Item};

use super::fetch_ident;

#[derive(Debug)]
pub struct EVMAPIFields {
    pub call: Ident,
    pub executive: Ident,
    pub ethereum: Ident,
}

impl TryFrom<&[Item]> for EVMAPIFields {
    type Error = &'static str;

    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut call = None;
        let mut executive = None;
        let mut ethereum = None;

        for item in value {
            match item {
                Item::Type(ty) => {
                    if ty.ident == "RuntimeCall" {
                        call = Some(fetch_ident(&ty.ty))
                    } else if ty.ident == "Executive" {
                        executive = Some(fetch_ident(&ty.ty))
                    } else if ty.ident == "Ethereum" {
                        ethereum = Some(fetch_ident(&ty.ty))
                    }
                }
                _ => (),
            }
        }

        let executive = executive.ok_or("`type Executive` not specified, but required")?;
        let ethereum = ethereum.ok_or("`type Ethereum` not specified, but required")?;
        let call = call.ok_or("`type Call` not specified, but required")?;

        Ok(EVMAPIFields {
            call,
            executive,
            ethereum,
        })
    }
}

pub fn evm_apis(
    runtime: &Ident,
    block: &Ident,
    runtime_call: &Ident,
    executive: &Ident,
    ethereum: &Ident,
) -> TokenStream {
    quote::quote! {
        use pallet_ethereum::{
            Call::transact, Transaction as EthereumTransaction, TransactionAction, TransactionData,
            TransactionStatus,
        };
        use pallet_evm::{Account as EVMAccount, FeeCalculator, Runner};
        use sp_core::{H160, H256, U256};

        impl fp_rpc::EthereumRuntimeRPCApi<#block> for #runtime {
            /// Returns runtime defined pallet_evm::ChainId.
            fn chain_id() -> u64 {
                <#runtime as pallet_evm::Config>::ChainId::get()
            }

            /// Returns pallet_evm::Accounts by address.
            fn account_basic(address: H160) -> EVMAccount {
                let (account, _) = pallet_evm::Pallet::<#runtime>::account_basic(&address);
                account
            }

            /// Returns FixedGasPrice::min_gas_price
            fn gas_price() -> U256 {
                let (gas_price, _) = <#runtime as pallet_evm::Config>::FeeCalculator::min_gas_price();
                gas_price
            }

            /// For a given account address, returns pallet_evm::AccountCodes.
            fn account_code_at(address: H160) -> Vec<u8> {
                pallet_evm::AccountCodes::<#runtime>::get(address)
            }

            /// Returns the converted FindAuthor::find_author authority id.
            fn author() -> H160 {
                <pallet_evm::Pallet<#runtime>>::find_author()
            }

            /// For a given account address and index, returns pallet_evm::AccountStorages.
            fn storage_at(address: H160, index: U256) -> H256 {
                let mut tmp = [0u8; 32];
                index.to_big_endian(&mut tmp);
                pallet_evm::AccountStorages::<#runtime>::get(address, H256::from_slice(&tmp[..]))
            }

            /// Returns a frame_ethereum::call response.
            fn call(
                from: H160,
                to: H160,
                data: Vec<u8>,
                value: U256,
                gas_limit: U256,
                max_fee_per_gas: Option<U256>,
                max_priority_fee_per_gas: Option<U256>,
                nonce: Option<U256>,
                estimate: bool,
                access_list: Option<Vec<(H160, Vec<H256>)>>,
            ) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
                let config = if estimate {
                    let mut config = <#runtime as pallet_evm::Config>::config().clone();
                    config.estimate = true;
                    Some(config)
                } else {
                    None
                };

                let gas_limit = gas_limit.min(u64::MAX.into());
                let transaction_data = TransactionData::new(
                    TransactionAction::Call(to),
                    data.clone(),
                    nonce.unwrap_or_default(),
                    gas_limit,
                    None,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    value,
                    Some(<#runtime as pallet_evm::Config>::ChainId::get()),
                    access_list.clone().unwrap_or_default(),
                );
                let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<#runtime>::transaction_weight(&transaction_data);

                <#runtime as pallet_evm::Config>::Runner::call(
                    from,
                    to,
                    data,
                    value,
                    gas_limit.unique_saturated_into(),
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list.unwrap_or_default(),
                    false,
                    true,
                    weight_limit,
                    proof_size_base_cost,
                    config.as_ref().unwrap_or(<#runtime as pallet_evm::Config>::config()),
                ).map_err(|err| err.error.into())
            }

            /// Returns a frame_ethereum::create response.
            fn create(
                from: H160,
                data: Vec<u8>,
                value: U256,
                gas_limit: U256,
                max_fee_per_gas: Option<U256>,
                max_priority_fee_per_gas: Option<U256>,
                nonce: Option<U256>,
                estimate: bool,
                access_list: Option<Vec<(H160, Vec<H256>)>>,
            ) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
                let config = if estimate {
                    let mut config = <#runtime as pallet_evm::Config>::config().clone();
                    config.estimate = true;
                    Some(config)
                } else {
                    None
                };

                let transaction_data = TransactionData::new(
                    TransactionAction::Create,
                    data.clone(),
                    nonce.unwrap_or_default(),
                    gas_limit,
                    None,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    value,
                    Some(<#runtime as pallet_evm::Config>::ChainId::get()),
                    access_list.clone().unwrap_or_default(),
                );
                let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<#runtime>::transaction_weight(&transaction_data);

                <#runtime as pallet_evm::Config>::Runner::create(
                    from,
                    data,
                    value,
                    gas_limit.unique_saturated_into(),
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list.unwrap_or_default(),
                    false,
                    true,
                    weight_limit,
                    proof_size_base_cost,
                    config.as_ref().unwrap_or(<#runtime as pallet_evm::Config>::config()),
                ).map_err(|err| err.error.into())
            }

            /// Return the current transaction status.
            fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
                pallet_ethereum::CurrentTransactionStatuses::<#runtime>::get()
            }

            /// Return the current block.
            fn current_block() -> Option<pallet_ethereum::Block> {
                pallet_ethereum::CurrentBlock::<#runtime>::get()
            }

            /// Return the current receipts.
            fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
                pallet_ethereum::CurrentReceipts::<#runtime>::get()
            }

            /// Return all the current data for a block in a single runtime call.
            fn current_all() -> (
                Option<pallet_ethereum::Block>,
                Option<Vec<pallet_ethereum::Receipt>>,
                Option<Vec<TransactionStatus>>
            ) {
                (
                    pallet_ethereum::CurrentBlock::<#runtime>::get(),
                    pallet_ethereum::CurrentReceipts::<#runtime>::get(),
                    pallet_ethereum::CurrentTransactionStatuses::<#runtime>::get()
                )
            }

            /// Receives a `Vec<OpaqueExtrinsic>` and filters out all the non-ethereum transactions.
            fn extrinsic_filter(
                xts: Vec<<#block as BlockT>::Extrinsic>,
            ) -> Vec<EthereumTransaction> {
                xts.into_iter().filter_map(|xt| match xt.0.function {
                    #runtime_call::Ethereum(transact { transaction }) => Some(transaction),
                    _ => None
                }).collect::<Vec<EthereumTransaction>>()
            }

            /// Return the elasticity multiplier.
            fn elasticity() -> Option<Permill> {
                Some(pallet_base_fee::Elasticity::<#runtime>::get())
            }

            /// Used to determine if gas limit multiplier for non-transactional calls (eth_call/estimateGas)
            /// is supported.
            fn gas_limit_multiplier_support() {}

            /// Return the pending block.
            fn pending_block(
                xts: Vec<<#block as BlockT>::Extrinsic>,
            ) -> (Option<pallet_ethereum::Block>, Option<Vec<TransactionStatus>>) {
                for ext in xts.into_iter() {
                    let _ = #executive::apply_extrinsic(ext);
                }

                #ethereum::on_finalize(System::block_number() + 1);

                (
                    pallet_ethereum::CurrentBlock::<#runtime>::get(),
                    pallet_ethereum::CurrentTransactionStatuses::<#runtime>::get()
                )
            }

            fn initialize_pending_block(header: &<#block as BlockT>::Header) {
                #executive::initialize_block(header);
            }
        }

        impl fp_rpc::ConvertTransactionRuntimeApi<#block> for #runtime {
            /// Converts an ethereum transaction into a transaction suitable for the runtime.
            fn convert_transaction(transaction: EthereumTransaction) -> <#block as BlockT>::Extrinsic {
                UncheckedExtrinsic::new_unsigned(
                    pallet_ethereum::Call::<#runtime>::transact { transaction }.into(),
                )
            }
        }
    }
}
