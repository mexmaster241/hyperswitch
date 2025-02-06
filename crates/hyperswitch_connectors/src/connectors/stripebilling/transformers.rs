use common_enums::enums;
use serde::{Deserialize, Serialize};
use masking::Secret;
use common_utils::{errors::CustomResult,ext_traits::ByteSliceExt};
use common_utils::types::{ StringMinorUnit };
use error_stack::ResultExt;
use hyperswitch_domain_models::{
    payment_method_data::PaymentMethodData,
    router_data::{ConnectorAuthType, RouterData},
    router_flow_types::refunds::{Execute, RSync},
    router_flow_types::GetRecoveryDetails,
    router_request_types::{ResponseId,GetRecoveryDetailsRequestData},
    router_response_types::{PaymentsResponseData, RefundsResponseData, GetRecoveryDetailsResponseData,FetchAttemptDetailsStatus},
    types::{PaymentsAuthorizeRouterData, RefundsRouterData,GetRecoveryDetailsRouterData},
};
use hyperswitch_interfaces::errors;
use crate::{
    types::{RefundsResponseRouterData, ResponseRouterData},
    utils::PaymentsAuthorizeRequestData,
};


//TODO: Fill the struct with respective fields
pub struct StripebillingRouterData<T> {
    pub amount: StringMinorUnit, // The type of amount that a connector accepts, for example, String, i64, f64, etc.
    pub router_data: T,
}

impl<T>
    From<(
        StringMinorUnit,
        T,
    )> for StripebillingRouterData<T>
{
    fn from(
        (amount, item): (
            StringMinorUnit,
            T,
        ),
    ) -> Self {
         //Todo :  use utils to convert the amount to the type of amount that a connector accepts
        Self {
            amount,
            router_data: item,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, PartialEq)]
pub struct StripebillingPaymentsRequest {
    amount: StringMinorUnit,
    card: StripebillingCard
}

#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct StripebillingCard {
    number: cards::CardNumber,
    expiry_month: Secret<String>,
    expiry_year: Secret<String>,
    cvc: Secret<String>,
    complete: bool,
}

