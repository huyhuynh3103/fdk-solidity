use crate::abi::IFDK;
use alloy_primitives::{Address, hex};
use alloy_sol_types::{SolCall, SolValue};
use foundry_evm::revm::{
    context::ContextTr,
    interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult},
    Inspector,
};

pub const CHEATCODE_ADDRESS: Address =
    Address::new(hex!("7109709ECfa91a80626fF3989D68f67F5b1DD12D"));

pub struct FdkInspector;

impl<CTX: ContextTr> Inspector<CTX> for FdkInspector {
    fn call(&mut self, context: &mut CTX, inputs: &mut CallInputs) -> Option<CallOutcome> {
        if inputs.target_address != CHEATCODE_ADDRESS {
            return None;
        }

        let input_bytes = inputs.input.bytes(context);
        if input_bytes.len() < 4 {
            return None;
        }

        // abi_decode expects full calldata (with selector)
        if let Ok(decoded) = IFDK::deployTransparentProxyCall::abi_decode(&input_bytes) {
            println!("Intercepted deployTransparentProxy!");
            println!("  logic: {}", decoded.logic);
            println!("  admin: {}", decoded.admin);

            let mocked_proxy = Address::repeat_byte(0x99);
            return Some(CallOutcome::new(
                InterpreterResult::new(
                    InstructionResult::Return,
                    mocked_proxy.abi_encode().into(),
                    Gas::new(inputs.gas_limit),
                ),
                inputs.return_memory_offset.clone(),
            ));
        }

        None
    }
}
