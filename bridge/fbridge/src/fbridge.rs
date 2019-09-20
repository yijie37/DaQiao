use rstd::prelude::*;

// TODO: zq bridge to fabric
pub fn getPledgeAmount(_ext_txid: &[u8]) -> Result<u128,()> {
    // TODO 验证外链ex_txid
    Ok(100)
}
pub fn withdraw( _ext_address: &[u8], _value: u128) -> Result<bool,()> {  
    // TODO 提现到外链
    Ok((true))
}