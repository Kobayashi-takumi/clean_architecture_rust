use crate::shared::error::{Error, Result};
use log::error;
use sqlx::{
    postgres::{PgPool, Postgres},
    Transaction,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Tx<T> = Transaction<'static, T>;
pub type ArcTx<T> = Arc<Mutex<Tx<T>>>;
pub type PgTx = ArcTx<Postgres>;

#[derive(Debug, Clone)]
pub struct PgTransaction {
    tx: PgTx,
}

impl PgTransaction {
    fn new(tx: Tx<Postgres>) -> Self {
        Self {
            tx: Arc::new(Mutex::new(tx)),
        }
    }
    pub async fn begin(pool: &PgPool) -> Result<Self> {
        let tx = pool.begin().await.map_err(|e| {
            error!("{}", e);
            Error::from(e)
        })?;
        Ok(Self::new(tx))
    }
    pub fn get(&self) -> &PgTx {
        &self.tx
    }
    pub async fn commit(self) -> Result<()> {
        let tx = Arc::try_unwrap(self.tx).map_err(|e| {
            error!("into: {e:?}");
            Error::Unknown
        })?;
        tx.into_inner().commit().await.map_err(|e| {
            error!("{}", e);
            e.into()
        })
    }
    pub async fn rollback(self) -> Result<()> {
        let tx = Arc::try_unwrap(self.tx).map_err(|e| {
            error!("into2: {e:?}");
            Error::Unknown
        })?;
        tx.into_inner().rollback().await.map_err(|e| {
            error!("{}", e);
            e.into()
        })
    }
}
