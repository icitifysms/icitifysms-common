use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
#[schema(as = PaystackPublicInfo)]
pub struct PublicInfo {
    pub access_code: String,
    pub reference: String,
}