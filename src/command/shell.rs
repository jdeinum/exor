use crate::command::traits::Command;
use crate::error::Result;

pub struct ExecuteCommand {
    pub command: String,
    pub args: Vec<String>,
}

#[async_trait::async_trait]
impl Command for ExecuteCommand {
    async fn apply(&self) -> Result<()> {
        todo!()
    }
    async fn undo(&self) -> Result<()> {
        todo!()
    }
}
