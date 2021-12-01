table! {
    users (id) {
        id -> Nullable<Integer>,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
    }
}

table! {
    users_oauth_github (github_id) {
        user_id -> Integer,
        github_id -> Integer,
    }
}

joinable!(users_oauth_github -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    users_oauth_github,
);
