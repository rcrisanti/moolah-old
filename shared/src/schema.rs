table! {
    delta_dates (delta_id, date) {
        delta_id -> Int4,
        date -> Date,
    }
}

table! {
    deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
    }
}

table! {
    predictions (id) {
        id -> Int4,
        username -> Varchar,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created -> Timestamp,
        last_login -> Timestamp,
    }
}

joinable!(delta_dates -> deltas (delta_id));
joinable!(deltas -> predictions (prediction_id));

allow_tables_to_appear_in_same_query!(
    delta_dates,
    deltas,
    predictions,
    users,
);
