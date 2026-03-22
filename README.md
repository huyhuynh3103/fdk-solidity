# fdk-solidity

A custom EVM execution environment built on [revm](https://github.com/bluealloy/revm) and [Foundry](https://github.com/foundry-rs/foundry), implementing cheatcode-style function interception following the [Foundry cheatcodes pattern](https://github.com/foundry-rs/foundry/blob/master/docs/dev/cheatcodes.md).

## How It Works

Calls to the cheatcode address (`0x7109709ECfa91a80626fF3989D68f67F5b1DD12D`) are intercepted by a custom `revm::Inspector` before the EVM executes any bytecode. The calldata is decoded using `sol!`-generated Rust bindings and short-circuited with a mocked response.

### Architecture

```
src/
├── abi.rs        # Cheatcode definitions (Solidity ABI via sol! macro)
├── inspector.rs  # Inspector that intercepts calls to the cheatcode address
└── main.rs       # EVM setup, transaction construction, and execution
```

### Flow

1. `main.rs` builds an EVM instance with `FdkInspector` attached
2. A transaction targeting `CHEATCODE_ADDRESS` is constructed with ABI-encoded calldata
3. During execution, `Inspector::call` fires and checks if the target is the cheatcode address
4. If matched, the calldata is decoded into a typed struct and a mocked `CallOutcome` is returned
5. The EVM never tries to execute bytecode at the cheatcode address

## Usage

```bash
cargo run -- --script-path <path>
```

Currently `--script-path` is a placeholder; the transaction is hardcoded as a mock call to `deployTransparentProxy`.

## Adding a New Cheatcode

### 1. Define the Solidity signature in `src/abi.rs`

```rust
sol! {
    interface IFDK {
        function deployTransparentProxy(address logic, address admin, bytes memory data) external returns (address proxy);
        // Add new cheatcodes:
        function config(string memory key) external returns (string memory value);
    }
}
```

### 2. Handle it in `src/inspector.rs`

Add a new decode branch inside the `call` method:

```rust
if let Ok(decoded) = IFDK::configCall::abi_decode(&input_bytes) {
    println!("Intercepted config! key: {}", decoded.key);

    let value = String::from("some_value");
    return Some(CallOutcome::new(
        InterpreterResult::new(
            InstructionResult::Return,
            value.abi_encode().into(),
            Gas::new(inputs.gas_limit),
        ),
        inputs.return_memory_offset.clone(),
    ));
}
```

### 3. (Optional) Construct a test call in `src/main.rs`

```rust
let calldata: Bytes = IFDK::configCall {
    key: "my_key".to_string(),
}
.abi_encode()
.into();
```

## Dependencies

- **foundry-evm** (v1.5.0) -- provides the revm re-export (v33.1.0) to avoid version conflicts
- **foundry-cheatcodes** (v1.5.0) -- Foundry's cheatcode infrastructure
- **alloy-sol-types** / **alloy-primitives** -- ABI encoding/decoding via the `sol!` macro
- **clap** -- CLI argument parsing
- **eyre** -- error handling
