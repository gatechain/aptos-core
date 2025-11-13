module native_test::calculator {
    use aptos_framework::add;

    public entry fun compute(x: u64, y: u64) {
        let result = add::add(x, y);
        std::debug::print(&result);
    }
}