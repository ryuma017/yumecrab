use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use super::{models::KaiinTable, Database};
use crate::domain::{NewUser, User};

#[async_trait]
pub trait UsersRepository: Interface {
    async fn insert_user(&self, user: NewUser) -> Result<User, anyhow::Error>;
}

#[derive(Component)]
#[shaku(interface = UsersRepository)]
pub struct UsersRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn insert_user(&self, new_user: NewUser) -> Result<User, anyhow::Error> {
        let kaiin: KaiinTable = new_user.try_into()?;
        let kaiin_id = sqlx::query_as!(
            KaiinTable,
            r#"
            INSERT INTO kaiin (adana, mail_address, password, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            kaiin.adana,
            kaiin.mail_address,
            kaiin.password,
            kaiin.created_at,
            kaiin.updated_at
        )
        .execute(self.database.pool())
        .await?
        .last_insert_id();

        Ok(kaiin.try_into().map(|user: User| user.set_id(kaiin_id))?)
    }
}
