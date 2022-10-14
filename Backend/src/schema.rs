// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        product_name -> Varchar,
        product_price -> Numeric,
    }
}
