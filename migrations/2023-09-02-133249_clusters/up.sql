CREATE TABLE clusters (
    id UUID PRIMARY KEY,
    name CHARACTER VARYING(255) NOT NULL,
    address CHARACTER VARYING(255) NOT NULL,
    certificate_authority_data TEXT NOT NULL,
    user_name CHARACTER VARYING(255) NOT NULL,
    user_credentials_certificate_data TEXT NOT NULL,
    user_crendetiral_key_data TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

SELECT diesel_manage_updated_at('clusters');