use ethers::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletItem {
    address: String,
    private_key: String,
    proxy: Option<String>,
    eligibility_checked: bool,
    eligible: bool,
    value: i64,
}

impl WalletItem {
    fn get_local_wallet(private_key: &String) -> eyre::Result<LocalWallet> {
        let wallet: LocalWallet = private_key.parse()?;
        Ok(wallet)
    }

    pub fn to_local_wallet(&self) -> eyre::Result<LocalWallet> {
        let wallet: LocalWallet = self.private_key.parse()?;
        Ok(wallet)
    }

    pub fn new(private_key: String, proxy: Option<String>) -> eyre::Result<Self> {
        let wallet = Self::get_local_wallet(&private_key)?;
        let address = format!("{:?}", wallet.address());

        Ok(Self {
            address,
            private_key,
            proxy,
            eligibility_checked: false,
            eligible: false,
            value: 0,
        })
    }

    pub fn set_eligible(&mut self, is_eligible: bool) {
        self.eligible = is_eligible
    }

    pub fn get_eligibility_checked(&self) -> bool {
        self.eligibility_checked
    }

    pub fn set_eligibility_checked(&mut self) {
        self.eligibility_checked = true
    }

    pub fn get_proxy(&self) -> Option<&String> {
        self.proxy.as_ref()
    }

    pub fn get_address(&self) -> &String {
        &self.address
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value
    }
}
