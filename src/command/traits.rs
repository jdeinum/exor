use crate::error::Result;

#[async_trait::async_trait]
pub trait Command {
    async fn apply(&self) -> Result<()>;
    async fn undo(&self) -> Result<()>;
}
