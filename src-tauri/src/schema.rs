// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Nullable<Integer>,
        player -> Text,
        hand -> Integer,
        kind -> Text,
        moment -> Text,
        sequence -> Integer,
        amount1 -> Nullable<Integer>,
        amount2 -> Nullable<Integer>,
        allin -> Nullable<Bool>,
    }
}

diesel::table! {
    blind (id) {
        id -> Nullable<Integer>,
        player -> Text,
        hand -> Integer,
        amount -> Integer,
        kind -> Text,
    }
}

diesel::table! {
    hand (id) {
        id -> Nullable<Integer>,
        time -> Integer,
        table_name -> Text,
        table_size -> Integer,
        winner -> Text,
        pot -> Integer,
        player1 -> Nullable<Text>,
        player2 -> Nullable<Text>,
        player3 -> Nullable<Text>,
        player4 -> Nullable<Text>,
        player5 -> Nullable<Text>,
        player6 -> Nullable<Text>,
        player7 -> Nullable<Text>,
        player8 -> Nullable<Text>,
        player9 -> Nullable<Text>,
        card1 -> Nullable<Text>,
        card2 -> Nullable<Text>,
        card3 -> Nullable<Text>,
        card4 -> Nullable<Text>,
        card5 -> Nullable<Text>,
    }
}

diesel::table! {
    holeCard (id) {
        id -> Nullable<Integer>,
        hand -> Integer,
        player -> Text,
        card1 -> Text,
        card2 -> Text,
    }
}

diesel::table! {
    player (name) {
        name -> Nullable<Text>,
        vpip -> Nullable<Float>,
        pfr -> Nullable<Float>,
        af -> Nullable<Float>,
        pre_3bet -> Nullable<Float>,
        fold_pre_3bet -> Nullable<Float>,
        cbet -> Nullable<Float>,
        fold_cbet -> Nullable<Float>,
        squeeze -> Nullable<Float>,
        nb_hand -> Nullable<Float>,
        nb_can_pre_3bet -> Nullable<Float>,
        nb_can_fold_pre_3bet -> Nullable<Float>,
        nb_can_cbet -> Nullable<Float>,
        nb_can_fold_cbet -> Nullable<Float>,
        nb_can_squeeze -> Nullable<Float>,
        nb_call -> Nullable<Float>,
        nb_bet -> Nullable<Float>,
        nb_raise -> Nullable<Float>,
    }
}

diesel::joinable!(action -> hand (hand));
diesel::joinable!(action -> player (player));
diesel::joinable!(blind -> hand (hand));
diesel::joinable!(blind -> player (player));
diesel::joinable!(holeCard -> hand (hand));
diesel::joinable!(holeCard -> player (player));

diesel::allow_tables_to_appear_in_same_query!(
    action,
    blind,
    hand,
    holeCard,
    player,
);
