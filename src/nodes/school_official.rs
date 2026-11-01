use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::nodes::{admin::user::Profile, school_office::Office};


#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Official {
    pub title: String,
    pub office: Office,
    pub user_id: Option<String>,
    pub profile: Option<Profile>,
    pub exists: bool
}