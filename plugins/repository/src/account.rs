use log::error;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, QuerySelect};
use xauthenticator_entity::account::{ActiveModel, Entity as AccountEntity, Model};
use xauthenticator_entity::PageParam;
use xauthenticator_error::CommonError;

// List accounts with basic page-based pagination
pub async fn list_accounts<C>(db: &C, param: PageParam) -> Result<Vec<Model>, CommonError>
where
    C: ConnectionTrait,
{
    let accounts = AccountEntity::find()
        .offset((param.current as u64).saturating_mul(param.size as u64))
        .limit(param.size as u64)
        .all(db)
        .await
        .map_err(|e| {
            error!("Failed to query accounts: {:?}", e);
            CommonError::DatabaseError(e)
        })?;

    Ok(accounts)
}

// Insert a new account record
pub async fn add_account<C>(db: &C, account: ActiveModel) -> Result<Model, CommonError>
where
    C: ConnectionTrait,
{
    let model = account
        .insert(db)
        .await
        .map_err(|e| {
            error!("Failed to insert account: {:?}", e);
            CommonError::DatabaseError(e)
        })?;
    Ok(model)
}

// Remove an account by id, returns number of affected rows
pub async fn remove_account<C>(db: &C, id: String) -> Result<u64, CommonError>
where
    C: ConnectionTrait,
{
    let res = AccountEntity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| {
            error!("Failed to delete account: {:?}", e);
            CommonError::DatabaseError(e)
        })?;
    Ok(res.rows_affected)
}

// Update an existing account using ActiveModel (must contain primary key)
pub async fn update_account<C>(db: &C, account: ActiveModel) -> Result<Model, CommonError>
where
    C: ConnectionTrait,
{
    let model = account
        .update(db)
        .await
        .map_err(|e| {
            error!("Failed to update account: {:?}", e);
            CommonError::DatabaseError(e)
        })?;
    Ok(model)
}
