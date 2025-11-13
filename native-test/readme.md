### 1. Publish a Move Module

You can publish your Move module to the Aptos testnet by running the following command. Replace the `--named-addresses` with the account address generated during the `aptos init` step.

```bash
./target/debug/aptos move publish \
  --package-dir ./native-test \
  --named-addresses native_test=<YOUR_ACCOUNT_ADDRESS> \
  --url http://0.0.0.0:8080 \
  --override-size-check
```

This command will deploy the Move package located in the `./native-test` directory to the Aptos testnet.

### 2. Run a Move Function

Once the module is published, you can invoke a function from it. For example, if you want to call the `compute` function from the `calculator` module, use the following command:

```bash
./target/debug/aptos move run \
  --function-id <YOUR_ACCOUNT_ADDRESS>::calculator::compute \
  --args u64:10 u64:20 \
  --url http://127.0.0.1:8080 \
  --local
```