use std::collections::HashMap;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Type, UseTree};

use generation::attribute::FeatureTag;
use generation::class::{Class, GtkImport};
use generation::prop::Property;
use generation::signal::Signal;
use crate::schema::{
    class_inherit_features, class_inherits, classes, imports, properties, property_features,
    signal_features, signals,
};

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = classes)]
pub struct ClassRow {
    pub id: i32,
    pub name: String,
    pub constructible: bool,
    pub file_name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = classes)]
pub struct NewClass<'a> {
    pub name: &'a str,
    pub constructible: bool,
    pub file_name: &'a str,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = class_inherits)]
pub struct ClassInheritRow {
    pub id: i32,
    pub class_id: i32,
    pub parent_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = class_inherits)]
pub struct NewClassInherit {
    pub class_id: i32,
    pub parent_id: i32,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = class_inherit_features)]
pub struct ClassInheritFeatureRow {
    pub id: i32,
    pub class_inherit_id: i32,
    pub feature: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = class_inherit_features)]
pub struct NewClassInheritFeature {
    pub class_inherit_id: i32,
    pub feature: String,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = properties)]
pub struct PropertyRow {
    pub id: i32,
    pub class_id: i32,
    pub name: String,
    pub ty: String,
    pub attrs: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = properties)]
pub struct NewProperty<'a> {
    pub class_id: i32,
    pub name: &'a str,
    pub ty: &'a str,
    pub attrs: &'a str,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = property_features)]
pub struct PropertyFeatureRow {
    pub id: i32,
    pub property_id: i32,
    pub feature: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = property_features)]
pub struct NewPropertyFeature {
    pub property_id: i32,
    pub feature: String,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = signals)]
pub struct SignalRow {
    pub id: i32,
    pub class_id: i32,
    pub name: String,
    pub args: String,
    pub ret: String,
    pub attrs: String,
    pub fn_bound: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = signals)]
pub struct NewSignal<'a> {
    pub class_id: i32,
    pub name: &'a str,
    pub args: &'a str,
    pub ret: &'a str,
    pub attrs: &'a str,
    pub fn_bound: &'a str,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = signal_features)]
pub struct SignalFeatureRow {
    pub id: i32,
    pub signal_id: i32,
    pub feature: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = signal_features)]
pub struct NewSignalFeature {
    pub signal_id: i32,
    pub feature: String,
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = imports)]
pub struct ImportRow {
    pub id: i32,
    pub class_id: i32,
    pub tree: String,
    pub attrs: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = imports)]
pub struct NewImport<'a> {
    pub class_id: i32,
    pub tree: &'a str,
    pub attrs: &'a str,
}

pub fn connect(path: impl AsRef<Path>) -> SqliteConnection {
    SqliteConnection::establish(path.as_ref().to_str().unwrap())
        .expect("Failed to connect to SQLite database")
}

fn last_insert_rowid(connection: &mut SqliteConnection) -> i32 {
    diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("last_insert_rowid()"))
        .get_result(connection)
        .unwrap()
}

