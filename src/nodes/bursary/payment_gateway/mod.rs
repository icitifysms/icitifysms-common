use serde::{Serialize, Deserialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

pub mod flutterwave_v4;
pub mod paystack;
pub mod flutterwave;
pub mod monnify;
pub mod remita;
pub mod opay;


#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
pub struct DefaultInfo{}


#[derive(Serialize, Deserialize, ToSchema, EnumString, Display, Clone)]
#[serde(untagged)]
pub enum PaymentGatewayPublicInfo {
    Opay(flutterwave_v4::PublicInfo),
    Paystack(paystack::PublicInfo),
    Flutterwave(flutterwave::PublicInfo),
    ApplePay(flutterwave_v4::PublicInfo),
    GooglePay(flutterwave_v4::PublicInfo),
    Monnify(monnify::PublicInfo),
    Remita(remita::PublicInfo),
    Default(DefaultInfo),
}