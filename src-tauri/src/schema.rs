// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Integer,
        player -> Text,
        hand -> BigInt,
        kind -> Text,
        moment -> Text,
        sequence -> Integer,
        amount1 -> Float,
        amount2 -> Float,
        allin -> Bool,
    }
}

diesel::table! {
    blind (id) {
        id -> Integer,
        player -> Text,
        hand -> BigInt,
        amount -> Float,
        kind -> Text,
    }
}

diesel::table! {
    hand (id) {
        id -> BigInt,
        content -> Text,
        real_money -> Bool,
        time -> BigInt,
        table_name -> Text,
        table_size -> Integer,
        winner -> Text,
        pot -> Float,
        player1 -> Text,
        player2 -> Text,
        player3 -> Text,
        player4 -> Text,
        player5 -> Text,
        player6 -> Text,
        player7 -> Text,
        player8 -> Text,
        player9 -> Text,
        card1 -> Text,
        card2 -> Text,
        card3 -> Text,
        card4 -> Text,
        card5 -> Text,
    }
}

diesel::table! {
    holeCard (id) {
        id -> Integer,
        hand -> BigInt,
        player -> Text,
        card1 -> Text,
        card2 -> Text,
    }
}

diesel::table! {
    player (name, real_money) {
        name -> Text,
        real_money -> Bool,
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

diesel::allow_tables_to_appear_in_same_query!(
    action,
    blind,
    hand,
    holeCard,
    player,
);
