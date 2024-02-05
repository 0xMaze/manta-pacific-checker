use crate::wallet_item::WalletItem;
use ethers::prelude::*;
use reqwest::{Client, Proxy};
use serde_json::json;

const MESSAGE: &str = "Please sign to confirm whether to operate the Treasure Boxes and NFTs. Operations may include: Checking Eligibility, Opening Treasure Boxes, and Claiming Treasure NFTs.";
const CHECK_ELIGIBILITY_URL: &str = "https://np-api.newparadigm.manta.network/getPoints";

pub struct Checker {}

impl Checker {
    fn build_request_client(wallet_item: &WalletItem) -> eyre::Result<Client> {
        let mut client_builder = Client::builder();
        let _ = match wallet_item.get_proxy() {
            Some(proxy_url) => {
                let proxy = Proxy::all(proxy_url)?;
                client_builder = client_builder.proxy(proxy);
                Some(proxy_url)
            }
            None => None,
        };
        Ok(client_builder.build()?)
    }
    async fn send_post_request(
        url: &str,
        payload: serde_json::Value,
        wallet_item: &WalletItem,
    ) -> eyre::Result<serde_json::Value> {
        let request_client = Self::build_request_client(wallet_item)?;
        let response = request_client.post(url).json(&payload).send().await?;

        let status = response.status();
        if status.is_success() {
            let json_response = response.json::<serde_json::Value>().await?;
            Ok(json_response)
        } else {
            let err_msg = response.text().await?;
            eyre::bail!("Request failed with status code {}: {}", status, err_msg);
        }
    }

    pub async fn check_eligibility(wallet_item: &WalletItem) -> eyre::Result<(bool, i64)> {
        let payload = json!({
            "address": wallet_item.get_address(),
            "dot_sig": "",
            "eth_sig": Self::get_signature(wallet_item).await?,
            "polkadot_address": "",
        });
        let response = Self::send_post_request(CHECK_ELIGIBILITY_URL, payload, wallet_item).await?;
        if let Some(total_score) = response["data"]["total_score"].as_i64() {
            let is_eligible = total_score > 0;
            Ok((is_eligible, total_score))
        } else {
            Err(eyre::eyre!("Total score not found in the response"))
        }
    }

    async fn get_signature(wallet_item: &WalletItem) -> eyre::Result<String> {
        let wallet = wallet_item.to_local_wallet()?;
        let signature = format!("0x{}", wallet.sign_message(MESSAGE).await?);
        Ok(signature)
    }
}
