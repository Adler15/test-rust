use crate::config::Conf;
use crate::repo::{Pool, Postgres};
use crate::router::route;
pub struct Service {
    pub conf: Conf,
    pub repo: Pool<Postgres>,
}

impl Service {
    pub fn new(conf: Conf, repo: Pool<Postgres>) -> Self {
        Self { conf, repo }
    }

    pub async fn start(&self) {
        route(&self).await;
    }
}
