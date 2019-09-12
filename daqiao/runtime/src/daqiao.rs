
extern crate qrml_tokens as tokens;

use support::{decl_module, decl_storage, decl_event, StorageMap, dispatch::Result, ensure};
use runtime_primitives::traits::CheckedSub;
use parity_codec::{Encode, Decode};
use system::ensure_signed;
use rstd::prelude::*;


pub type TokenId<T> = <T as tokens::Trait>::TokenId;
pub type ChainId = u32;
pub type Hash = primitives::H256;

pub trait Trait: system::Trait + tokens::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct PledgeInfo <U, V>{
  chain_id: ChainId,
  ext_txid: Hash,
  account_id: U,
  pledge_amount: V,
  can_withdraw: bool,
  withdraw_history: Vec<Hash>
}

decl_storage! {
  trait Store for Module<T: Trait> as Daqiao {
    // 外链id => token id
    ChainToken get(chain_token): map ChainId => Option<TokenId<T>>;
    
    // 外链质押txid => PledgeInfo
    PledgeRecords get(pledge_records): map T::Hash => PledgeInfo<T::AccountId, T::TokenBalance>;
  }
}

decl_module! {
  /// The module declaration.
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn deposit_event<T>() = default;

    // 关联ChainId和TokenId
    pub fn register(origin, chain_id: ChainId, token_id: TokenId<T>) -> Result {
      Self::_register(origin, chain_id, token_id)
    }
    
    // 质押
    // TODO: 尝试自动验证
    pub fn pledge(origin, chain_id: ChainId, ext_txid: Hash, value: T::TokenBalance) -> Result {
      Self::_pledge(origin, chain_id, ext_txid, value)
    }
    
    // 提现
    pub fn withdraw(origin, chain_id: ChainId, ext_txid: Hash, value: T::TokenBalance) -> Result {
      Self::_withdraw(origin, chain_id, ext_txid, value)
    }
  }
}

decl_event!(
  pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Balance = <T as tokens::Trait>::TokenBalance {
    Pledged(ChainId, Hash, AccountId, Balance),
    Withdrawn(ChainId, Hash, AccountId, Balance),
  }
);

impl<T: Trait> Module<T> {
  
  fn _register(origin: T::Origin, chain_id: ChainId, token_id: TokenId<T>) -> Result {
    // TODO sudo权限验证
    let _ = ensure_signed(origin)?;
    ensure!(!<ChainToken<T>>::exists(chain_id.clone()), "ChainId already exists.");
    <ChainToken<T>>::insert(chain_id, token_id);
    Ok(())
  }
  
  
  pub fn _pledge(origin: T::Origin, chain_id: ChainId, ext_txid: Hash, value: T::TokenBalance) -> Result {
    let sender = ensure_signed(origin)?;
    let token_id = match Self::chain_token(chain_id.clone()) {
      Some(t) => t,
      None => return Err("Chain id not exists.")
    };
    
    // TODO 验证外链ex_txid
    
    <tokens::Module<T>>::mint(token_id.clone(), sender.clone(), value)?;
    Self::deposit_event(RawEvent::Pledged(chain_id, ext_txid, sender, value));
    Ok(())
  }
  
  pub fn _withdraw(origin: T::Origin, chain_id: ChainId, ext_txid: Hash, value: T::TokenBalance) -> Result {
    let sender = ensure_signed(origin) ?;
    let token_id = match Self::chain_token(chain_id.clone()) {
      Some(t) => t,
      None => return Err("Chain id not exists.")
    };
  
    // 检查余额是否足够
    let balance = <tokens::Module<T>>::balance_of(&(token_id.clone(), sender.clone()));
    let _ = balance.checked_sub(&value).ok_or("Not sufficient balance for withdraw")?;
    
    // TODO 提现到外链
  
    <tokens::Module<T>>::burn(token_id, sender.clone(), value)?;
    Self::deposit_event(RawEvent::Withdrawn(chain_id, ext_txid, sender, value));
    Ok(())
  }
}

/// tests for this module
#[cfg(test)]
mod tests {
  use super::*;

  use runtime_io::with_externalities;
  use primitives::{H256, Blake2Hasher};
  use support::{impl_outer_origin, assert_ok};
  use runtime_primitives::{
    BuildStorage,
    traits::{BlakeTwo256, IdentityLookup},
    testing::{Digest, DigestItem, Header}
  };

  impl_outer_origin! {
    pub enum Origin for Test {}
  }

  // For testing the module, we construct most of a mock runtime. This means
  // first constructing a configuration type (`Test`) which `impl`s each of the
  // configuration traits of modules we want to use.
  #[derive(Clone, Eq, PartialEq)]
  pub struct Test;
  impl system::Trait for Test {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Digest = Digest;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type Log = DigestItem;
  }
  impl Trait for Test {
    type Event = ();
  }
  type TemplateModule = Module<Test>;

  // This function basically just builds a genesis storage key/value store according to
  // our desired mockup.
  fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
  }

  #[test]
  fn it_works_for_default_value() {
    with_externalities(&mut new_test_ext(), || {
      // Just a dummy test for the dummy funtion `do_something`
      // calling the `do_something` function with a value 42
      assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
      // asserting that the stored value is equal to what we stored
      assert_eq!(TemplateModule::something(), Some(42));
    });
  }
}
