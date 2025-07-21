
use alloc::{vec::Vec, boxed::Box};

use stylus_sdk::alloy_primitives::U256;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum KeyedOrValue<T: BorshDeserialize + BorshSerialize + Clone> {
    Value(Box<T>),
    Keyed(Box<[u8; 64]>),
}

pub type Addr = [u8; 32];

pub type Sig = [u8; 64];


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BU256 {
    pub x: U256
}

impl BorshSerialize for BU256 {
    fn serialize<W: borsh::io::Write>(&self, w: &mut W) -> Result<(), borsh::io::Error> {
        w.write_all(&self.x.to_le_bytes::<32>())
    }
}

impl BorshDeserialize for BU256 {
    fn deserialize_reader<R: borsh::io::Read>(r: &mut R) -> Result<Self, borsh::io::Error> {
        let mut x = [0u8; 32];
        r.read(&mut x)?;
        Ok(Self {
            x: U256::from_le_bytes(x),
        })
    }
}

impl From<U256> for BU256 {
    fn from(x: U256) -> Self {
        Self { x }
    }
}

impl Into<U256> for BU256 {
    fn into(self) -> U256 {
        self.x
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DepositArgs {
    /// The ticket that is available inside the contract, indicating an
    /// unspent balance.
    pub ticket: BU256,
}

/// Deposits the server made available that could be converted to a
/// underlying balance. Might be a deposit a user made with Stripe for
/// example.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ServerDepositArgs {
    pub recipient: Addr,
    pub amount: BU256,
    pub nonce: u32
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AccountAndSig<T: BorshSerialize + BorshDeserialize + Clone> {
    pub sig: [u8; 64],
    pub id: u32,
    pub value: Box<T>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ServerAndSig<T: BorshSerialize + BorshDeserialize + Clone> {
    pub sig: [u8; 64],
    pub value: Box<T>,
}

/// A raw deposit made by a user of the game token, which is then
/// converted to the debt token to be played.
pub type GameDeposit = KeyedOrValue<AccountAndSig<DepositArgs>>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum MintDebtTokenArgs {
    FromDeposit(AccountAndSig<GameDeposit>),
    FromPreviousGame(AccountAndSig<GameConclude>),
    FromServer(ServerAndSig<ServerDepositArgs>),
}

/// The converted instrument that allows a user to spend a token position
/// by consuming the debt token. Either converted from a Deposit, or
/// created arbitrarily by the server, with the users taking on the risk
/// of a bank run.
pub type GameMintDebtToken = KeyedOrValue<MintDebtTokenArgs>;

/// Arguments that constrain the gameplay.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EnterArgs {
    pub round_limit: u32,
    pub bip_points: u32,
    pub fixed_buyin: BU256,
    pub whitelisted_ids: Vec<Addr>,
    pub random_nonce: BU256
}

/// Enters the game, with a signature to prove the user knows the identifier
/// and the rules of the game.
pub type GameEnter = KeyedOrValue<AccountAndSig<(GameMintDebtToken, EnterArgs)>>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PredictArgs {
    pub estimated: BU256,
    pub round: u32,
}

/// Prediction made inside the game loop. The Prediction type can
/// include BeginGame.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum GamePrediction {
    /// We can ellide this type's display to the user, by simply
    /// constructing it on the fly, and only having it here as an
    /// intermediate/bridging type.
    BeginGame(GameEnter),
    Prediction(KeyedOrValue<AccountAndSig<(PredictArgs, GamePrediction)>>),
}

/// The final conclusion of the game. This type aggregates the previous
/// participation of each player in the game. It can be aggregated by
/// anyone, since it's validity is dependent on on the amount of items in
/// the contents.
pub type GameConclude = KeyedOrValue<Vec<GamePrediction>>;

/// Consume the debt token, and return the deposit to the user that they're
/// entitled to here after consuming the conclusion of the game through the
/// GameConclude type and retracing the game steps.
pub type GameWithdraw = ServerAndSig<AccountAndSig<MintDebtTokenArgs>>;
