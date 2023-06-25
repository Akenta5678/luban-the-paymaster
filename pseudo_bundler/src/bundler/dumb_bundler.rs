use aa_bundler_primitives::{
    UserOperation, UserOperationByHash, UserOperationGasEstimation, UserOperationHash,
    UserOperationPartial, UserOperationReceipt,
};
use async_trait::async_trait;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H160, U256, U64},
};
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    tracing::info,
    types::{
        error::ErrorCode,
        ErrorObject,
    },
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

/// A simplified bundler implementation based on AA-Bundler
/// https://github.com/Vid201/aa-bundler
pub struct DumbBundler<M: Middleware> {
    /// The Provider that connects to Goerli
    pub eth_provider: Arc<M>,
    /// Goerli Chain ID
    pub eth_chain_id: U64,
    /// The Provider that connects to Mumbai
    pub poly_provider: Arc<M>,
    /// Mumbai Chain ID
    pub poly_chain_id: U64,
    /// Entry point address
    pub entry_point: Address,
    /// Max verification gas
    pub max_verification_gas: U256,
    /// Call gas Limit
    pub call_gas_limit: U256,
}

impl<M> DumbBundler<M>
where
    M: Middleware + 'static,
    M::Provider: Send + Sync + 'static,
{
    pub fn new(
        eth_provider: Arc<M>,
        poly_provider: Arc<M>,
        max_verification_gas: U256,
        call_gas_limit: U256,
    ) -> Self {
        Self {
            eth_provider,
            eth_chain_id: U64::from(5),
            poly_provider,
            poly_chain_id: U64::from(80001),
            entry_point: H160::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap(),
            max_verification_gas,
            call_gas_limit,
        }
    }
}

/// Eth API trait ported from AA-Bundler
///  https://github.com/Vid201/aa-bundler/blob/main/crates/rpc/src/eth_api.rs
#[derive(Serialize, Deserialize, Clone)]
pub struct EstimateUserOperationGasResponse {
    pub pre_verification_gas: U256,
    pub verification_gas_limit: U256,
    pub call_gas_limit: U256,
}

#[rpc(server, namespace = "eth")]
pub trait EthApi {
    #[method(name = "chainId")]
    async fn chain_id(&self) -> RpcResult<U64>;
    #[method(name = "supportedEntryPoints")]
    async fn supported_entry_points(&self) -> RpcResult<Vec<Address>>;
    #[method(name = "sendUserOperation")]
    async fn send_user_operation(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<UserOperationHash>;
    #[method(name = "estimateUserOperationGas")]
    async fn estimate_user_operation_gas(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<EstimateUserOperationGasResponse>;
    #[method(name = "getUserOperationReceipt")]
    async fn get_user_operation_receipt(
        &self,
        user_operation_hash: UserOperationHash,
    ) -> RpcResult<Option<UserOperationReceipt>>;
}

#[async_trait]
impl<M> EthApiServer for DumbBundler<M>
where
    M: Middleware + 'static,
    M::Provider: Send + Sync,
{
    async fn chain_id(&self) -> RpcResult<U64> {
        Ok(U64::from(6969))
    }

    async fn supported_entry_points(&self) -> RpcResult<Vec<Address>> {
        Ok(vec![Address::default()])
    }

    async fn send_user_operation(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<UserOperationHash> {
        info!("{:?}", user_operation);
        info!("{:?}", entry_point);
        let data = serde_json::value::to_raw_value(&"{\"a\": 100, \"b\": 200}").unwrap();
        println!("data: {:?}", data);
        // Ok(SendUserOperationResponse::Success(H256::default()));
	Err(
		ErrorObject::owned(
			ErrorCode::ServerError(-32000).code(),
			"Not implemented",
			Some(data),
		)
	)
    }

    async fn estimate_user_operation_gas(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<EstimateUserOperationGasResponse> {
        info!("{:?}", user_operation);
        info!("{:?}", entry_point);
        Ok(EstimateUserOperationGasResponse {
            pre_verification_gas: U256::from(0),
            verification_gas_limit: U256::from(0),
            call_gas_limit: U256::from(self.call_gas_limit),
        })
    }

    //     async fn estimate_user_operation_gas(
    //     &self,
    //     req: Request<EstimateUserOperationGasRequest>,
    // ) -> Result<Response<EstimateUserOperationGasResponse>, Status> {
    //     let req = req.into_inner();

    //     let uo = parse_uo(req.uo)?;
    //     let ep = parse_addr(req.ep)?;

    //     let uo_pool = parse_uo_pool(self.get_uo_pool(&ep))?;

    //     Ok(Response::new(
    //         match uo_pool.estimate_user_operation_gas(&uo).await {
    //             Ok(gas) => EstimateUserOperationGasResponse {
    //                 res: EstimateUserOperationGasResult::Estimated as i32,
    //                 data: serde_json::to_string(&gas).map_err(|err| {
    //                     Status::internal(format!("Failed to serialize gas: {err}"))
    //                 })?,
    //             },
    //             Err(err) => EstimateUserOperationGasResponse {
    //                 res: EstimateUserOperationGasResult::NotEstimated as i32,
    //                 data: serde_json::to_string(&err).map_err(|err| {
    //                     Status::internal(format!("Failed to serialize error: {err}"))
    //                 })?,
    //             },
    //         },
    //     ))
    // }


    // pub async fn estimate_user_operation_gas(
    //     &self,
    //     uo: &UserOperation,
    // ) -> Result<UserOperationGasEstimation, SimulationError> {
    //     let sim_res = self.simulate_user_operation(uo, false).await?;

    //     match self.entry_point.simulate_execution(uo.clone()).await {
    //         Ok(_) => {}
    //         Err(err) => {
    //             return Err(match err {
    //                 EntryPointErr::JsonRpcError(err) => SimulationError::Execution {
    //                     message: err.message,
    //                 },
    //                 _ => SimulationError::UnknownError {
    //                     message: format!("{err:?}"),
    //                 },
    //             })
    //         }
    //     }

    //     let exec_res = match self.entry_point.simulate_handle_op(uo.clone()).await {
    //         Ok(res) => res,
    //         Err(err) => {
    //             return Err(match err {
    //                 EntryPointErr::JsonRpcError(err) => SimulationError::Execution {
    //                     message: err.message,
    //                 },
    //                 _ => SimulationError::UnknownError {
    //                     message: format!("{err:?}"),
    //                 },
    //             })
    //         }
    //     };

    //     let base_fee_per_gas =
    //         self.base_fee_per_gas()
    //             .await
    //             .map_err(|err| SimulationError::UnknownError {
    //                 message: err.to_string(),
    //             })?;
    //     let call_gas_limit = calculate_call_gas_limit(
    //         exec_res.paid,
    //         exec_res.pre_op_gas,
    //         uo.max_fee_per_gas
    //             .min(uo.max_priority_fee_per_gas + base_fee_per_gas),
    //     );

    //     Ok(UserOperationGasEstimation {
    //         pre_verification_gas: Overhead::default().calculate_pre_verification_gas(uo),
    //         verification_gas_limit: sim_res.verification_gas_limit,
    //         call_gas_limit,
    //     })
    // }

    async fn get_user_operation_receipt(
        &self,
        user_operation_hash: UserOperationHash,
    ) -> RpcResult<Option<UserOperationReceipt>> {
        info!("{:?}", user_operation_hash);
        Ok(None)
    }
}
