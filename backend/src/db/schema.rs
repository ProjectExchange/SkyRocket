table! {
    users (id) {
        id -> Integer,
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

table! {
    users_roles (user_id, role_id) {
        user_id -> Integer,
        role_id -> Integer,
    }
}

joinable!(users_oauth_github -> users (user_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    users_oauth_github,
    users_roles,
);
