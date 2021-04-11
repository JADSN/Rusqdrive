use syn::{punctuated::Punctuated, Attribute, Field, Meta, Token};

use super::field_struct::FieldStruct;

// * Make a Vec<String> based of Vec<bool>(populate with `output_true` and `output_false`)
pub fn vec_bool_to_vec_string(
    input: Vec<bool>,
    output_true: &str,
    output_false: &str,
) -> Vec<String> {
    input
        .into_iter()
        .map(|x| match x {
            true => output_true.into(),
            false => output_false.into(),
        })
        .collect::<Vec<String>>()
}

// *  Convert sql type to String
pub fn convert_typ_struct_to_sql3_type(input: Vec<String>) -> Vec<String> {
    input
        .into_iter()
        .map(|i| match i.as_ref() {
            "bool" => String::from("BOOLEAN"),
            "u8" | "Option<u8>" | "u16" | "Option<u16>" | "u32"
            | "Option<u32>" | "u64" | "Option<u64>" | "i8" | "Option<i8>"
            | "i16" | "Option<i16>" | "i32" | "Option<i32>" | "i64"
            | "Option<i64>" => String::from("INTEGER"),
            "f32" | "Option<f32>" | "f64" | "Option<f64>" => {
                String::from("DOUBLE")
            }
            "String" | "Option<String>" => String::from("TEXT"),
            "DateTime<Utc>" | "Option<DateTime<Utc>>" => {
                String::from("DATETIME")
            }
            _ => todo!("{} - Type not supported", i),
        })
        .collect::<Vec<String>>()
}

// * Return all attributes that are ident_name and has rusqdrive
pub fn filter_path(
    ident_name: &str,
    fields: Option<&Punctuated<Field, Token![,]>>,
) -> Vec<bool> {
    // let empty: &Punctuated<Meta, Comma> = Default::default().clone();
    let mut data_vec = vec![];
    let fs = fields.expect(&format!(
        "Line: {}, Column: {}, File: {}",
        line!(),
        column!(),
        file!(),
    ));
    let fs_except_id = fs.iter().skip(1).count();

    // * Counter to check if all fields all fields(except `id`) of struct has #([rusqdrive(...))
    let mut rusqdrive_path_counter = 0;

    for field_named in &fs.clone() {
        for attr in &field_named.attrs {
            // * Counter to check if all fields all fields(except `id`) of struct has #([rusqdrive(...))
            rusqdrive_path_counter += 1;

            let path = attr.clone().path;

            // * Check id has `#[rusqdrive(...)]`
            // TODO: Move for another place: this validation occurs number of attributes in #[rusqdrive(...)]
            if field_named
                .ident
                .as_ref()
                .expect(&format!(
                    "Line: {}, Column: {}, File: {}",
                    line!(),
                    column!(),
                    file!(),
                ))
                .eq(&"id")
            {
                if path.is_ident("rusqdrive") {
                    panic!(" #[rusqdrive(...)] is not necessary for `id`")
                }
            }

            if path.is_ident("rusqdrive") {
                let metas = attr
                    .parse_args_with(
                        Punctuated::<Meta, Token![,]>::parse_terminated,
                    )
                    .expect(&format!(
                        "Line: {}, Column: {}, File: {}",
                        line!(),
                        column!(),
                        file!(),
                    ))
                    .into_iter()
                    .filter(|meta| meta.path().is_ident(ident_name));

                for meta in metas {
                    match meta {
                        Meta::NameValue(mnv) => {
                            let lit = mnv.lit;
                            match lit {
                                syn::Lit::Bool(lit_bool) => {
                                    let bool_value = lit_bool.value;
                                    match bool_value {
                                        true => data_vec.push(true),
                                        false => data_vec.push(false),
                                    }
                                }
                                _ => panic!("Values allowed: [true | false]"),
                            }
                        }
                        _ => {
                            panic!(
                                r#"Only works with a name-value pair within an attribute, like `not_null = true`"#
                            )
                        }
                    }
                }
            } else {
                panic!(r#"Only works with  #[rusqdrive(...)] "#)
            }
        }
    }

    // * Check if all fields all fields(except `id`) of struct has #([rusqdrive(...))
    if rusqdrive_path_counter < fs_except_id {
        panic!("All rusqdrives attributes(except `id`) must be at least #[rusqdrive(not_null = false, unique = false)]");
    }

    data_vec
}

// * Obtain the value of `#[tablename = "users"]`
pub fn obtain_table_name(attrs: &Vec<Attribute>) -> String {
    let struct_attrs = attrs.iter().take(1).collect::<Vec<_>>();
    match struct_attrs.first() {
        Some(attr) => {
            let attr_tablename_meta = attr.parse_meta().expect(&format!(
                "Line: {}, Column: {}, File: {}",
                line!(),
                column!(),
                file!(),
            ));
            match attr_tablename_meta {
                Meta::NameValue(mnv) => match mnv.lit {
                    syn::Lit::Str(lit_str) => lit_str.value(),
                    _ => {
                        panic!(
                            r#"#[tablename = "tablename"] - Accept only String "#
                        )
                    }
                },
                _ => panic!(
                    r#"Try: #[tablename = "tablename"] above o below your derive `derive[(Debug)]`"#
                ),
            }
        }
        None => {
            panic!(
                r#"Add: #[tablename = "tablename"] above o below your derive `derive[(Debug)]`"#
            )
        }
    }
}

// * Obtain keys and values of a struct
pub fn obtain_fields_struct(
    fields: Option<&Punctuated<Field, Token![,]>>,
) -> Vec<FieldStruct> {
    // let empty = Default::default();
    fields
        .expect(&format!(
            "Line: {}, Column: {}, File: {}",
            line!(),
            column!(),
            file!(),
        ))
        .iter()
        .enumerate()
        .map(|(_, f)| FieldStruct::new(f.clone()))
        .collect()
}
