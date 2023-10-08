use crate::model::Result;
use crate::{ctx::Ctx, model::ModelManager};
use serde::{Deserialize, Serialize};

// region:    --- Task types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskToCreate {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskToUpdate {
    pub title: Option<String>,
}
// endregion: --- Task types

// region:    --- TaskBmc

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskToCreate) -> Result<Task> {
        let db = mm.db();

        let created: Vec<Task> = db.create("user").content(task_c).await?;

        //Safe to unwrap i guess?
        let created = created.first().unwrap();

        Ok(created.clone())
    }
}

// endregion: --- TaskBmc

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        Ok(())
    }
}
