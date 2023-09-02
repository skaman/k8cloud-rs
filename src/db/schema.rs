// @generated automatically by Diesel CLI.

diesel::table! {
    clusters (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        certificate_authority_data -> Text,
        #[max_length = 255]
        user_name -> Varchar,
        user_credentials_certificate_data -> Text,
        user_crendetiral_key_data -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
