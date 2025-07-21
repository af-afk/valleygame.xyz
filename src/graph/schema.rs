use async_graphql::{Enum, Object, Result, SimpleObject};

use deadpool_postgres::Pool as PostgresPool;

use std::sync::LazyLock;

static DB: LazyLock<PostgresPool> = LazyLock::new(|| crate::db::create_db());

/// Status of a deposit, indicating whether it's available or consumed
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum DepositStatus {
    /// Unspent amount that was tracked as deposited by the user.
    Unspent,

    /// Amount that was withdrawn, and consumed.
    Withdrawn,

    /// Amount that was consumed during normal gameplay, and then
    /// converted into a new amount as a result of the wins and losses
    /// the player racked up.
    AfterGame,
}

/// Deposit made by a user, possibly including a LayerZero amount,
/// which can be used for optimistic bridging from several chains.
#[derive(SimpleObject, Debug, Clone)]
pub struct Deposit {
    pub lz_tx: Option<String>,

    /// Recorded in the database ticket ID from the deposits contract.
    pub ticket_id: String,

    /// Status of the deposit, which indicates either a consumed coin, or
    /// one that can be spent.
    pub status: DepositStatus,

    /// Amount that this deposit is worth at present.
    pub amount: String,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Lobby {
    /// Random nonce that indicates the soon to be created Game id.
    pub random_nonce: String,

    /// More information on the game that this lobby is for.
    pub game: Game,

    /// Locked in players who can play the game.
    pub locked_in_players: Vec<String>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Game {
    pub creator: String,
    pub round_limit: i32,
    pub bip_points: String,
    pub fixed_buyin: String,
    pub whitelisted_ids: Vec<String>,
    pub random_nonce: String,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Prediction {
    pub amount: String,
    pub addr: String,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Round {
    pub ongoing: bool,
    pub predictions: Vec<Prediction>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Ongoing {
    pub rounds: Vec<Round>,
    pub game: Game,
}

pub struct Query;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub auth: Option<String>,
}

impl Default for AuthContext {
    fn default() -> Self {
        AuthContext { auth: None }
    }
}

#[Object]
impl Query {
    async fn lobbies(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Lobby>> {
        let AuthContext { auth } = ctx.data::<AuthContext>().unwrap();
        auth.clone().unwrap();
        DB.get()
            .await
            .unwrap()
            .query("SELECT 1", &[])
            .await
            .unwrap();
        Ok(vec![])
    }

    async fn ongoing(&self) -> Result<Vec<Ongoing>> {
        Ok(vec![])
    }

    async fn deposits(&self, _addr: String) -> Result<Vec<Deposit>> {
        Ok(vec![])
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register_game(&self) -> Result<bool> {
        Ok(true)
    }
}
