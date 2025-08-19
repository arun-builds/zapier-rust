// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Text,
        zap_id -> Text,
        action_id -> Text,
    }
}

diesel::table! {
    available_action (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    available_trigger (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    trigger (id) {
        id -> Text,
        zap_id -> Text,
        trigger_id -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    zap (id) {
        id -> Text,
        trigger_id -> Text,
    }
}

diesel::table! {
    zap_run (id) {
        id -> Text,
        zap_id -> Text,
        metadata -> Jsonb,
    }
}

diesel::table! {
    zap_run_outbox (id) {
        id -> Text,
        zap_run_id -> Text,
    }
}

diesel::joinable!(action -> available_action (action_id));
diesel::joinable!(action -> zap (zap_id));
diesel::joinable!(trigger -> available_trigger (trigger_id));
diesel::joinable!(trigger -> zap (zap_id));
diesel::joinable!(zap_run -> zap (zap_id));
diesel::joinable!(zap_run_outbox -> zap_run (zap_run_id));

diesel::allow_tables_to_appear_in_same_query!(
    action,
    available_action,
    available_trigger,
    trigger,
    user,
    zap,
    zap_run,
    zap_run_outbox,
);
