table! {
    deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
        repetition -> Int2,
        start_on -> Date,
        end_on -> Nullable<Date>,
        repeat_day -> Nullable<Int2>,
        repeat_weekday -> Nullable<Varchar>,
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

joinable!(deltas -> predictions (prediction_id));

allow_tables_to_appear_in_same_query!(
    deltas,
    predictions,
    users,
);
