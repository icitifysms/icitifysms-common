use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::nodes::bursary::payment_gateway::PaymentGatewayPublicInfo;
use crate::nodes::node::PaymentUrls;
use crate::nodes::school::SchoolValues;

pub mod payment_items;
pub mod payment_gateway;


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Bursary::InitiatePaymentResponse)]
pub struct InitiatePaymentResponse {
    pub transaction_number: String,
    pub transaction_reference: String,
    pub token: String,
    pub request_time: String,
    pub payment_urls: PaymentUrls,
    pub school: SchoolValues,
    pub payment_gateway_info: PaymentGatewayPublicInfo
}