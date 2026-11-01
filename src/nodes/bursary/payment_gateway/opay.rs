use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
#[schema(as = OpayPublicInfo)]
pub struct PublicInfo {
    pub cashierUrl: String
}


