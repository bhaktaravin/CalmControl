use crate::store::UserStore;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_store: UserStore,
}
