diesel::table! {
    classes (id) {
        id -> Integer,
        name -> Text,
        constructible -> Bool,
        file_name -> Text,
    }
}

diesel::table! {
    class_inherits (id) {
        id -> Integer,
        class_id -> Integer,
        parent_id -> Integer,
    }
}

diesel::table! {
    class_inherit_features (id) {
        id -> Integer,
        class_inherit_id -> Integer,
        feature -> Text,
    }
}

diesel::table! {
    properties (id) {
        id -> Integer,
        class_id -> Integer,
        name -> Text,
        ty -> Text,
        attrs -> Text,
    }
}

diesel::table! {
    property_features (id) {
        id -> Integer,
        property_id -> Integer,
        feature -> Text,
    }
}

diesel::table! {
    signals (id) {
        id -> Integer,
        class_id -> Integer,
        name -> Text,
        args -> Text,
        ret -> Text,
        attrs -> Text,
        fn_bound -> Text,
    }
}

diesel::table! {
    signal_features (id) {
        id -> Integer,
        signal_id -> Integer,
        feature -> Text,
    }
}

diesel::table! {
    imports (id) {
        id -> Integer,
        class_id -> Integer,
        tree -> Text,
        attrs -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    class_inherits,
    class_inherit_features,
    properties,
    property_features,
    signals,
    signal_features,
    imports,
);
