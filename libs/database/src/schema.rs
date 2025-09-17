// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Uuid,
        zap_id -> Uuid,
        action_id -> Uuid,
        metadata -> Jsonb,
        sorting_order -> Int4,
    }
}

diesel::table! {
    available_action (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    available_trigger (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    trigger (id) {
        id -> Uuid,
        zap_id -> Uuid,
        trigger_id -> Uuid,
        metadata -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    zap (id) {
        id -> Uuid,
        trigger_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    zap_run (id) {
        id -> Uuid,
        zap_id -> Uuid,
        metadata -> Jsonb,
    }
}

diesel::table! {
    zap_run_outbox (id) {
        id -> Uuid,
        zap_run_id -> Uuid,
    }
}

diesel::joinable!(action -> available_action (action_id));
diesel::joinable!(action -> zap (zap_id));
diesel::joinable!(trigger -> available_trigger (trigger_id));
diesel::joinable!(trigger -> zap (zap_id));
diesel::joinable!(zap -> users (user_id));
diesel::joinable!(zap_run -> zap (zap_id));
diesel::joinable!(zap_run_outbox -> zap_run (zap_run_id));

diesel::allow_tables_to_appear_in_same_query!(
    action,
    available_action,
    available_trigger,
    trigger,
    users,
    zap,
    zap_run,
    zap_run_outbox,
);
