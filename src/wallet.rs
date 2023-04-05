use super::*;

pub struct Wallet {
  _private: (),
}

impl Wallet {
  pub fn load(options: &Options) -> Result<Self> {
    options.bitcoin_rpc_client_for_wallet_command(false)?;

    Ok(Self { _private: () })
  }
}