impl TryFrom<&StripebillingRouterData<&PaymentsAuthorizeRouterData>> for StripebillingPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &StripebillingRouterData<&PaymentsAuthorizeRouterData>) -> Result<Self,Self::Error> {
        match item.router_data.request.payment_method_data.clone() {
            PaymentMethodData::Card(req_card) => {
                let card = StripebillingCard {
                    number: req_card.card_number,
                    expiry_month: req_card.card_exp_month,
                    expiry_year: req_card.card_exp_year,
                    cvc: req_card.card_cvc,
                    complete: item.router_data.request.is_auto_capture()?,
                };
                Ok(Self {
                    amount: item.amount.clone(),
                    card,
                })
            }
            _ => Err(errors::ConnectorError::NotImplemented("Payment method".to_string()).into()),
        }
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct StripebillingAuthType {
    pub(super) api_key: Secret<String>
}

impl TryFrom<&ConnectorAuthType> for StripebillingAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
                api_key: api_key.to_owned(),
            }),
            _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
        }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StripebillingPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<StripebillingPaymentStatus> for common_enums::AttemptStatus {
    fn from(item: StripebillingPaymentStatus) -> Self {
        match item {
            StripebillingPaymentStatus::Succeeded => Self::Charged,
            StripebillingPaymentStatus::Failed => Self::Failure,
            StripebillingPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StripebillingPaymentsResponse {
    status: StripebillingPaymentStatus,
    id: String,
}

impl<F,T> TryFrom<ResponseRouterData<F, StripebillingPaymentsResponse, T, PaymentsResponseData>> for RouterData<F, T, PaymentsResponseData> {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: ResponseRouterData<F, StripebillingPaymentsResponse, T, PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: common_enums::AttemptStatus::from(item.response.status),
            response: Ok(PaymentsResponseData::TransactionResponse {
                resource_id: ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: Box::new(None),
                mandate_reference: Box::new(None),
                connector_metadata: None,
                network_txn_id: None,
                connector_response_reference_id: None,
                incremental_authorization_allowed: None,
                charge_id: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct StripebillingRefundRequest {
    pub amount: StringMinorUnit
}

impl<F> TryFrom<&StripebillingRouterData<&RefundsRouterData<F>>> for StripebillingRefundRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &StripebillingRouterData<&RefundsRouterData<F>>) -> Result<Self,Self::Error> {
        Ok(Self {
            amount: item.amount.to_owned(),
        })
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    id: String,
    status: RefundStatus
}

impl TryFrom<RefundsResponseRouterData<Execute, RefundResponse>>
    for RefundsRouterData<Execute>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: RefundsResponseRouterData<Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

impl TryFrom<RefundsResponseRouterData<RSync, RefundResponse>> for RefundsRouterData<RSync>
{
     type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: RefundsResponseRouterData<RSync, RefundResponse>) -> Result<Self,Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct StripebillingErrorResponse {
    pub status_code: u16,
    pub code: String,
    pub message: String,
    pub reason: Option<String>,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct StripebillingWebhookBody {
    #[serde(rename = "type")]
    pub event_type : StripebillingEventType,
    pub data : StripebillingWebhookData
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StripebillingEventType {
    #[serde(rename="invoice.paid")]
    PaymentSucceeded,
    #[serde(rename="invoice.payment_failed")]
    PaymentFailed,
    #[serde(rename="invoice.voided")]
    InvoiceDeleted
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StripebillingWebhookData {
    pub object : StripebillingWebhookObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StripebillingWebhookObject{
    //invoiceid
    #[serde(rename="id")]
    pub invoice_id : String,
    //currency
    pub currency : enums::Currency,
    //customer id
    pub customer : String,
    #[serde(rename="amount_remaining")]
    pub amount :  common_utils::types::MinorUnit,
    //charge id to get payment method
    pub charge : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StripebillingRecoveryDetailsData {

    pub charge_id : String,
    pub status : StripebillingChargeStatus,
    pub amount : common_utils::types::MinorUnit,
    pub currency : enums::Currency,
    pub customer : String,
    pub payment_method : String,
    pub failure_code : String,
    pub failure_message : String,
    pub created : i64

}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StripebillingChargeStatus{
    Succeeded,
    Failed,
    Pending
}


impl From<StripebillingChargeStatus> for FetchAttemptDetailsStatus {
    fn from(status: StripebillingChargeStatus) -> Self {
        match status {
            StripebillingChargeStatus::Succeeded => Self::Succeeded,
            StripebillingChargeStatus::Failed => Self::Failed,
            StripebillingChargeStatus::Pending => Self::Pending,
        }
    }
}

pub struct StripebillingRevenueRecoveryDetails{
    // amount
   pub amount: common_utils::types::MinorUnit,
   /// currency
   pub currency: enums::Currency,
   /// merchant reference id ex: invoice_id
   pub invoice_id: String,
   /// connector transaction id
   pub charge_id: Option<String>,
   /// error code for failure payments
   pub faliure_code: Option<String>,
   /// error_message for failure messages
   pub failure_message: Option<String>,
   /// mandate id of the connector
   pub mandate_id: Option<String>,
   /// connnector customer id
   pub customer_id: Option<String>,
   /// payment merchant connnector account reference id
   pub connector_account_reference_id: Option<String>,
   /// created_at
   pub created_at: Option<time::PrimitiveDateTime>,
}

impl StripebillingRevenueRecoveryDetails{
    pub fn from_webhook_and_additional_data(
        webhook : &StripebillingWebhookBody,
        additional_data_payload : &StripebillingRecoveryDetailsData
    ) -> Self{

        let created_at = time::OffsetDateTime::from_unix_timestamp(additional_data_payload.created).unwrap();
        Self{
            amount: webhook.data.object.amount,
            currency: webhook.data.object.currency,
            invoice_id: webhook.data.object.invoice_id.clone(),
            charge_id: Some(webhook.data.object.charge.clone()),
            faliure_code: Some(additional_data_payload.failure_code.clone()),
            failure_message: Some(additional_data_payload.failure_message.clone()),
            mandate_id: Some(additional_data_payload.payment_method.clone()),
            customer_id: Some(additional_data_payload.customer.clone()),
            connector_account_reference_id: None,
            created_at : Some(time::PrimitiveDateTime::new(created_at.date(),created_at.time())),
        }
    }
}

impl TryFrom<
    ResponseRouterData<
        GetRecoveryDetails,
        StripebillingRecoveryDetailsData,
        GetRecoveryDetailsRequestData,
        GetRecoveryDetailsResponseData
        >>
    for GetRecoveryDetailsRouterData
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: ResponseRouterData<
                            GetRecoveryDetails,
                            StripebillingRecoveryDetailsData,
                            GetRecoveryDetailsRequestData,
                            GetRecoveryDetailsResponseData
                    >   
    ) -> Result<Self,Self::Error> {
        Ok(Self {
            response: Ok(GetRecoveryDetailsResponseData{
                status: item.response.status.into(),
                payment_method: item.response.payment_method,
                error_code: item.response.failure_code,
                error_message: item.response.failure_message,
            }),
            ..item.data
        })
    }
}


impl StripebillingWebhookBody {
    pub fn get_webhook_object_from_body(
        body: &[u8],
    ) -> CustomResult<StripebillingWebhookBody, errors::ConnectorError> {
        let webhook_body = body
            .parse_struct::<StripebillingWebhookBody>("StripebillingWebhookBody")
            .change_context(errors::ConnectorError::WebhookBodyDecodingFailed)?;
        Ok(webhook_body)
    }
}



impl TryFrom<StripebillingRevenueRecoveryDetails> for hyperswitch_interfaces::recovery::RecoveryPayload{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: StripebillingRevenueRecoveryDetails,
    ) -> Result<Self,Self::Error>{
        Ok(hyperswitch_interfaces::recovery::RecoveryPayload{
            amount: item.amount,
            currency: item.currency,
            merchant_reference_id: item.invoice_id,
            connector_transaction_id: item.charge_id,
            error_code: item.faliure_code,
            error_message: item.failure_message,
            connector_mandate_id: item.mandate_id,
            connector_customer_id: item.customer_id,
            connector_account_reference_id: item.connector_account_reference_id,
            created_at: item.created_at,
        })
        
    }
}

impl TryFrom<StripebillingWebhookBody> for hyperswitch_interfaces::recovery::RecoveryPayload{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: StripebillingWebhookBody) -> Result<Self,Self::Error> {
        Ok(Self{
            amount: item.data.object.amount.clone(),
            currency: item.data.object.currency.clone(),
            merchant_reference_id: item.data.object.invoice_id.clone(),
            connector_transaction_id: Some(item.data.object.charge.clone()),
            error_code: None,
            error_message: None,
            connector_mandate_id: None,
            connector_customer_id: None,
            connector_account_reference_id: None,
            created_at: None,
        })
    }
}