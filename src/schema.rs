// @generated automatically by Diesel CLI.

diesel::table! {
    conditions (id, created_on) {
        id -> VarChar,
        created_on -> Timestamptz,
        location -> Text,
        temperature -> Nullable<Float8>,
        humidity -> Nullable<Float8>,
    }
}

diesel::table! {
    conditions_default (id, created_on) {
        id -> VarChar,
        created_on -> Timestamptz,
        location -> Text,
        temperature -> Nullable<Float8>,
        humidity -> Nullable<Float8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(conditions, conditions_default,);
