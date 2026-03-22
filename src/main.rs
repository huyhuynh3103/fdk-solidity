mod abi;
mod inspector;

use crate::abi::IFDK;
use crate::inspector::{FdkInspector, CHEATCODE_ADDRESS};
use alloy_primitives::Bytes;
use alloy_sol_types::SolCall;
use clap::Parser;
use foundry_evm::revm::{
    context::TxEnv,
    database_interface::EmptyDB,
    handler::{MainBuilder, MainContext},
    primitives::{TxKind, hardfork::SpecId},
    InspectEvm,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    script_path: String,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    println!("Booting execution environment for: {}", cli.script_path);

    let caller = alloy_primitives::Address::repeat_byte(0xAA);

    let calldata: Bytes = IFDK::deployTransparentProxyCall {
        logic: alloy_primitives::Address::repeat_byte(0x11),
        admin: alloy_primitives::Address::repeat_byte(0x22),
        data: vec![].into(),
    }
    .abi_encode()
    .into();

    let ctx = foundry_evm::revm::context::Context::mainnet()
        .modify_cfg_chained(|cfg| cfg.spec = SpecId::CANCUN)
        .with_db(EmptyDB::new());
    let mut evm = ctx.build_mainnet_with_inspector(FdkInspector);

    let tx = TxEnv::builder()
        .caller(caller)
        .kind(TxKind::Call(CHEATCODE_ADDRESS))
        .gas_limit(1_000_000)
        .data(calldata)
        .build()
        .map_err(|e| eyre::eyre!("{e:?}"))?;

    println!("Executing EVM...");
    let result = evm.inspect_tx(tx)?;

    println!("Execution Result: {:?}", result.result);

    Ok(())
}
