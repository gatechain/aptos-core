// Deploy this module to the special address 0x1
module aptos_framework::add {
    
    // ----------------------------------------------------
    // Native function declaration
    // ----------------------------------------------------
    
    // The keyword `native` is required.
    // The function name `add` must match the "add" in the Rust registration.
    // The signature `(u64, u64): u64` must match the type and order in the Rust implementation.
    native public fun add(a: u64, b: u64): u64;

    
    // ----------------------------------------------------
    // Write a wrapper function for external calls
    // ----------------------------------------------------
    
    // This is a regular Move function that can be called by external entry functions.
    public fun get_sum(a: u64, b: u64): u64 {
        // Call your native function here
        add(a, b)
    }
    
    // ----------------------------------------------------
    // Test entry function
    // ----------------------------------------------------
    
    public entry fun debug_test_add() {
        let result = get_sum(100, 200);
        
        // Ensure you have permission to call std::debug::print, usually works on the local testnet
        std::debug::print(&result);
    }
}
