use enum_iterator::Sequence;
use macros::ReverseOrd;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, Clone, Default)]
#[schema(as = UserProfile)]
pub struct Profile{
    pub uid: u32,
    pub branch: String,
    pub userId: String,
    pub username: String,
    pub activeStatus: String,
    pub name: String,
    pub admission_number: Option<String>,
    pub tid: Option<i32>,
    pub email: Option<String>,
    pub phoneNumber: String,
    pub clearance: Clearance,
    pub dp: String,
    pub student_dp: Option<String>,
    pub signature: Option<String>,
    pub description: String,
    pub has_student_account: bool,
    pub has_teacher_account: bool
}



#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Copy, Hash, EnumString, Display, Default, Sequence, ReverseOrd)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Clearance{
    IcitifyDev,
    SchoolAdmin,
    BranchAdmin,
    #[serde(rename = "head_of_academics")]
    #[strum(serialize = "head_of_academics")]
    HeadOfAcademics,
    StaffAdmin,
    Bursar,
    #[serde(rename = "bursary_department")]
    #[strum(serialize = "bursary_department")]
    BursaryMember,
    StaffUser,
    Teacher,
    Parent,
    #[default]
    Student
}