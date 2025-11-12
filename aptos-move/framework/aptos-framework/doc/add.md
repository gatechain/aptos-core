
<a id="0x1_add"></a>

# Module `0x1::add`



-  [Function `add`](#0x1_add_add)
-  [Function `get_sum`](#0x1_add_get_sum)
-  [Function `debug_test_add`](#0x1_add_debug_test_add)


<pre><code><b>use</b> <a href="../../aptos-stdlib/doc/debug.md#0x1_debug">0x1::debug</a>;
</code></pre>



<a id="0x1_add_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="add.md#0x1_add">add</a>(a: u64, b: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="add.md#0x1_add">add</a>(a: u64, b: u64): u64;
</code></pre>



</details>

<a id="0x1_add_get_sum"></a>

## Function `get_sum`



<pre><code><b>public</b> <b>fun</b> <a href="add.md#0x1_add_get_sum">get_sum</a>(a: u64, b: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="add.md#0x1_add_get_sum">get_sum</a>(a: u64, b: u64): u64 {
    // Call your <b>native</b> function here
    <a href="add.md#0x1_add">add</a>(a, b)
}
</code></pre>



</details>

<a id="0x1_add_debug_test_add"></a>

## Function `debug_test_add`



<pre><code><b>public</b> entry <b>fun</b> <a href="add.md#0x1_add_debug_test_add">debug_test_add</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="add.md#0x1_add_debug_test_add">debug_test_add</a>() {
    <b>let</b> result = <a href="add.md#0x1_add_get_sum">get_sum</a>(100, 200);

    // Ensure you have permission <b>to</b> call std::debug::print, usually works on the <b>local</b> testnet
    std::debug::print(&result);
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
