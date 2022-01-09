use failure::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Repository Error")]
struct RepositoryError;

#[derive(Debug, Fail)]
pub enum TodoAppError {
  #[fail(display = "Rbatis error: {}", _0)]
  RbatisError(rbatis::Error),
  #[fail(display = "Repository Error")]
  RepositoryError,
}

pub type Result<T> = std::result::Result<T, TodoAppError>;

impl From<rbatis::Error> for TodoAppError {
  fn from(e: rbatis::Error) -> Self {
      TodoAppError::RbatisError(e)
  }
}
