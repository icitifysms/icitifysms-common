use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct PaymentUrls{
    pub successful: String,
    pub cancelled: String,
    pub failed: String,
    pub invalid: String
}