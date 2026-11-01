use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
#[schema(as = MonnifyPublicInfo)]
pub struct PublicInfo {}
