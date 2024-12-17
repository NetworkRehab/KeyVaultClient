# Azure Key Vault CLI

A simple CLI tool written in Rust to retrieve secrets from Azure Key Vault and output them in plain text.

Used for testing. **Do not use in production.**

## Prerequisites

- **Rust** programming language installed.
- An **Azure account** with access to an Azure Key Vault.
- The `KEY_VAULT_NAME` environment variable set to your Key Vault's name.
- **Azure CLI** installed and logged in with appropriate permissions.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/NetworkRehab/KeyVaultClient.git
cd KeyVaultClient
```

Build the application using Cargo:

```bash
cargo build --release
```

## Usage

Set the `KEY_VAULT_NAME` environment variable:

```bash
export KEY_VAULT_NAME=your-key-vault-name
```

Run the CLI tool with the secret name as an argument:

```bash
./target/release/azure_keyvault_cli <secret-name>
```

Example:

```bash
./target/release/azure_keyvault_cli my-secret
```

This command retrieves the value of `my-secret` from Azure Key Vault and prints it in plain text.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
