use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::fmt;

use serde_bytes::ByteBuf;

pub type Subaccount = [u8; 32];

pub type Balance = u128;

pub type BlockIndex = Nat;

pub const DEFAULT_SUBACCOUNT: &Subaccount = &[0; 32];

pub const TOTAL_SUPPLY: u32 = 100_000_000;

pub const DECIMALS: u8 = 8u8;

#[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

impl Account {
    #[inline]
    pub fn effective_subaccount(&self) -> &Subaccount {
        self.subaccount.as_ref().unwrap_or(DEFAULT_SUBACCOUNT)
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.effective_subaccount() == other.effective_subaccount()
    }
}

impl Eq for Account {}

impl std::cmp::PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Account {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.owner.cmp(&other.owner).then_with(|| {
            self.effective_subaccount()
                .cmp(other.effective_subaccount())
        })
    }
}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.effective_subaccount().hash(state);
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subaccount {
            None => write!(f, "{}", self.owner),
            Some(subaccount) => write!(f, "0x{}.{}", hex::encode(&subaccount[..]), self.owner),
        }
    }
}

impl From<Principal> for Account {
    fn from(owner: Principal) -> Self {
        Self {
            owner,
            subaccount: None,
        }
    }
}


#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TransferArg {
    #[serde(default)]
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    #[serde(default)]
    pub fee: Option<Balance>,
    #[serde(default)]
    pub created_at_time: Option<u64>,
    #[serde(default)]
    pub memo: Option<Memo>,
    pub amount: Balance,
}

pub const MAX_MEMO_LENGTH: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct MemoTooLarge(usize);

impl fmt::Display for MemoTooLarge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Memo field is {} bytes long, max allowed length is {}",
            self.0, MAX_MEMO_LENGTH
        )
    }
}

#[derive(
    Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default,
)]
#[serde(transparent)]
pub struct Memo(#[serde(deserialize_with = "deserialize_memo_bytes")] pub ByteBuf);

fn deserialize_memo_bytes<'de, D>(d: D) -> Result<ByteBuf, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;
    let bytes = ByteBuf::deserialize(d)?;
    let memo = Memo::try_from(bytes).map_err(D::Error::custom)?;
    Ok(memo.into())
}

impl From<[u8; MAX_MEMO_LENGTH]> for Memo {
    fn from(memo: [u8; MAX_MEMO_LENGTH]) -> Self {
        Self(ByteBuf::from(memo.to_vec()))
    }
}

impl From<u64> for Memo {
    fn from(num: u64) -> Self {
        Self(ByteBuf::from(num.to_be_bytes().to_vec()))
    }
}

impl TryFrom<ByteBuf> for Memo {
    type Error = MemoTooLarge;

    fn try_from(b: ByteBuf) -> Result<Self, MemoTooLarge> {
        if b.len() > MAX_MEMO_LENGTH {
            return Err(MemoTooLarge(b.len()));
        }
        Ok(Self(b))
    }
}

impl TryFrom<Vec<u8>> for Memo {
    type Error = MemoTooLarge;

    fn try_from(v: Vec<u8>) -> Result<Self, MemoTooLarge> {
        Self::try_from(ByteBuf::from(v))
    }
}

impl From<Memo> for ByteBuf {
    fn from(memo: Memo) -> Self {
        memo.0
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransferError {
    BadFee { expected_fee: Balance },
    BadBurn { min_burn_amount: Balance },
    InsufficientFunds { balance: Balance },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: BlockIndex },
    GenericError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StandardRecord {
    pub name: String,
    pub url: String,
}
