use sea_orm_migration::{prelude::* };

use argon2::{
    password_hash::PasswordHasher ,
    Argon2
};
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261017_064037_create_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_users_table(manager).await?;
        Self::add_user(manager, "admin", "asdf").await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_users_table(_manager).await
    }
}
impl Migration{
    async fn create_users_table(manager: &SchemaManager<'_>)->Result<(), DbErr>{
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Users::UserName)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Users::EncryptedPassword)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Users::IsActive)
                        .boolean()
                        .not_null()
                        .default(true)
                    ).to_owned(),
            )
            .await
    }
    async fn drop_users_table(manager: &SchemaManager<'_>)->Result<(), DbErr>{
        manager
            .drop_table(
                Table::drop()
                    .table(Users::Table)
                    .to_owned(),
        ).await
    }

    async fn add_user( manager: &SchemaManager<'_>, username: &str, password: &str)->Result<(), DbErr>{
        let argon2 = Argon2::default();
        let password_hash= argon2
            .hash_password(password.as_bytes() )
            .map_err( |e| DbErr::Custom(format!("Failed to hash password {:?}", e)))?
            .to_string();

        let insert = Query::insert()
            .into_table(Users::Table)
            .columns([Users::UserName, Users::EncryptedPassword,])
            .select_from(
                Query::select()
                    .expr(Expr::val(username))
                    .expr(Expr::val(password_hash))
                    .to_owned()
            ).map_err(|e|DbErr::Custom(format!("{:?}", e)))?
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }
}


#[derive(DeriveIden)]
enum Users{
    Table,
    Id,
    UserName,
    EncryptedPassword,
    IsActive,
}
