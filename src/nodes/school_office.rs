use enum_iterator::Sequence;
use macros::ParseEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Sequence, Hash, PartialEq, Eq, Display, EnumString, ParseEnum, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Office{
    HeadOfAcademics,
    Bursar,
    AdmissionOfficer
}
