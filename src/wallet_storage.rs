use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::checker::Checker;
use crate::wallet_item::WalletItem;
use ethers::core::rand;
use ethers::core::rand::prelude::SliceRandom;
use itertools::{EitherOrBoth::*, Itertools};
use log::info;
use serde::{Deserialize, Serialize};

const STORAGE_FILE_PATH: &str = "data/storage.json";
const PRIVATE_KEYS_FILE_PATH: &str = "data/private_keys.txt";
const PROXIES_FILE_PATH: &str = "data/proxies.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletStorage {
    storage: Vec<WalletItem>,
}

impl WalletStorage {
    pub fn new(wallet_items: Vec<WalletItem>) -> Self {
        Self {
            storage: wallet_items,
        }
    }

    pub fn get_storage(&self) -> &Vec<WalletItem> {
        &self.storage
    }

    pub fn get_unchecked_wallet(&mut self) -> Option<&mut WalletItem> {
        let mut rng = rand::thread_rng();
        self.storage
            .choose_mut(&mut rng)
            .filter(|wallet| !wallet.get_eligibility_checked())
    }

    fn has_unchecked_wallets(&self) -> bool {
        self.storage
            .iter()
            .any(|wallet| !wallet.get_eligibility_checked())
    }

    pub fn load_from_json() -> eyre::Result<Self> {
        let storage_file = File::open(STORAGE_FILE_PATH)?;
        let reader = BufReader::new(storage_file);
        let wallets = serde_json::from_reader(reader)?;
        Ok(Self::new(wallets))
    }

    pub fn save_to_json(&self) -> eyre::Result<()> {
        let storage_file = File::create(STORAGE_FILE_PATH)?;
        let mut writer = BufWriter::new(storage_file);
        serde_json::to_writer_pretty(&mut writer, self.get_storage())?;
        writer.flush()?;
        Ok(())
    }

    pub fn create() -> eyre::Result<Self> {
        let pks = Self::read_txt(PRIVATE_KEYS_FILE_PATH)?;
        let proxies = Self::read_txt(PROXIES_FILE_PATH)?;
        let wallet_items = pks
            .iter()
            .zip_longest(proxies)
            .map(|items| match items {
                Both(pk, proxy) => WalletItem::new(pk.to_string(), Some(proxy)),
                Left(pk) => WalletItem::new(pk.to_string(), None),
                Right(proxy) => Err(eyre::eyre!("Missing private key for proxy `{}`", proxy)),
            })
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Self::new(wallet_items))
    }

    fn read_txt(file_path: &str) -> eyre::Result<Vec<String>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .collect();
        Ok(lines)
    }

    pub async fn check_wallets() -> eyre::Result<()> {
        let mut storage = Self::load_from_json()?;
        while storage.has_unchecked_wallets() {
            let wallet_item = storage.get_unchecked_wallet().unwrap();
            info!("`{}` | Checking eligibility", wallet_item.get_address());
            let (eligible, value) = Checker::check_eligibility(wallet_item).await?;
            wallet_item.set_eligibility_checked();
            wallet_item.set_eligible(eligible);
            wallet_item.set_value(value);
            info!(
                "`{}` | Is eligible: {}. Value: {}",
                wallet_item.get_address(),
                eligible,
                value
            );
            storage.save_to_json()?;
        }
        info!("No more unchecked wallets left");
        Ok(())
    }
}
