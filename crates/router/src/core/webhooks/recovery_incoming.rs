use std::{marker::PhantomData,str::FromStr};
use common_utils::ext_traits::ValueExt;
use api_models::webhooks::{self, WebhookResponseTracker};
use common_utils::ext_traits::ByteSliceExt;
use error_stack::ResultExt;
use error_stack::report;
use hyperswitch_domain_models::router_request_types::GetRecoveryDetailsRequestData;
use hyperswitch_domain_models::router_response_types::GetRecoveryDetailsResponseData;
use hyperswitch_interfaces::recovery::RecoveryPayload;
use hyperswitch_interfaces::webhooks::IncomingWebhookRequestDetails;
use router_env::{instrument, tracing};
use hyperswitch_domain_models::merchant_connector_account::MerchantConnectorAccount;
use types::api::{ConnectorData,GetToken};
use crate::{
    core::errors::{self, CustomResult},
    core::payments::helpers,
    core::payments,
    routes::SessionState,
    types::{
        self,
        api::{self, IncomingWebhook},
        domain,
        PaymentAddress
    },
    services::connector_integration_interface::ConnectorEnum,
    services,
    // db::{get_and_deserialize_key, StorageInterface},
};



#[allow(clippy::too_many_arguments)]
#[instrument(skip_all)]
#[cfg(feature= "recovery")]
pub async fn recovery_incoming_webhook_flow(
    state: SessionState,
    merchant_account: domain::MerchantAccount,
    merchant_connector_account : MerchantConnectorAccount,
    connector_name : &str,
    _business_profile: domain::Profile,
    _key_store: domain::MerchantKeyStore,
    _webhook_details: api::IncomingWebhookDetails,
    source_verified: bool,
    connector: &ConnectorEnum,
    request_details: &IncomingWebhookRequestDetails<'_>,
    event_type: webhooks::IncomingWebhookEvent,
) -> CustomResult<WebhookResponseTracker, errors::ApiErrorResponse> {
    use error_stack::report;
    use hyperswitch_interfaces::recovery::{RecoveryAction, RecoveryActionTrait, RecoveryPayload, RecoveryTrait};

    match source_verified {
        true => {
            let _db = &*state.store;

            let connector_enum = api_models::enums::Connector::from_str(&connector_name)
            .change_context(errors::ApiErrorResponse::InvalidDataValue {
                field_name: "connector",
            })
            .attach_printable_lazy(|| {
                format!("unable to parse connector name {connector_name:?}")
            })?;
            let connector_with_additonal_recovery_details_call = &state.conf.additonal_recovery_details_call;

            let invoice_details = match connector_with_additonal_recovery_details_call.connectors_with_additional_recovery_details_call.contains(&connector_enum){
                true => {

                    println!("going through the difficult block");
                    let invoice_details = connector.get_recovery_details(request_details).change_context(errors::ApiErrorResponse::InternalServerError)?;
                    
                    let additional_data = handle_additional_recovery_details_call(
                        &connector,
                        &state,
                        &merchant_account,
                        merchant_connector_account.clone(),
                        &connector_name,
                        &invoice_details, 
                    ).await
                    .change_context(errors::ApiErrorResponse::WebhookProcessingFailure)
                    .attach_printable("cannot make the additional call")?;

                    RecoveryPayload{
                        amount : invoice_details.amount,
                        currency : invoice_details.currency,
                        merchant_reference_id : invoice_details.merchant_reference_id,
                        connector_account_reference_id : invoice_details.connector_account_reference_id,
                        connector_transaction_id: invoice_details.connector_transaction_id,
                        error_code : invoice_details.error_code.or(additional_data.payment_processor_error_code),
                        error_message : invoice_details.error_message.or(additional_data.payment_processor_error_message),
                        connector_mandate_id : invoice_details.connector_mandate_id.or(additional_data.payment_method),
                        connector_customer_id : invoice_details.connector_customer_id,
                        created_at : invoice_details.created_at.or(additional_data.created_at)
                    }

                },
                false => {

                    println!("Going thorugh the easy block");

                    let recovery_details = connector.get_recovery_details(request_details).change_context(errors::ApiErrorResponse::InternalServerError)?;
                    recovery_details
                    
                },
            };

            println!("THe recovery payload looks like this");
            println!("{:?}",invoice_details);
            
            let _payment_intent = invoice_details.get_intent().change_context(errors::ApiErrorResponse::InternalServerError)?;
            let payment_attempt = invoice_details.get_attempt().change_context(errors::ApiErrorResponse::InternalServerError)?;

            // find optional running job associated with payment intent
            // let running_job = invoice_details.

            let passive_churn_recovery_data = payment_attempt.feature_metadata.and_then(|metadata|metadata.passive_churn_recovery);
            let triggered_by = passive_churn_recovery_data.map(|data|data.triggered_by);

            let action = RecoveryAction::find_action(event_type,triggered_by);

            match action{
                RecoveryAction::CancelInvoice => todo!(),
                RecoveryAction::FailPaymentExternal => todo!(),
                RecoveryAction::SuccessPaymentExternal => todo!(),
                RecoveryAction::PendingPayment => todo!(),
                RecoveryAction::NoAction => todo!(),
                RecoveryAction::InvalidAction => todo!(),
            }
        },
        false => Err(report!(
            errors::ApiErrorResponse::WebhookAuthenticationFailed
        ))
    }
}

