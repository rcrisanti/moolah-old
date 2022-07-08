table! {
    daily_deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
        start_on -> Date,
        end_on -> Date,
    }
}

table! {
    monthly_deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
        start_on -> Date,
        end_on -> Date,
        repeat_day -> Int2,
    }
}

table! {
    once_deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
        start_on -> Date,
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

table! {
    weekly_deltas (id) {
        id -> Int4,
        prediction_id -> Int4,
        name -> Varchar,
        value -> Float4,
        positive_uncertainty -> Float4,
        negative_uncertainty -> Float4,
        start_on -> Date,
        end_on -> Date,
        repeat_weekday -> Int2,
    }
}

joinable!(daily_deltas -> predictions (prediction_id));
joinable!(monthly_deltas -> predictions (prediction_id));
joinable!(once_deltas -> predictions (prediction_id));
joinable!(weekly_deltas -> predictions (prediction_id));

allow_tables_to_appear_in_same_query!(
    daily_deltas,
    monthly_deltas,
    once_deltas,
    predictions,
    users,
    weekly_deltas,
);
