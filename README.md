# Greeter indexer on Fuel

_Please refer to the [documentation website](https://docs.envio.dev) for a thorough guide on all Envio indexer features_

## Local usage

1. Clone the repository

   ```sh
   git clone git@github.com:enviodev/fuel-greeter.git
   ```

2. Open it locally

   ```sh
   cd fuel-greeter
   ```

3. Install dependencies (requires [pnpm@9](https://pnpm.io/))

   ```sh
   pnpm i
   ```

4. Run envio

   ```sh
   pnpm dev
   ```

5. Verify it's working correctly by checking the Hasura:
   1. Open http://localhost:8080
   2. Enter admin-secret `testing`