const IRRELEVANT_ATTEMPT_ID_IN_SOURCE_VERIFICATION_FLOW: &str =
    "irrelevant_attempt_id_in_source_verification_flow";

const IRRELEVANT_CONNECTOR_REQUEST_REFERENCE_ID_IN_SOURCE_VERIFICATION_FLOW: &str =
"irrelevant_connector_request_reference_id_in_source_verification_flow";


async fn handle_additional_recovery_details_call(
    _connector: &ConnectorEnum,
    state: &SessionState,
    merchant_account: &domain::MerchantAccount,
    merchant_connector_account: MerchantConnectorAccount,
    connector_name: &str,
    invoice_details: &RecoveryPayload
) -> CustomResult<GetRecoveryDetailsResponseData, errors::ConnectorError> {

    let connector_data = ConnectorData::get_connector_by_name(
        &state.conf.connectors,
        connector_name,
        GetToken::Connector,
        None,
    )
    .change_context(errors::ConnectorError::WebhookSourceVerificationFailed)
    .attach_printable("invalid connector name received in payment attempt")?;

    let connector_integration: services::BoxedGetAdditionalRecoveryDetailsIntegrationInterface<
        hyperswitch_domain_models::router_flow_types::GetRecoveryDetails,
        GetRecoveryDetailsRequestData,
        GetRecoveryDetailsResponseData,
    > = connector_data.connector.get_connector_integration();

    let router_data = construct_router_data_for_additional_call(
        state,
        connector_name,
        merchant_connector_account,
        merchant_account,
        invoice_details,
    )
    .await
    .change_context(errors::ConnectorError::RequestEncodingFailed)
    .attach_printable("Failed while constructing additional recovery details call router data")?;

    let response = services::execute_connector_processing_step(
        state,
        connector_integration,
        &router_data,
        payments::CallConnectorAction::Trigger,
        None,
    )
    .await?;

    let recovery_details = response
        .response;
    
    match recovery_details {
        Ok(response)=> Ok(response),
       _=> Ok(GetRecoveryDetailsResponseData{
        payment_method : None,
        status: None,
        payment_method_details : None,
        payment_processor_error_code : None,
        payment_processor_error_message : None,
        created_at : None
       }) 
    }

}

async fn construct_router_data_for_additional_call(
    state: &SessionState,
    connector_name: &str,
    merchant_connector_account: MerchantConnectorAccount,
    merchant_account: &domain::MerchantAccount,
    invoice_details: &RecoveryPayload,
) -> CustomResult<types::GetRecoveryDetailsRouterData, errors::ApiErrorResponse>{
    let auth_type: types::ConnectorAuthType =
        helpers::MerchantConnectorAccountType::DbVal(Box::new(merchant_connector_account.clone()))
            .get_connector_account_details()
            .parse_value("ConnectorAuthType")
            .change_context(errors::ApiErrorResponse::InternalServerError)?;
    

    let router_data = types::RouterData {
        flow: PhantomData,
        merchant_id: merchant_account.get_id().clone(),
        connector: connector_name.to_string(),
        customer_id: None,
        tenant_id: state.tenant.tenant_id.clone(),
        payment_id: common_utils::id_type::PaymentId::get_irrelevant_id("source_verification_flow")
            .get_string_repr()
            .to_owned(),
        attempt_id: IRRELEVANT_ATTEMPT_ID_IN_SOURCE_VERIFICATION_FLOW.to_string(),
        status: diesel_models::enums::AttemptStatus::default(),
        payment_method: diesel_models::enums::PaymentMethod::default(),
        connector_auth_type: auth_type,
        description: None,
        address: PaymentAddress::default(),
        auth_type: diesel_models::enums::AuthenticationType::default(),
        connector_meta_data: None,
        connector_wallets_details: None,
        amount_captured: None,
        minor_amount_captured: None,
        request : GetRecoveryDetailsRequestData{
            payment_attempt_id : invoice_details.connector_transaction_id.clone().unwrap()
        },
        response: Err(types::ErrorResponse::default()),
        access_token: None,
        session_token: None,
        reference_id: None,
        payment_method_token: None,
        connector_customer: None,
        recurring_mandate_payment_data: None,
        preprocessing_id: None,
        connector_request_reference_id:
            IRRELEVANT_CONNECTOR_REQUEST_REFERENCE_ID_IN_SOURCE_VERIFICATION_FLOW.to_string(),
        #[cfg(feature = "payouts")]
        payout_method_data: None,
        #[cfg(feature = "payouts")]
        quote_id: None,
        test_mode: None,
        payment_method_balance: None,
        payment_method_status: None,
        connector_api_version: None,
        connector_http_status_code: None,
        external_latency: None,
        apple_pay_flow: None,
        frm_metadata: None,
        refund_id: None,
        dispute_id: None,
        connector_response: None,
        integrity_check: Ok(()),
        additional_merchant_data: None,
        header_payload: None,
        connector_mandate_request_reference_id: None,
        authentication_id: None,
        psd2_sca_exemption_type: None,
    };
    Ok(router_data)
}    
