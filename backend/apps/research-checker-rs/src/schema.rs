// @generated automatically by Diesel CLI.

diesel::table! {
    retraction (id) {
        id -> Int4,
        title -> Nullable<Text>,
        affiliations -> Array<Nullable<Text>>,
        journal -> Nullable<Text>,
        countries -> Array<Nullable<Text>>,
        authors -> Array<Nullable<Text>>,
        link -> Nullable<Text>,
        doi -> Nullable<Varchar>,
        pubmed_id -> Nullable<Varchar>,
        issues -> Array<Nullable<Text>>,
    }
}
