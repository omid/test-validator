use crate::models::User;
use diesel::PgConnection;
use validator::ValidationError;

pub fn validate<F>(email: &str, args: (&PgConnection, F)) -> Result<(), ValidationError>
where
    F: FnOnce(&str, &PgConnection) -> Result<Option<User>, diesel::result::Error>,
{
    let (conn, func) = args;

    let res = func(email, conn);

    match res {
        Ok(user) => {
            if user.is_none() {
                return Err(ValidationError::new("email"));
            }
        }
        Err(_) => {
            return Err(ValidationError::new("email"));
        }
    }

    Ok(())
}
