use thiserror::Error as ThisError;

/// `nvimo`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvimo`'s error type.
#[derive(Clone, Debug, ThisError)]
// TODO: add derive(Eq, PartialEq)
pub enum Error {
    #[error(transparent)]
    Api(#[from] api::Error),

    #[error(transparent)]
    Nvim(#[from] types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] types::conversion::Error),

    #[error(transparent)]
    Serialize(#[from] types::serde::SerializeError),

    #[error(transparent)]
    Deserialize(#[from] types::serde::DeserializeError),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] libuv::Error),

    #[error(transparent)]
    Mlua(#[from] mlua::Error),

    #[error(transparent)]
    Olua(#[from] olua::Error),
}
