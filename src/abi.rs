use alloy_sol_types::sol;

// This macro generates the `IMyVm::deployTransparentProxyCall` struct
sol! {
    interface IFDK {
        function deployTransparentProxy(address logic, address admin, bytes memory data) external returns (address proxy);
        // You can add your config cheatcodes here later:
        // function config(string memory key) external returns (string memory value);
    }
}