pub fn initialize(path: impl AsRef<Path>) {
    let mut connection = connect(&path);

    diesel::sql_query("CREATE TABLE IF NOT EXISTS classes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        constructible BOOLEAN NOT NULL DEFAULT 0,
        file_name TEXT NOT NULL
    )").execute(&mut connection).unwrap();

    // Migrate from the old comma-separated `inherits` column (if present).
    let has_inherits: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
        "CASE WHEN EXISTS (SELECT 1 FROM pragma_table_info('classes') WHERE name = 'inherits') \
         THEN 1 ELSE 0 END",
    ))
        .get_result(&mut connection)
        .unwrap();
    if has_inherits > 0 {
        diesel::sql_query("ALTER TABLE classes DROP COLUMN inherits")
            .execute(&mut connection)
            .unwrap();
    }

    // Migrate the `*_features` tables away from the old flat `inverted` column
    // to the serialized `FeatureTag` expression stored in `feature`.
    // SQLite cannot `DROP COLUMN` a column that participates in a UNIQUE
    // constraint, so we rebuild each table without `inverted` instead.
    for (table, parent_col) in [
        ("class_inherit_features", "class_inherit_id"),
        ("property_features", "property_id"),
        ("signal_features", "signal_id"),
    ] {
        let has_inverted: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            &format!(
                "CASE WHEN EXISTS (SELECT 1 FROM pragma_table_info('{table}') \
                 WHERE name = 'inverted') THEN 1 ELSE 0 END"
            ),
        ))
            .get_result(&mut connection)
            .unwrap();
        if has_inverted > 0 {
            diesel::sql_query(&format!(
                "ALTER TABLE {table} RENAME TO {table}_old"
            ))
                .execute(&mut connection)
                .unwrap();
            diesel::sql_query(&format!(
                "CREATE TABLE {table} (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    {parent_col} INTEGER NOT NULL,
                    feature TEXT NOT NULL,
                    UNIQUE({parent_col}, feature)
                )"
            ))
                .execute(&mut connection)
                .unwrap();
            diesel::sql_query(&format!(
                "INSERT INTO {table} (id, {parent_col}, feature) \
                 SELECT id, {parent_col}, feature FROM {table}_old"
            ))
                .execute(&mut connection)
                .unwrap();
            diesel::sql_query(&format!("DROP TABLE {table}_old"))
                .execute(&mut connection)
                .unwrap();
        }
    }

    diesel::sql_query("CREATE TABLE IF NOT EXISTS class_inherits (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        class_id INTEGER NOT NULL REFERENCES classes(id),
        parent_id INTEGER NOT NULL REFERENCES classes(id)
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS class_inherit_features (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        class_inherit_id INTEGER NOT NULL REFERENCES class_inherits(id),
        feature TEXT NOT NULL,
        UNIQUE(class_inherit_id, feature)
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS properties (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        class_id INTEGER NOT NULL REFERENCES classes(id),
        name TEXT NOT NULL,
        ty TEXT NOT NULL,
        attrs TEXT NOT NULL
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS property_features (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        property_id INTEGER NOT NULL REFERENCES properties(id),
        feature TEXT NOT NULL,
        UNIQUE(property_id, feature)
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS signals (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        class_id INTEGER NOT NULL REFERENCES classes(id),
        name TEXT NOT NULL,
        args TEXT NOT NULL,
        ret TEXT NOT NULL,
        attrs TEXT NOT NULL,
        fn_bound TEXT NOT NULL
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS signal_features (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        signal_id INTEGER NOT NULL REFERENCES signals(id),
        feature TEXT NOT NULL,
        UNIQUE(signal_id, feature)
    )").execute(&mut connection).unwrap();

    diesel::sql_query("CREATE TABLE IF NOT EXISTS imports (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        class_id INTEGER NOT NULL REFERENCES classes(id),
        tree TEXT NOT NULL,
        attrs TEXT NOT NULL
    )").execute(&mut connection).unwrap();
}

/// Serialize a single `syn` token-based type into a stable, parseable string.
fn to_text(tokens: &TokenStream) -> String {
    tokens.to_string()
}

fn type_to_text(ty: &Type) -> String {
    to_text(&ty.to_token_stream())
}

fn attrs_to_text(attrs: &[Attribute]) -> String {
    to_text(&quote::quote!( #( #attrs )* ))
}

fn types_to_text(types: &[Type]) -> String {
    to_text(&quote::quote!( #( #types ),* ))
}

fn ret_to_text(ret: &syn::ReturnType) -> String {
    to_text(&ret.to_token_stream())
}

fn use_tree_to_text(tree: &UseTree) -> String {
    to_text(&tree.to_token_stream())
}

/// Parse a string back into `Vec<Attribute>` by attaching them to a stub function.
fn parse_attrs(text: &str) -> Vec<Attribute> {
    if text.is_empty() {
        return vec![];
    }
    let ts: TokenStream = syn::parse_str(text)
        .unwrap_or_else(|err| panic!("Failed to parse stored attributes `{text}`: {err}"));
    let item: syn::ItemFn = syn::parse2(quote::quote!( #ts fn __ousia_stub() {} ))
        .unwrap_or_else(|err| panic!("Failed to parse stored attributes `{text}`: {err}"));
    item.attrs
}

/// Parse a string back into a `Type`.
fn parse_type(text: &str) -> Type {
    syn::parse_str(text)
        .unwrap_or_else(|err| panic!("Failed to parse stored Type `{text}`: {err}"))
}

/// Parse a comma-separated list of types back into `Vec<Type>`.
fn parse_types(text: &str) -> Vec<Type> {
    if text.is_empty() {
        return vec![];
    }
    let ts: TokenStream = syn::parse_str(text)
        .unwrap_or_else(|err| panic!("Failed to parse stored types `{text}`: {err}"));
    let item: syn::ItemStruct = syn::parse2(quote::quote!( struct __OusiaStub( #ts ); ))
        .unwrap_or_else(|err| panic!("Failed to parse stored types `{text}`: {err}"));
    match item.fields {
        syn::Fields::Unnamed(fields) => fields.unnamed.into_iter().map(|f| f.ty).collect(),
        _ => unreachable!(),
    }
}

fn parse_return_type(text: &str) -> syn::ReturnType {
    if text.is_empty() {
        return syn::ReturnType::Default;
    }
    syn::parse_str(text)
        .unwrap_or_else(|err| panic!("Failed to parse stored ReturnType `{text}`: {err}"))
}

fn parse_use_tree(text: &str) -> UseTree {
    syn::parse_str(text)
        .unwrap_or_else(|err| panic!("Failed to parse stored UseTree `{text}`: {err}"))
}

fn feature_to_text(feat: &FeatureTag) -> String {
    feat.to_expr_string()
}

fn parse_feature_rows<'a, I>(rows: I) -> Vec<FeatureTag>
where
    I: IntoIterator<Item = String>,
{
    rows.into_iter()
        .map(|feature| {
            FeatureTag::from_expr_string(&feature)
                .unwrap_or_else(|err| panic!("Failed to parse stored feature `{feature}`: {err}"))
        })
        .collect()
}

/// Insert the given classes (with their properties, signals and imports) into the database.
pub fn insert_classes(path: impl AsRef<Path>, classes: &[(std::path::PathBuf, Class)]) -> diesel::QueryResult<()> {
    let mut connection = connect(path);

    connection.transaction(|connection| {
        // Pass 1: upsert every class and record its id by name.
        let mut ids_by_name = HashMap::new();
        for (path, class) in classes {
            let file_name = path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();

            let new_class = NewClass {
                name: &class.name,
                constructible: class.constructible,
                file_name: &file_name,
            };

            diesel::insert_into(classes::table)
                .values(&new_class)
                .on_conflict(classes::name)
                .do_update()
                .set((
                    classes::constructible.eq(new_class.constructible),
                    classes::file_name.eq(&new_class.file_name),
                ))
                .execute(connection)?;

            let class_id: i32 = classes::table
                .filter(classes::name.eq(&class.name))
                .select(classes::id)
                .first(connection)?;

            ids_by_name.insert(class.name.clone(), class_id);
        }

        // Pass 2: clear and re-insert each class's children so re-indexing is idempotent.
        for (_, class) in classes {
            let class_id = ids_by_name[&class.name];

            // Clear inheritance rows (and their features).
            let inherit_ids: Vec<i32> = class_inherits::table
                .filter(class_inherits::class_id.eq(class_id))
                .select(class_inherits::id)
                .load(connection)?;
            diesel::delete(class_inherit_features::table
                .filter(class_inherit_features::class_inherit_id.eq_any(&inherit_ids)))
                .execute(connection)?;
            diesel::delete(class_inherits::table
                .filter(class_inherits::class_id.eq(class_id)))
                .execute(connection)?;

            // Clear property rows (and their features).
            let prop_ids: Vec<i32> = properties::table
                .filter(properties::class_id.eq(class_id))
                .select(properties::id)
                .load(connection)?;
            diesel::delete(property_features::table
                .filter(property_features::property_id.eq_any(&prop_ids)))
                .execute(connection)?;
            diesel::delete(properties::table
                .filter(properties::class_id.eq(class_id)))
                .execute(connection)?;

            // Clear signal rows (and their features).
            let signal_ids: Vec<i32> = signals::table
                .filter(signals::class_id.eq(class_id))
                .select(signals::id)
                .load(connection)?;
            diesel::delete(signal_features::table
                .filter(signal_features::signal_id.eq_any(&signal_ids)))
                .execute(connection)?;
            diesel::delete(signals::table
                .filter(signals::class_id.eq(class_id)))
                .execute(connection)?;

            // Clear imports for the class.
            diesel::delete(imports::table
                .filter(imports::class_id.eq(class_id)))
                .execute(connection)?;

            // Insert inherits (each entry keyed by its feature tags).
            for (features, parents) in &class.inherits {
                for parent in parents {
                    if let Some(&parent_id) = ids_by_name.get(parent) {
                        let new_inherit = NewClassInherit { class_id, parent_id };
                        diesel::insert_into(class_inherits::table)
                            .values(&new_inherit)
                            .execute(connection)?;
                        let inherit_id = last_insert_rowid(connection);
                        for feat in features {
                            let new_feature = NewClassInheritFeature {
                                class_inherit_id: inherit_id,
                                feature: feature_to_text(feat),
                            };
                            diesel::insert_into(class_inherit_features::table)
                                .values(&new_feature)
                                .on_conflict_do_nothing()
                                .execute(connection)?;
                        }
                    }
                }
            }

            // Insert properties (each tagged).
            for (name, tagged) in &class.setters {
                let new_prop = NewProperty {
                    class_id,
                    name,
                    ty: &type_to_text(&tagged.item.ty),
                    attrs: &attrs_to_text(&tagged.item.attrs),
                };
                diesel::insert_into(properties::table)
                    .values(&new_prop)
                    .execute(connection)?;
                let property_id = last_insert_rowid(connection);
                for feat in &tagged.tag {
                    let new_feature = NewPropertyFeature {
                        property_id,
                        feature: feature_to_text(feat),
                    };
                    diesel::insert_into(property_features::table)
                        .values(&new_feature)
                        .on_conflict_do_nothing()
                        .execute(connection)?;
                }
            }

            // Insert signals (each tagged).
            for (name, tagged) in &class.signals {
                let new_signal = NewSignal {
                    class_id,
                    name,
                    args: &types_to_text(&tagged.item.args),
                    ret: &ret_to_text(&tagged.item.ret),
                    attrs: &attrs_to_text(&tagged.item.attrs),
                    fn_bound: &tagged.item.fn_bound,
                };
                diesel::insert_into(signals::table)
                    .values(&new_signal)
                    .execute(connection)?;
                let signal_id = last_insert_rowid(connection);
                for feat in &tagged.tag {
                    let new_feature = NewSignalFeature {
                        signal_id,
                        feature: feature_to_text(feat),
                    };
                    diesel::insert_into(signal_features::table)
                        .values(&new_feature)
                        .on_conflict_do_nothing()
                        .execute(connection)?;
                }
            }

            for import in &class.used {
                let new_import = NewImport {
                    class_id,
                    tree: &use_tree_to_text(&import.tree),
                    attrs: &attrs_to_text(&import.attrs),
                };
                diesel::insert_into(imports::table)
                    .values(&new_import)
                    .execute(connection)?;
            }
        }

        Ok(())
    })
}

/// Load every class (with its properties, signals and imports) from the database.
pub fn load_classes(path: impl AsRef<Path>) -> diesel::QueryResult<Vec<(String, Class)>> {
    let mut connection = connect(path);

    let class_rows = classes::table
        .order(classes::id)
        .load::<ClassRow>(&mut connection)?;

    let mut result = Vec::with_capacity(class_rows.len());

    for row in class_rows {
        // Inherits: each row is (feature-set -> parents).
        let inherit_rows = class_inherits::table
            .filter(class_inherits::class_id.eq(row.id))
            .inner_join(classes::table.on(class_inherits::parent_id.eq(classes::id)))
            .select((class_inherits::id, classes::name))
            .load::<(i32, String)>(&mut connection)?;

        let mut inherits: HashMap<Vec<FeatureTag>, Vec<String>> = HashMap::new();
        for (inherit_id, parent_name) in inherit_rows {
            let feats = class_inherit_features::table
                .filter(class_inherit_features::class_inherit_id.eq(inherit_id))
                .order(class_inherit_features::id)
                .select(class_inherit_features::feature)
                .load::<String>(&mut connection)?;
            let feats = parse_feature_rows(feats);
            inherits.entry(feats).or_default().push(parent_name);
        }

        // Properties (tagged).
        let prop_rows = properties::table
            .filter(properties::class_id.eq(row.id))
            .order(properties::id)
            .load::<PropertyRow>(&mut connection)?;

        let mut setters = HashMap::new();
        for prop in prop_rows {
            let tags = property_features::table
                .filter(property_features::property_id.eq(prop.id))
                .order(property_features::id)
                .select(property_features::feature)
                .load::<String>(&mut connection)?;
            setters.insert(
                prop.name.clone(),
                generation::attribute::Tagged::new(
                    parse_feature_rows(tags),
                    Property {
                        name: prop.name,
                        ty: parse_type(&prop.ty),
                        attrs: parse_attrs(&prop.attrs),
                    },
                ),
            );
        }

        // Signals (tagged).
        let signal_rows = signals::table
            .filter(signals::class_id.eq(row.id))
            .order(signals::id)
            .load::<SignalRow>(&mut connection)?;

        let mut signals_map = HashMap::new();
        for sig in signal_rows {
            let tags = signal_features::table
                .filter(signal_features::signal_id.eq(sig.id))
                .order(signal_features::id)
                .select(signal_features::feature)
                .load::<String>(&mut connection)?;
            signals_map.insert(
                sig.name.clone(),
                generation::attribute::Tagged::new(
                    parse_feature_rows(tags),
                    Signal {
                        name: sig.name,
                        args: parse_types(&sig.args),
                        ret: parse_return_type(&sig.ret),
                        attrs: parse_attrs(&sig.attrs),
                        fn_bound: sig.fn_bound,
                    },
                ),
            );
        }

        // Imports.
        let import_rows = imports::table
            .filter(imports::class_id.eq(row.id))
            .order(imports::id)
            .load::<ImportRow>(&mut connection)?;

        let mut used = Vec::with_capacity(import_rows.len());
        for import in import_rows {
            used.push(GtkImport {
                tree: parse_use_tree(&import.tree),
                attrs: parse_attrs(&import.attrs),
            });
        }

        let class = Class {
            name: row.name,
            setters,
            used,
            signals: signals_map,
            inherits,
            constructible: row.constructible,
        };

        result.push((row.file_name, class));
    }

    Ok(result)
}
