/// File: sources/dex.move
/// Deploy to: 0x1
module aptos_framework::dex {

    native public fun create_pool<X: key, Y: key>(): u64;
    native public fun get_pool<X: key, Y: key>(): u64;

    // ========================================
    // 2. Wrapper functions (External calls)
    // ========================================

    public fun new_pool<X: key, Y: key>() {
        create_pool<X, Y>();
    }

    public fun pool_state<X: key, Y: key>(): u64 {
        get_pool<X, Y>()
    }

    // ========================================
    // 3. Test entry points
    // ========================================

    public entry fun test_dex() {
        // Create Pool<CoinA, CoinB>
        new_pool<CoinA, CoinB>();

        let pool_state = pool_state<CoinA, CoinB>();
    }

    public entry fun test_create_pool() {
        new_pool<CoinA, CoinB>();
    }

    /// Second transaction: Query pool state
    public entry fun test_get_pool() {
        let state = pool_state<CoinA, CoinB>();
    }
    
    // ========================================
    // 4. Simulate Coin types
    // ========================================

    struct CoinA has key {}
    struct CoinB has key {}
}
