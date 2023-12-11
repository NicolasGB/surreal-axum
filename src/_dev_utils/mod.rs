use tokio::sync::OnceCell;

use crate::model::ModelManager;

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            ModelManager::new().await.unwrap_or_else(|err| {
                panic!("Coult not init ModelManager for tests, cause: {err:?}")
            })
        })
        .await;

    //A mm is build to be cloned
    mm.clone()
}
