pub use diesel_models::types::OrderDetailsWithAmount;

use crate::{
    router_data::{AccessToken, RouterData}, router_data_v2::{flow_common_types::GetAdditionalRecoveryDataCommon, RouterDataV2}, router_flow_types::{
        AccessTokenAuth, Authorize, AuthorizeSessionToken, CalculateTax, Capture, CompleteAuthorize, CreateConnectorCustomer, Execute, GetRecoveryDetails, PSync, PaymentMethodToken, PostAuthenticate, PostSessionTokens, PreAuthenticate, PreProcessing, RSync, Session, SetupMandate, Void
    }, router_request_types::{
        unified_authentication_service::{
            UasAuthenticationResponseData, UasPostAuthenticationRequestData,
            UasPreAuthenticationRequestData,
        }, AccessTokenRequestData, AuthorizeSessionTokenData, CompleteAuthorizeData, ConnectorCustomerData, GetRecoveryDetailsRequestData, PaymentMethodTokenizationData, PaymentsAuthorizeData, PaymentsCancelData, PaymentsCaptureData, PaymentsPostSessionTokensData, PaymentsPreProcessingData, PaymentsSessionData, PaymentsSyncData, PaymentsTaxCalculationData, RefundsData, SetupMandateRequestData
    }, router_response_types::{
        GetRecoveryDetailsResponseData, PaymentsResponseData, RefundsResponseData, TaxCalculationResponseData
    }
};

pub type PaymentsAuthorizeRouterData =
    RouterData<Authorize, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsAuthorizeSessionTokenRouterData =
    RouterData<AuthorizeSessionToken, AuthorizeSessionTokenData, PaymentsResponseData>;
pub type PaymentsPreProcessingRouterData =
    RouterData<PreProcessing, PaymentsPreProcessingData, PaymentsResponseData>;
pub type PaymentsSyncRouterData = RouterData<PSync, PaymentsSyncData, PaymentsResponseData>;
pub type PaymentsCaptureRouterData = RouterData<Capture, PaymentsCaptureData, PaymentsResponseData>;
pub type PaymentsCancelRouterData = RouterData<Void, PaymentsCancelData, PaymentsResponseData>;
pub type SetupMandateRouterData =
    RouterData<SetupMandate, SetupMandateRequestData, PaymentsResponseData>;
pub type RefundsRouterData<F> = RouterData<F, RefundsData, RefundsResponseData>;
pub type RefundExecuteRouterData = RouterData<Execute, RefundsData, RefundsResponseData>;
pub type RefundSyncRouterData = RouterData<RSync, RefundsData, RefundsResponseData>;
pub type TokenizationRouterData =
    RouterData<PaymentMethodToken, PaymentMethodTokenizationData, PaymentsResponseData>;
pub type ConnectorCustomerRouterData =
    RouterData<CreateConnectorCustomer, ConnectorCustomerData, PaymentsResponseData>;
pub type PaymentsCompleteAuthorizeRouterData =
    RouterData<CompleteAuthorize, CompleteAuthorizeData, PaymentsResponseData>;
pub type PaymentsTaxCalculationRouterData =
    RouterData<CalculateTax, PaymentsTaxCalculationData, TaxCalculationResponseData>;
pub type RefreshTokenRouterData = RouterData<AccessTokenAuth, AccessTokenRequestData, AccessToken>;
pub type PaymentsPostSessionTokensRouterData =
    RouterData<PostSessionTokens, PaymentsPostSessionTokensData, PaymentsResponseData>;
pub type PaymentsSessionRouterData = RouterData<Session, PaymentsSessionData, PaymentsResponseData>;

pub type UasPostAuthenticationRouterData =
    RouterData<PostAuthenticate, UasPostAuthenticationRequestData, UasAuthenticationResponseData>;

pub type UasPreAuthenticationRouterData =
    RouterData<PreAuthenticate, UasPreAuthenticationRequestData, UasAuthenticationResponseData>;

pub type GetRecoveryDetailsRouterData = 
    RouterData<GetRecoveryDetails,GetRecoveryDetailsRequestData,GetRecoveryDetailsResponseData>;