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
        content -> Text,
        real_money -> Bool,
        time -> Integer,
        table_name -> Text,
        table_size -> Integer,
        winner -> Text,
        pot -> Float,
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
    #[allow(non_snake_case)]
    holeCard (id) {
        id -> Nullable<Integer>,
        hand -> Integer,
        player -> Text,
        card1 -> Text,
        card2 -> Text,
    }
}

diesel::table! {
    player (name, real_money) {
        name -> Nullable<Text>,
        real_money -> Nullable<Bool>,
        vpip -> Float,
        pfr -> Float,
        af -> Float,
        pre_3bet -> Float,
        fold_pre_3bet -> Float,
        cbet -> Float,
        fold_cbet -> Float,
        squeeze -> Float,
        nb_hand -> Float,
        nb_can_pre_3bet -> Float,
        nb_can_fold_pre_3bet -> Float,
        nb_can_cbet -> Float,
        nb_can_fold_cbet -> Float,
        nb_can_squeeze -> Float,
        nb_call -> Float,
        nb_bet -> Float,
        nb_raise -> Float,
    }
}

diesel::joinable!(action -> hand (hand));
diesel::joinable!(blind -> hand (hand));
diesel::joinable!(holeCard -> hand (hand));

diesel::allow_tables_to_appear_in_same_query!(action, blind, hand, holeCard, player,);
