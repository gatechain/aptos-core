
<a id="0x1_dex"></a>

# Module `0x1::dex`

File: sources/dex.move
Deploy to: 0x1


-  [Resource `CoinA`](#0x1_dex_CoinA)
-  [Resource `CoinB`](#0x1_dex_CoinB)
-  [Function `create_pool`](#0x1_dex_create_pool)
-  [Function `get_pool`](#0x1_dex_get_pool)
-  [Function `new_pool`](#0x1_dex_new_pool)
-  [Function `pool_state`](#0x1_dex_pool_state)
-  [Function `test_dex`](#0x1_dex_test_dex)
-  [Function `test_create_pool`](#0x1_dex_test_create_pool)
-  [Function `test_get_pool`](#0x1_dex_test_get_pool)


<pre><code></code></pre>



<a id="0x1_dex_CoinA"></a>

## Resource `CoinA`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_CoinA">CoinA</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_dex_CoinB"></a>

## Resource `CoinB`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_CoinB">CoinB</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_dex_create_pool"></a>

## Function `create_pool`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_create_pool">create_pool</a>&lt;X: key, Y: key&gt;(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_create_pool">create_pool</a>&lt;X: key, Y: key&gt;(): u64;
</code></pre>



</details>

<a id="0x1_dex_get_pool"></a>

## Function `get_pool`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pool">get_pool</a>&lt;X: key, Y: key&gt;(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pool">get_pool</a>&lt;X: key, Y: key&gt;(): u64;
</code></pre>



</details>

<a id="0x1_dex_new_pool"></a>

## Function `new_pool`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_new_pool">new_pool</a>&lt;X: key, Y: key&gt;()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_new_pool">new_pool</a>&lt;X: key, Y: key&gt;() {
    <a href="dex.md#0x1_dex_create_pool">create_pool</a>&lt;X, Y&gt;();
}
</code></pre>



</details>

<a id="0x1_dex_pool_state"></a>

## Function `pool_state`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_pool_state">pool_state</a>&lt;X: key, Y: key&gt;(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_pool_state">pool_state</a>&lt;X: key, Y: key&gt;(): u64 {
    <a href="dex.md#0x1_dex_get_pool">get_pool</a>&lt;X, Y&gt;()
}
</code></pre>



</details>

<a id="0x1_dex_test_dex"></a>

## Function `test_dex`



<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_dex">test_dex</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_dex">test_dex</a>() {
    // Create Pool&lt;<a href="dex.md#0x1_dex_CoinA">CoinA</a>, <a href="dex.md#0x1_dex_CoinB">CoinB</a>&gt;
    <a href="dex.md#0x1_dex_new_pool">new_pool</a>&lt;<a href="dex.md#0x1_dex_CoinA">CoinA</a>, <a href="dex.md#0x1_dex_CoinB">CoinB</a>&gt;();

    <b>let</b> pool_state = <a href="dex.md#0x1_dex_pool_state">pool_state</a>&lt;<a href="dex.md#0x1_dex_CoinA">CoinA</a>, <a href="dex.md#0x1_dex_CoinB">CoinB</a>&gt;();
}
</code></pre>



</details>

<a id="0x1_dex_test_create_pool"></a>

## Function `test_create_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_create_pool">test_create_pool</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_create_pool">test_create_pool</a>() {
    <a href="dex.md#0x1_dex_new_pool">new_pool</a>&lt;<a href="dex.md#0x1_dex_CoinA">CoinA</a>, <a href="dex.md#0x1_dex_CoinB">CoinB</a>&gt;();
}
</code></pre>



</details>

<a id="0x1_dex_test_get_pool"></a>

## Function `test_get_pool`

Second transaction: Query pool state


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_get_pool">test_get_pool</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_test_get_pool">test_get_pool</a>() {
    <b>let</b> state = <a href="dex.md#0x1_dex_pool_state">pool_state</a>&lt;<a href="dex.md#0x1_dex_CoinA">CoinA</a>, <a href="dex.md#0x1_dex_CoinB">CoinB</a>&gt;();
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
