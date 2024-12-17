
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key_vault_name = env::var("KEY_VAULT_NAME")?;
    let secret_name = env::args().nth(1).expect("Usage: cargo run <secret-name>");

    let credential = DefaultAzureCredential::default();
    let vault_url = format!("https://{}.vault.azure.net", key_vault_name);
    let client = SecretClient::new(&vault_url, credential)?;

    let secret = client.get_secret(&secret_name).await?;
    println!("{}", secret.value);

    Ok(())
}