use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::{migrate::MigrateError, Error as DbError};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("データがありませんでした")]
    NotFound,
    #[error("{0}に無効な値が設定されています")]
    Configuration(String),
    #[error("予期せぬエラーが発生しました")]
    Unknown,
    #[error("{0}が正しくありません")]
    InvalidFormat(String),
    #[error("実装されていないコードを呼び出しています")]
    Unimplemented,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Configuration(_) | Self::Unknown => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::Unimplemented => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::Unknown.to_string(),
            )
                .into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::InvalidFormat(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}

impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        match value {
            DbError::RowNotFound => Error::NotFound,
            _ => Error::Unknown,
        }
    }
}

impl From<MigrateError> for Error {
    fn from(_value: MigrateError) -> Self {
        Self::Unknown
    }
}
