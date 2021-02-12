table! {
    translations (translation_keyword) {
        translation_id -> Int8,
        translation_keyword -> Varchar,
        translation_result -> Nullable<Json>,
    }
}
