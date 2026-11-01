use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use crate::nodes::prelude::*;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, Default, Clone)]
pub struct PaymentItem{
    pub id: String,
    pub value: String,
    pub label: String,
    #[serde(deserialize_with = "deserialize_to_default")]
    pub r#type: PaymentItemType,
    pub lsn: String,
    pub category: PaymentItemCategory,
    pub amount: Option<i32>
}



#[derive(Sequence, Debug, Clone)]
pub enum PaymentItemProperty{
    Id,
    Value,
    Label,
    Type,
    Lsn,
    Category
}

impl KeyValueEnum for PaymentItemProperty{

    fn value(&self) -> String {
        match self{
            PaymentItemProperty::Id => "Content ID".to_string(),
            PaymentItemProperty::Value => "Value".to_string(),
            PaymentItemProperty::Label => "Label".to_string(),
            PaymentItemProperty::Type => "Type".to_string(),
            PaymentItemProperty::Lsn => "Ledger Label".to_string(),
            PaymentItemProperty::Category => "Category".to_string()
        }
    }
}


#[derive(Sequence, Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PaymentItemCategory{
    Fixed,
    #[default]
    Unfixed,
    Defined,
    Compulsory
}



#[derive(Sequence, Debug, Serialize, Deserialize, ToSchema, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PaymentItemType{
    #[default]
    Termly,
    Annually,
    Extra
}
impl KeyValueEnum for PaymentItemType{}
