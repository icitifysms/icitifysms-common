use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::nodes::{bursary::payment_items::PaymentItem, school_official::Official};



#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SchoolValues{
    pub name: String,
    pub session: String,
    pub term: String,
    pub fontFamily: String,
    pub fontWeight: String,
    pub fontSize: String,
    pub defaultTitle: String,
    pub logo: String,
    pub backgroundImage: String,
    pub address: String,
    pub motto: String,
    pub emailAddress: String,
    pub phoneNumber: String,
    pub mobileContacts: String,
    pub abbreviatedName: String,
    pub paymentItems: IndexMap<String, PaymentItem>,
    pub usesSplittedTuition: bool,
    pub usesMidTerm: bool,
    pub usesStaffTuitionFee: bool,
    pub hasAdmissionPortal: bool,
    pub usesBulkReceipt: bool,
    pub showSchoolBill: bool,
    pub showIndividualLedger: bool,
    pub usesCompulsoryPaymentFirst: bool,
    pub usesCompulsoryBasedPayment: bool,
    pub tuitionFeeBenchmark: Vec<String>,
    pub usesQuranMemorization: bool,
    pub schoolType: Vec<String>,
    pub classes: Vec<String>,
    pub classArms: Vec<String>,
    pub departments: Vec<String>,
    pub courses: Vec<String>,
    pub themeColor: String,
    pub houses: Vec<String>,
    // pub clearances: String,
    pub headOfAcademics: Official,
    pub headOfAcademicsTitle: String,
    pub reportSheetPrecision: usize, 
    pub showPosition: bool,
    //pub specsDir: String,
    pub schoolBillNote: String,
    //pub scopes: String
}