use dioxus::prelude::*;

#[cfg(feature = "server")]
pub use helper::IntoMyError;
#[cfg(feature = "server")] mod helper {
use crate::server::error::MyError;

/// we always want to convert a `Result` to `Result<T, MyError>` first and than to `Result<T, ServerFnErrorErr>, to avoid leaking private Server data.
/// The best thing would be to impl `From<E> for ServerFnErrorErr where E: Into<MyError>` so we can use `?` normal again.
/// Sadly this is not allowed.
/// So we create at least a helper so we can use `foo.into_my_error()?` instead of `Into::<MyError>::into(foo)?` at least
pub trait IntoMyError<T>  {
    fn into_my_error(self) ->  Result<T,MyError>;
}


impl<T,E> IntoMyError<T> for Result<T,E>
where E: Into<MyError>
{
    fn into_my_error(self) -> Result<T, MyError> {
        self.map_err(|err| {
            err.into()
        })
    }
}
}

#[server]
pub async fn check_token(token: String) -> Result<bool, ServerFnError> {
    Ok(crate::server::sql::check_token(&token).await.into_my_error()?)
}


#[server]
pub async fn import_telegram_packs(token: String, packs: Vec<String>) -> Result<(), ServerFnError> {
    println!("todo import packs: {packs:?}");
    Ok(())
}