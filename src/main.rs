use crate::wallet_storage::WalletStorage;
use log::info;
use std::io;

mod checker;
mod wallet_item;
mod wallet_storage;

async fn menu() -> eyre::Result<()> {
    info!(
        r"

                  __  '              _ ,.,              ,.,  ' ‘                       _ ‘
            ,·:'´/::::/'`;·.,        '/:::::/`,           /:::/';       /:¯:'`:*:^:*:´':¯::/'`;‘
        .:´::::/::::/:::::::`;     /;: :;/:::\         /;:;/:'i‘      /:: :: : : : : : :::/::'/
       /:;:· '´ ¯¯'`^·-;::::/' ‘  ,´     `;::';       ,´   'i:'i     ,´¯ '` * ^ * ´' ¯   '`;/    ‘
      /·´           _   '`;/‘     i        \::',      ,:    'i:';    '`,                  ,·'   '
     'i            ;::::'`;*       ;         ';::\ .,_';     ';:'i'      '`*^*'´;       .´         ‘
      `;           '`;:::::'`:,    ';         ';::/::::';     ;':;            .´     .'      _   ' ‘
        `·,           '`·;:::::';   \          \/::::;'      i:/'°        .´      ,'´~:~/:::/`:,
      ,~:-'`·,           `:;::/'    '\          '`~'´     ,'/          .´      ,'´::::::/:::/:::'i‘
     /:::::::::';           ';/        \                  /          ,'        '*^~·~*'´¯'`·;:/
   ,:~·- . -·'´          ,'´           '`,             ;'           /                        ,'/
   '`·,               , ·'´                `·.,    ,.·´            ';                      ,.´
        '`*^·–·^*'´'           ‘               ¯         °         '`*^~–––––-·~^'´

        ___  _  _ ____   / ____ _   _ ___  _ _        _  _     ___  ____ _  _ ____ _  _ ____
         |   |\/| |___  /  [__   \_/  |__] | |        |  |       /  |__| |_/  |  | |\ | |___
         |  .|  | |___ /   ___]   |   |__] | |___ ___  \/  ___  /__ |  | | \_ |__| | \| |___

1. [DATABASE] Создать базу данных | Create database
2. [CHECKER] Чекер кошельков | Wallets checker
"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let choice = input.trim().to_lowercase();
    match choice.as_str() {
        "1" => {
            let wallet_storage = WalletStorage::create()?;
            wallet_storage.save_to_json()?;
            info!("Database saved!");
        }
        "2" => {
            WalletStorage::check_wallets().await?;
        }
        _ => {
            todo!()
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();
    menu().await?;
    Ok(())
}
