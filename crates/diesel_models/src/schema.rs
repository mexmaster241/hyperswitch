// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    address (address_id) {
        #[max_length = 64]
        address_id -> Varchar,
        #[max_length = 128]
        city -> Nullable<Varchar>,
        country -> Nullable<CountryAlpha2>,
        line1 -> Nullable<Bytea>,
        line2 -> Nullable<Bytea>,
        line3 -> Nullable<Bytea>,
        state -> Nullable<Bytea>,
        zip -> Nullable<Bytea>,
        first_name -> Nullable<Bytea>,
        last_name -> Nullable<Bytea>,
        phone_number -> Nullable<Bytea>,
        #[max_length = 8]
        country_code -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 64]
        customer_id -> Nullable<Varchar>,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        payment_id -> Nullable<Varchar>,
        #[max_length = 32]
        updated_by -> Varchar,
        email -> Nullable<Bytea>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    api_keys (key_id) {
        #[max_length = 64]
        key_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        #[max_length = 128]
        hashed_api_key -> Varchar,
        #[max_length = 16]
        prefix -> Varchar,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
        last_used -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    authentication (authentication_id) {
        #[max_length = 64]
        authentication_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        authentication_connector -> Varchar,
        #[max_length = 64]
        connector_authentication_id -> Nullable<Varchar>,
        authentication_data -> Nullable<Jsonb>,
        #[max_length = 64]
        payment_method_id -> Varchar,
        #[max_length = 64]
        authentication_type -> Nullable<Varchar>,
        #[max_length = 64]
        authentication_status -> Varchar,
        #[max_length = 64]
        authentication_lifecycle_status -> Varchar,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        error_message -> Nullable<Text>,
        #[max_length = 64]
        error_code -> Nullable<Varchar>,
        connector_metadata -> Nullable<Jsonb>,
        maximum_supported_version -> Nullable<Jsonb>,
        #[max_length = 64]
        threeds_server_transaction_id -> Nullable<Varchar>,
        #[max_length = 64]
        cavv -> Nullable<Varchar>,
        #[max_length = 64]
        authentication_flow_type -> Nullable<Varchar>,
        message_version -> Nullable<Jsonb>,
        #[max_length = 64]
        eci -> Nullable<Varchar>,
        #[max_length = 64]
        trans_status -> Nullable<Varchar>,
        #[max_length = 64]
        acquirer_bin -> Nullable<Varchar>,
        #[max_length = 64]
        acquirer_merchant_id -> Nullable<Varchar>,
        three_ds_method_data -> Nullable<Varchar>,
        three_ds_method_url -> Nullable<Varchar>,
        acs_url -> Nullable<Varchar>,
        challenge_request -> Nullable<Varchar>,
        acs_reference_number -> Nullable<Varchar>,
        acs_trans_id -> Nullable<Varchar>,
        acs_signed_content -> Nullable<Varchar>,
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 255]
        payment_id -> Nullable<Varchar>,
        #[max_length = 128]
        merchant_connector_id -> Varchar,
        #[max_length = 64]
        ds_trans_id -> Nullable<Varchar>,
        #[max_length = 128]
        directory_server_id -> Nullable<Varchar>,
        #[max_length = 64]
        acquirer_country_code -> Nullable<Varchar>,
        service_details -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    blocklist (merchant_id, fingerprint_id) {
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        fingerprint_id -> Varchar,
        data_kind -> BlocklistDataKind,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    blocklist_fingerprint (id) {
        id -> Int4,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        fingerprint_id -> Varchar,
        data_kind -> BlocklistDataKind,
        encrypted_fingerprint -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    blocklist_lookup (id) {
        id -> Int4,
        #[max_length = 64]
        merchant_id -> Varchar,
        fingerprint -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    business_profile (profile_id) {
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        profile_name -> Varchar,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        return_url -> Nullable<Text>,
        enable_payment_response_hash -> Bool,
        #[max_length = 255]
        payment_response_hash_key -> Nullable<Varchar>,
        redirect_to_merchant_with_http_post -> Bool,
        webhook_details -> Nullable<Json>,
        metadata -> Nullable<Json>,
        routing_algorithm -> Nullable<Json>,
        intent_fulfillment_time -> Nullable<Int8>,
        frm_routing_algorithm -> Nullable<Jsonb>,
        payout_routing_algorithm -> Nullable<Jsonb>,
        is_recon_enabled -> Bool,
        applepay_verified_domains -> Nullable<Array<Nullable<Text>>>,
        payment_link_config -> Nullable<Jsonb>,
        session_expiry -> Nullable<Int8>,
        authentication_connector_details -> Nullable<Jsonb>,
        payout_link_config -> Nullable<Jsonb>,
        is_extended_card_info_enabled -> Nullable<Bool>,
        extended_card_info_config -> Nullable<Jsonb>,
        is_connector_agnostic_mit_enabled -> Nullable<Bool>,
        use_billing_as_payment_method_billing -> Nullable<Bool>,
        collect_shipping_details_from_wallet_connector -> Nullable<Bool>,
        collect_billing_details_from_wallet_connector -> Nullable<Bool>,
        outgoing_webhook_custom_http_headers -> Nullable<Bytea>,
        always_collect_billing_details_from_wallet_connector -> Nullable<Bool>,
        always_collect_shipping_details_from_wallet_connector -> Nullable<Bool>,
        #[max_length = 64]
        tax_connector_id -> Nullable<Varchar>,
        is_tax_connector_enabled -> Nullable<Bool>,
        version -> ApiVersion,
        dynamic_routing_algorithm -> Nullable<Json>,
        is_network_tokenization_enabled -> Bool,
        is_auto_retries_enabled -> Nullable<Bool>,
        max_auto_retries_enabled -> Nullable<Int2>,
        is_click_to_pay_enabled -> Bool,
        authentication_product_ids -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    captures (capture_id) {
        #[max_length = 64]
        capture_id -> Varchar,
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        status -> CaptureStatus,
        amount -> Int8,
        currency -> Nullable<Currency>,
        #[max_length = 255]
        connector -> Varchar,
        #[max_length = 255]
        error_message -> Nullable<Varchar>,
        #[max_length = 255]
        error_code -> Nullable<Varchar>,
        #[max_length = 255]
        error_reason -> Nullable<Varchar>,
        tax_amount -> Nullable<Int8>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 64]
        authorized_attempt_id -> Varchar,
        #[max_length = 128]
        connector_capture_id -> Nullable<Varchar>,
        capture_sequence -> Int2,
        #[max_length = 128]
        connector_response_reference_id -> Nullable<Varchar>,
        #[max_length = 512]
        connector_capture_data -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    cards_info (card_iin) {
        #[max_length = 16]
        card_iin -> Varchar,
        card_issuer -> Nullable<Text>,
        card_network -> Nullable<Text>,
        card_type -> Nullable<Text>,
        card_subtype -> Nullable<Text>,
        card_issuing_country -> Nullable<Text>,
        #[max_length = 32]
        bank_code_id -> Nullable<Varchar>,
        #[max_length = 32]
        bank_code -> Nullable<Varchar>,
        #[max_length = 32]
        country_code -> Nullable<Varchar>,
        date_created -> Timestamp,
        last_updated -> Nullable<Timestamp>,
        last_updated_provider -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    configs (key) {
        id -> Int4,
        #[max_length = 255]
        key -> Varchar,
        config -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    customers (customer_id, merchant_id) {
        #[max_length = 64]
        customer_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        name -> Nullable<Bytea>,
        email -> Nullable<Bytea>,
        phone -> Nullable<Bytea>,
        #[max_length = 8]
        phone_country_code -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        metadata -> Nullable<Json>,
        connector_customer -> Nullable<Jsonb>,
        modified_at -> Timestamp,
        #[max_length = 64]
        address_id -> Nullable<Varchar>,
        #[max_length = 64]
        default_payment_method_id -> Nullable<Varchar>,
        #[max_length = 64]
        updated_by -> Nullable<Varchar>,
        version -> ApiVersion,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    dashboard_metadata (id) {
        id -> Int4,
        #[max_length = 64]
        user_id -> Nullable<Varchar>,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        org_id -> Varchar,
        data_key -> DashboardMetadata,
        data_value -> Json,
        #[max_length = 64]
        created_by -> Varchar,
        created_at -> Timestamp,
        #[max_length = 64]
        last_modified_by -> Varchar,
        last_modified_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    dispute (dispute_id) {
        #[max_length = 64]
        dispute_id -> Varchar,
        #[max_length = 255]
        amount -> Varchar,
        #[max_length = 255]
        currency -> Varchar,
        dispute_stage -> DisputeStage,
        dispute_status -> DisputeStatus,
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        attempt_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        connector_status -> Varchar,
        #[max_length = 255]
        connector_dispute_id -> Varchar,
        #[max_length = 255]
        connector_reason -> Nullable<Varchar>,
        #[max_length = 255]
        connector_reason_code -> Nullable<Varchar>,
        challenge_required_by -> Nullable<Timestamp>,
        connector_created_at -> Nullable<Timestamp>,
        connector_updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        connector -> Varchar,
        evidence -> Jsonb,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
        dispute_amount -> Int8,
        #[max_length = 32]
        organization_id -> Varchar,
        dispute_currency -> Nullable<Currency>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    dynamic_routing_stats (attempt_id, merchant_id) {
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        attempt_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        profile_id -> Varchar,
        amount -> Int8,
        #[max_length = 64]
        success_based_routing_connector -> Varchar,
        #[max_length = 64]
        payment_connector -> Varchar,
        currency -> Nullable<Currency>,
        #[max_length = 64]
        payment_method -> Nullable<Varchar>,
        capture_method -> Nullable<CaptureMethod>,
        authentication_type -> Nullable<AuthenticationType>,
        payment_status -> AttemptStatus,
        conclusive_classification -> SuccessBasedRoutingConclusiveState,
        created_at -> Timestamp,
        #[max_length = 64]
        payment_method_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    events (event_id) {
        #[max_length = 64]
        event_id -> Varchar,
        event_type -> EventType,
        event_class -> EventClass,
        is_webhook_notified -> Bool,
        #[max_length = 64]
        primary_object_id -> Varchar,
        primary_object_type -> EventObjectType,
        created_at -> Timestamp,
        #[max_length = 64]
        merchant_id -> Nullable<Varchar>,
        #[max_length = 64]
        business_profile_id -> Nullable<Varchar>,
        primary_object_created_at -> Nullable<Timestamp>,
        #[max_length = 64]
        idempotent_event_id -> Nullable<Varchar>,
        #[max_length = 64]
        initial_attempt_id -> Nullable<Varchar>,
        request -> Nullable<Bytea>,
        response -> Nullable<Bytea>,
        delivery_attempt -> Nullable<WebhookDeliveryAttempt>,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    file_metadata (file_id, merchant_id) {
        #[max_length = 64]
        file_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        file_name -> Nullable<Varchar>,
        file_size -> Int4,
        #[max_length = 255]
        file_type -> Varchar,
        #[max_length = 255]
        provider_file_id -> Nullable<Varchar>,
        #[max_length = 255]
        file_upload_provider -> Nullable<Varchar>,
        available -> Bool,
        created_at -> Timestamp,
        #[max_length = 255]
        connector_label -> Nullable<Varchar>,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    fraud_check (frm_id, attempt_id, payment_id, merchant_id) {
        #[max_length = 64]
        frm_id -> Varchar,
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        attempt_id -> Varchar,
        created_at -> Timestamp,
        #[max_length = 255]
        frm_name -> Varchar,
        #[max_length = 255]
        frm_transaction_id -> Nullable<Varchar>,
        frm_transaction_type -> FraudCheckType,
        frm_status -> FraudCheckStatus,
        frm_score -> Nullable<Int4>,
        frm_reason -> Nullable<Jsonb>,
        #[max_length = 255]
        frm_error -> Nullable<Varchar>,
        payment_details -> Nullable<Jsonb>,
        metadata -> Nullable<Jsonb>,
        modified_at -> Timestamp,
        #[max_length = 64]
        last_step -> Varchar,
        payment_capture_method -> Nullable<CaptureMethod>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    gateway_status_map (connector, flow, sub_flow, code, message) {
        #[max_length = 64]
        connector -> Varchar,
        #[max_length = 64]
        flow -> Varchar,
        #[max_length = 64]
        sub_flow -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        #[max_length = 1024]
        message -> Varchar,
        #[max_length = 64]
        status -> Varchar,
        #[max_length = 64]
        router_error -> Nullable<Varchar>,
        #[max_length = 64]
        decision -> Varchar,
        created_at -> Timestamp,
        last_modified -> Timestamp,
        step_up_possible -> Bool,
        #[max_length = 255]
        unified_code -> Nullable<Varchar>,
        #[max_length = 1024]
        unified_message -> Nullable<Varchar>,
        #[max_length = 64]
        error_category -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    generic_link (link_id) {
        #[max_length = 64]
        link_id -> Varchar,
        #[max_length = 64]
        primary_reference -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        expiry -> Timestamp,
        link_data -> Jsonb,
        link_status -> Jsonb,
        link_type -> GenericLinkType,
        url -> Text,
        return_url -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    incremental_authorization (authorization_id, merchant_id) {
        #[max_length = 64]
        authorization_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        payment_id -> Varchar,
        amount -> Int8,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 64]
        status -> Varchar,
        #[max_length = 255]
        error_code -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        #[max_length = 64]
        connector_authorization_id -> Nullable<Varchar>,
        previously_authorized_amount -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    locker_mock_up (id) {
        id -> Int4,
        #[max_length = 255]
        card_id -> Varchar,
        #[max_length = 255]
        external_id -> Varchar,
        #[max_length = 255]
        card_fingerprint -> Varchar,
        #[max_length = 255]
        card_global_fingerprint -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        card_number -> Varchar,
        #[max_length = 255]
        card_exp_year -> Varchar,
        #[max_length = 255]
        card_exp_month -> Varchar,
        #[max_length = 255]
        name_on_card -> Nullable<Varchar>,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        duplicate -> Nullable<Bool>,
        #[max_length = 8]
        card_cvc -> Nullable<Varchar>,
        #[max_length = 64]
        payment_method_id -> Nullable<Varchar>,
        enc_card_data -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    mandate (mandate_id) {
        #[max_length = 64]
        mandate_id -> Varchar,
        #[max_length = 64]
        customer_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        payment_method_id -> Varchar,
        mandate_status -> MandateStatus,
        mandate_type -> MandateType,
        customer_accepted_at -> Nullable<Timestamp>,
        #[max_length = 64]
        customer_ip_address -> Nullable<Varchar>,
        #[max_length = 255]
        customer_user_agent -> Nullable<Varchar>,
        #[max_length = 128]
        network_transaction_id -> Nullable<Varchar>,
        #[max_length = 64]
        previous_attempt_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        mandate_amount -> Nullable<Int8>,
        mandate_currency -> Nullable<Currency>,
        amount_captured -> Nullable<Int8>,
        #[max_length = 64]
        connector -> Varchar,
        #[max_length = 128]
        connector_mandate_id -> Nullable<Varchar>,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
        metadata -> Nullable<Jsonb>,
        connector_mandate_ids -> Nullable<Jsonb>,
        #[max_length = 64]
        original_payment_id -> Nullable<Varchar>,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
        #[max_length = 64]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    merchant_account (merchant_id) {
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        enable_payment_response_hash -> Bool,
        #[max_length = 255]
        payment_response_hash_key -> Nullable<Varchar>,
        redirect_to_merchant_with_http_post -> Bool,
        merchant_name -> Nullable<Bytea>,
        merchant_details -> Nullable<Bytea>,
        webhook_details -> Nullable<Json>,
        sub_merchants_enabled -> Nullable<Bool>,
        #[max_length = 64]
        parent_merchant_id -> Nullable<Varchar>,
        #[max_length = 128]
        publishable_key -> Nullable<Varchar>,
        storage_scheme -> MerchantStorageScheme,
        #[max_length = 64]
        locker_id -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        routing_algorithm -> Nullable<Json>,
        primary_business_details -> Json,
        intent_fulfillment_time -> Nullable<Int8>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        frm_routing_algorithm -> Nullable<Jsonb>,
        payout_routing_algorithm -> Nullable<Jsonb>,
        #[max_length = 32]
        organization_id -> Varchar,
        is_recon_enabled -> Bool,
        #[max_length = 64]
        default_profile -> Nullable<Varchar>,
        recon_status -> ReconStatus,
        payment_link_config -> Nullable<Jsonb>,
        pm_collect_link_config -> Nullable<Jsonb>,
        version -> ApiVersion,
        is_platform_account -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    merchant_connector_account (merchant_connector_id) {
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        connector_name -> Varchar,
        connector_account_details -> Bytea,
        test_mode -> Nullable<Bool>,
        disabled -> Nullable<Bool>,
        #[max_length = 128]
        merchant_connector_id -> Varchar,
        payment_methods_enabled -> Nullable<Array<Nullable<Json>>>,
        connector_type -> ConnectorType,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_label -> Nullable<Varchar>,
        business_country -> Nullable<CountryAlpha2>,
        #[max_length = 255]
        business_label -> Nullable<Varchar>,
        #[max_length = 64]
        business_sub_label -> Nullable<Varchar>,
        frm_configs -> Nullable<Jsonb>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        connector_webhook_details -> Nullable<Jsonb>,
        frm_config -> Nullable<Array<Nullable<Jsonb>>>,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        applepay_verified_domains -> Nullable<Array<Nullable<Text>>>,
        pm_auth_config -> Nullable<Jsonb>,
        status -> ConnectorStatus,
        additional_merchant_data -> Nullable<Bytea>,
        connector_wallets_details -> Nullable<Bytea>,
        version -> ApiVersion,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    merchant_key_store (merchant_id) {
        #[max_length = 64]
        merchant_id -> Varchar,
        key -> Bytea,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    organization (org_id) {
        #[max_length = 32]
        org_id -> Varchar,
        org_name -> Nullable<Text>,
        organization_details -> Nullable<Jsonb>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 32]
        id -> Nullable<Varchar>,
        organization_name -> Nullable<Text>,
        platform_merchant_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payment_attempt (attempt_id, merchant_id) {
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        attempt_id -> Varchar,
        status -> AttemptStatus,
        amount -> Int8,
        currency -> Nullable<Currency>,
        save_to_locker -> Nullable<Bool>,
        #[max_length = 64]
        connector -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        offer_amount -> Nullable<Int8>,
        surcharge_amount -> Nullable<Int8>,
        tax_amount -> Nullable<Int8>,
        #[max_length = 64]
        payment_method_id -> Nullable<Varchar>,
        payment_method -> Nullable<Varchar>,
        #[max_length = 128]
        connector_transaction_id -> Nullable<Varchar>,
        capture_method -> Nullable<CaptureMethod>,
        capture_on -> Nullable<Timestamp>,
        confirm -> Bool,
        authentication_type -> Nullable<AuthenticationType>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        cancellation_reason -> Nullable<Varchar>,
        amount_to_capture -> Nullable<Int8>,
        #[max_length = 64]
        mandate_id -> Nullable<Varchar>,
        browser_info -> Nullable<Jsonb>,
        #[max_length = 255]
        error_code -> Nullable<Varchar>,
        #[max_length = 128]
        payment_token -> Nullable<Varchar>,
        connector_metadata -> Nullable<Jsonb>,
        #[max_length = 50]
        payment_experience -> Nullable<Varchar>,
        #[max_length = 64]
        payment_method_type -> Nullable<Varchar>,
        payment_method_data -> Nullable<Jsonb>,
        #[max_length = 64]
        business_sub_label -> Nullable<Varchar>,
        straight_through_algorithm -> Nullable<Jsonb>,
        preprocessing_step_id -> Nullable<Varchar>,
        mandate_details -> Nullable<Jsonb>,
        error_reason -> Nullable<Text>,
        multiple_capture_count -> Nullable<Int2>,
        #[max_length = 128]
        connector_response_reference_id -> Nullable<Varchar>,
        amount_capturable -> Int8,
        #[max_length = 32]
        updated_by -> Varchar,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
        authentication_data -> Nullable<Json>,
        encoded_data -> Nullable<Text>,
        #[max_length = 255]
        unified_code -> Nullable<Varchar>,
        #[max_length = 1024]
        unified_message -> Nullable<Varchar>,
        net_amount -> Nullable<Int8>,
        external_three_ds_authentication_attempted -> Nullable<Bool>,
        #[max_length = 64]
        authentication_connector -> Nullable<Varchar>,
        #[max_length = 64]
        authentication_id -> Nullable<Varchar>,
        mandate_data -> Nullable<Jsonb>,
        #[max_length = 64]
        fingerprint_id -> Nullable<Varchar>,
        #[max_length = 64]
        payment_method_billing_address_id -> Nullable<Varchar>,
        #[max_length = 64]
        charge_id -> Nullable<Varchar>,
        #[max_length = 64]
        client_source -> Nullable<Varchar>,
        #[max_length = 64]
        client_version -> Nullable<Varchar>,
        customer_acceptance -> Nullable<Jsonb>,
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 32]
        organization_id -> Varchar,
        #[max_length = 32]
        card_network -> Nullable<Varchar>,
        shipping_cost -> Nullable<Int8>,
        order_tax_amount -> Nullable<Int8>,
        #[max_length = 512]
        connector_transaction_data -> Nullable<Varchar>,
        connector_mandate_detail -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payment_intent (payment_id, merchant_id) {
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        status -> IntentStatus,
        amount -> Int8,
        currency -> Nullable<Currency>,
        amount_captured -> Nullable<Int8>,
        #[max_length = 64]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 64]
        connector_id -> Nullable<Varchar>,
        #[max_length = 64]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 64]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        setup_future_usage -> Nullable<FutureUsage>,
        off_session -> Nullable<Bool>,
        #[max_length = 128]
        client_secret -> Nullable<Varchar>,
        #[max_length = 64]
        active_attempt_id -> Varchar,
        business_country -> Nullable<CountryAlpha2>,
        #[max_length = 64]
        business_label -> Nullable<Varchar>,
        order_details -> Nullable<Array<Nullable<Jsonb>>>,
        allowed_payment_method_types -> Nullable<Json>,
        connector_metadata -> Nullable<Json>,
        feature_metadata -> Nullable<Json>,
        attempt_count -> Int2,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 64]
        merchant_decision -> Nullable<Varchar>,
        #[max_length = 255]
        payment_link_id -> Nullable<Varchar>,
        payment_confirm_source -> Nullable<PaymentSource>,
        #[max_length = 32]
        updated_by -> Varchar,
        surcharge_applicable -> Nullable<Bool>,
        request_incremental_authorization -> Nullable<RequestIncrementalAuthorization>,
        incremental_authorization_allowed -> Nullable<Bool>,
        authorization_count -> Nullable<Int4>,
        session_expiry -> Nullable<Timestamp>,
        #[max_length = 64]
        fingerprint_id -> Nullable<Varchar>,
        request_external_three_ds_authentication -> Nullable<Bool>,
        charges -> Nullable<Jsonb>,
        frm_metadata -> Nullable<Jsonb>,
        customer_details -> Nullable<Bytea>,
        billing_details -> Nullable<Bytea>,
        #[max_length = 255]
        merchant_order_reference_id -> Nullable<Varchar>,
        shipping_details -> Nullable<Bytea>,
        is_payment_processor_token_flow -> Nullable<Bool>,
        shipping_cost -> Nullable<Int8>,
        #[max_length = 32]
        organization_id -> Varchar,
        tax_details -> Nullable<Jsonb>,
        skip_external_tax_calculation -> Nullable<Bool>,
        psd2_sca_exemption_type -> Nullable<ScaExemptionType>,
        split_payments -> Nullable<Jsonb>,
        #[max_length = 64]
        platform_merchant_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payment_link (payment_link_id) {
        #[max_length = 255]
        payment_link_id -> Varchar,
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 255]
        link_to_pay -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        amount -> Int8,
        currency -> Nullable<Currency>,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        fulfilment_time -> Nullable<Timestamp>,
        #[max_length = 64]
        custom_merchant_name -> Nullable<Varchar>,
        payment_link_config -> Nullable<Jsonb>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 255]
        secure_link -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payment_methods (payment_method_id) {
        #[max_length = 64]
        customer_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        payment_method_id -> Varchar,
        accepted_currency -> Nullable<Array<Nullable<Currency>>>,
        #[max_length = 32]
        scheme -> Nullable<Varchar>,
        #[max_length = 128]
        token -> Nullable<Varchar>,
        #[max_length = 255]
        cardholder_name -> Nullable<Varchar>,
        #[max_length = 64]
        issuer_name -> Nullable<Varchar>,
        #[max_length = 64]
        issuer_country -> Nullable<Varchar>,
        payer_country -> Nullable<Array<Nullable<Text>>>,
        is_stored -> Nullable<Bool>,
        #[max_length = 32]
        swift_code -> Nullable<Varchar>,
        #[max_length = 128]
        direct_debit_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_modified -> Timestamp,
        payment_method -> Nullable<Varchar>,
        #[max_length = 64]
        payment_method_type -> Nullable<Varchar>,
        #[max_length = 128]
        payment_method_issuer -> Nullable<Varchar>,
        payment_method_issuer_code -> Nullable<PaymentMethodIssuerCode>,
        metadata -> Nullable<Json>,
        payment_method_data -> Nullable<Bytea>,
        #[max_length = 64]
        locker_id -> Nullable<Varchar>,
        last_used_at -> Timestamp,
        connector_mandate_details -> Nullable<Jsonb>,
        customer_acceptance -> Nullable<Jsonb>,
        #[max_length = 64]
        status -> Varchar,
        #[max_length = 255]
        network_transaction_id -> Nullable<Varchar>,
        #[max_length = 128]
        client_secret -> Nullable<Varchar>,
        payment_method_billing_address -> Nullable<Bytea>,
        #[max_length = 64]
        updated_by -> Nullable<Varchar>,
        version -> ApiVersion,
        #[max_length = 128]
        network_token_requestor_reference_id -> Nullable<Varchar>,
        #[max_length = 64]
        network_token_locker_id -> Nullable<Varchar>,
        network_token_payment_method_data -> Nullable<Bytea>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payout_attempt (payout_attempt_id) {
        #[max_length = 64]
        payout_attempt_id -> Varchar,
        #[max_length = 64]
        payout_id -> Varchar,
        #[max_length = 64]
        customer_id -> Nullable<Varchar>,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        address_id -> Nullable<Varchar>,
        #[max_length = 64]
        connector -> Nullable<Varchar>,
        #[max_length = 128]
        connector_payout_id -> Nullable<Varchar>,
        #[max_length = 64]
        payout_token -> Nullable<Varchar>,
        status -> PayoutStatus,
        is_eligible -> Nullable<Bool>,
        error_message -> Nullable<Text>,
        #[max_length = 64]
        error_code -> Nullable<Varchar>,
        business_country -> Nullable<CountryAlpha2>,
        #[max_length = 64]
        business_label -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
        routing_info -> Nullable<Jsonb>,
        #[max_length = 255]
        unified_code -> Nullable<Varchar>,
        #[max_length = 1024]
        unified_message -> Nullable<Varchar>,
        additional_payout_method_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    payouts (payout_id) {
        #[max_length = 64]
        payout_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        customer_id -> Nullable<Varchar>,
        #[max_length = 64]
        address_id -> Nullable<Varchar>,
        payout_type -> Nullable<PayoutType>,
        #[max_length = 64]
        payout_method_id -> Nullable<Varchar>,
        amount -> Int8,
        destination_currency -> Currency,
        source_currency -> Currency,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        recurring -> Bool,
        auto_fulfill -> Bool,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        #[max_length = 64]
        entity_type -> Varchar,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        attempt_count -> Int2,
        #[max_length = 64]
        profile_id -> Varchar,
        status -> PayoutStatus,
        confirm -> Nullable<Bool>,
        #[max_length = 255]
        payout_link_id -> Nullable<Varchar>,
        #[max_length = 128]
        client_secret -> Nullable<Varchar>,
        #[max_length = 32]
        priority -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    process_tracker (id) {
        #[max_length = 127]
        id -> Varchar,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        tag -> Array<Nullable<Text>>,
        #[max_length = 64]
        runner -> Nullable<Varchar>,
        retry_count -> Int4,
        schedule_time -> Nullable<Timestamp>,
        #[max_length = 255]
        rule -> Varchar,
        tracking_data -> Json,
        #[max_length = 255]
        business_status -> Varchar,
        status -> ProcessTrackerStatus,
        event -> Array<Nullable<Text>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    refund (merchant_id, refund_id) {
        #[max_length = 64]
        internal_reference_id -> Varchar,
        #[max_length = 64]
        refund_id -> Varchar,
        #[max_length = 64]
        payment_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 128]
        connector_transaction_id -> Varchar,
        #[max_length = 64]
        connector -> Varchar,
        #[max_length = 128]
        connector_refund_id -> Nullable<Varchar>,
        #[max_length = 64]
        external_reference_id -> Nullable<Varchar>,
        refund_type -> RefundType,
        total_amount -> Int8,
        currency -> Currency,
        refund_amount -> Int8,
        refund_status -> RefundStatus,
        sent_to_gateway -> Bool,
        refund_error_message -> Nullable<Text>,
        metadata -> Nullable<Json>,
        #[max_length = 128]
        refund_arn -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 64]
        attempt_id -> Varchar,
        #[max_length = 255]
        refund_reason -> Nullable<Varchar>,
        refund_error_code -> Nullable<Text>,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 32]
        updated_by -> Varchar,
        #[max_length = 32]
        merchant_connector_id -> Nullable<Varchar>,
        charges -> Nullable<Jsonb>,
        #[max_length = 32]
        organization_id -> Varchar,
        #[max_length = 512]
        connector_refund_data -> Nullable<Varchar>,
        #[max_length = 512]
        connector_transaction_data -> Nullable<Varchar>,
        split_refunds -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    relay (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 128]
        connector_resource_id -> Varchar,
        #[max_length = 64]
        connector_id -> Varchar,
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        relay_type -> RelayType,
        request_data -> Nullable<Jsonb>,
        status -> RelayStatus,
        #[max_length = 128]
        connector_reference_id -> Nullable<Varchar>,
        #[max_length = 64]
        error_code -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        response_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    reverse_lookup (lookup_id) {
        #[max_length = 128]
        lookup_id -> Varchar,
        #[max_length = 128]
        sk_id -> Varchar,
        #[max_length = 128]
        pk_id -> Varchar,
        #[max_length = 128]
        source -> Varchar,
        #[max_length = 32]
        updated_by -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    roles (role_id) {
        #[max_length = 64]
        role_name -> Varchar,
        #[max_length = 64]
        role_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        org_id -> Varchar,
        groups -> Array<Nullable<Text>>,
        scope -> RoleScope,
        created_at -> Timestamp,
        #[max_length = 64]
        created_by -> Varchar,
        last_modified_at -> Timestamp,
        #[max_length = 64]
        last_modified_by -> Varchar,
        #[max_length = 64]
        entity_type -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    routing_algorithm (algorithm_id) {
        #[max_length = 64]
        algorithm_id -> Varchar,
        #[max_length = 64]
        profile_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        kind -> RoutingAlgorithmKind,
        algorithm_data -> Jsonb,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        algorithm_for -> TransactionType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    themes (theme_id) {
        #[max_length = 64]
        theme_id -> Varchar,
        #[max_length = 64]
        tenant_id -> Varchar,
        #[max_length = 64]
        org_id -> Nullable<Varchar>,
        #[max_length = 64]
        merchant_id -> Nullable<Varchar>,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        #[max_length = 64]
        entity_type -> Varchar,
        #[max_length = 64]
        theme_name -> Varchar,
        #[max_length = 64]
        email_primary_color -> Varchar,
        #[max_length = 64]
        email_foreground_color -> Varchar,
        #[max_length = 64]
        email_background_color -> Varchar,
        #[max_length = 64]
        email_entity_name -> Varchar,
        email_entity_logo_url -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    unified_translations (unified_code, unified_message, locale) {
        #[max_length = 255]
        unified_code -> Varchar,
        #[max_length = 1024]
        unified_message -> Varchar,
        #[max_length = 255]
        locale -> Varchar,
        #[max_length = 1024]
        translation -> Varchar,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    user_authentication_methods (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 64]
        auth_id -> Varchar,
        #[max_length = 64]
        owner_id -> Varchar,
        #[max_length = 64]
        owner_type -> Varchar,
        #[max_length = 64]
        auth_type -> Varchar,
        private_config -> Nullable<Bytea>,
        public_config -> Nullable<Jsonb>,
        allow_signup -> Bool,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        #[max_length = 64]
        email_domain -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    user_key_store (user_id) {
        #[max_length = 64]
        user_id -> Varchar,
        key -> Bytea,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    user_roles (id) {
        id -> Int4,
        #[max_length = 64]
        user_id -> Varchar,
        #[max_length = 64]
        merchant_id -> Nullable<Varchar>,
        #[max_length = 64]
        role_id -> Varchar,
        #[max_length = 64]
        org_id -> Nullable<Varchar>,
        status -> UserStatus,
        #[max_length = 64]
        created_by -> Varchar,
        #[max_length = 64]
        last_modified_by -> Varchar,
        created_at -> Timestamp,
        last_modified -> Timestamp,
        #[max_length = 64]
        profile_id -> Nullable<Varchar>,
        #[max_length = 64]
        entity_id -> Nullable<Varchar>,
        #[max_length = 64]
        entity_type -> Nullable<Varchar>,
        version -> UserRoleVersion,
        #[max_length = 64]
        tenant_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::enums::diesel_exports::*;

    users (user_id) {
        #[max_length = 64]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        is_verified -> Bool,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        totp_status -> TotpStatus,
        totp_secret -> Nullable<Bytea>,
        totp_recovery_codes -> Nullable<Array<Nullable<Text>>>,
        last_password_modified_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    address,
    api_keys,
    authentication,
    blocklist,
    blocklist_fingerprint,
    blocklist_lookup,
    business_profile,
    captures,
    cards_info,
    configs,
    customers,
    dashboard_metadata,
    dispute,
    dynamic_routing_stats,
    events,
    file_metadata,
    fraud_check,
    gateway_status_map,
    generic_link,
    incremental_authorization,
    locker_mock_up,
    mandate,
    merchant_account,
    merchant_connector_account,
    merchant_key_store,
    organization,
    payment_attempt,
    payment_intent,
    payment_link,
    payment_methods,
    payout_attempt,
    payouts,
    process_tracker,
    refund,
    relay,
    reverse_lookup,
    roles,
    routing_algorithm,
    themes,
    unified_translations,
    user_authentication_methods,
    user_key_store,
    user_roles,
    users,
);
