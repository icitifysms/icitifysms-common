use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
#[schema(as = FlutterwavePublicInfo)]
pub struct PublicInfo {
    pub public_key: String,
    pub transaction_charge: i32
}


