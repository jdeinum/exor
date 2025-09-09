use async_trait::async_trait;

use crate::error::Result;

#[async_trait::async_trait]
pub(crate) trait Connection {}
