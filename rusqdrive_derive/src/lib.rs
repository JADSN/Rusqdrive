#![warn(clippy::all)]
#![deny(unsafe_code)]

mod create_table;
mod field_struct;
mod utils;

use create_table::CreateTable;
use utils::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DeriveInput,
    Field, Fields, Token,
};

fn mount_struct(
    ast: &DeriveInput,
    fields: Option<&Punctuated<Field, Token![,]>>,
    struct_attrs: &Vec<Attribute>,
    crud_name: String,
) -> TokenStream2 {
    let struct_name = &ast.ident;

    let tablename_derive_name = obtain_table_name(struct_attrs);

    let struct_fields = obtain_fields_struct(fields);

    let fields_len = fields
        .expect(&format!(
            "Line: {}, Column: {}, File: {}",
            line!(),
            column!(),
            file!(),
        ))
        .len();

    let read_all_names = struct_fields
        .iter()
        .enumerate()
        .map(|(idx, f)| f.as_row_read_all(idx));

    let only_names = struct_fields.iter().skip(1).map(|f| f.only_names());

    let params_fields = struct_fields.iter().skip(1).map(|f| f.as_params());

    // * OBTAIN ALL TYPES(values) OF A STRUCT
    let mut types = vec![];
    let fns = &fields
        .expect(&format!(
            "Line: {}, Column: {}, File: {}",
            line!(),
            column!(),
            file!(),
        ))
        .clone();

    // * Support Option
    for field_named in fns.iter().skip(1).clone() {
        let ty = field_named.ty.clone();
        match ty.clone() {
            syn::Type::Path(type_path) => {
                let tp_segments = type_path.clone().path.segments;

                for tpseg in tp_segments {
                    let option = tpseg.ident.to_string();
                    let args = tpseg.arguments;
                    match args {
                        syn::PathArguments::AngleBracketed(abgas) => {
                            let abgas_args = abgas.args;
                            for ga in abgas_args {
                                match ga {
                                    syn::GenericArgument::Type(typ) => {
                                        match typ {
                                            syn::Type::Path(type_path) => {
                                                // dbg!(&type_path);
                                                // * Recognize simple Chrono Option -> Option<...<T>>

                                                for path_seg in type_path
                                                    .clone()
                                                    .path
                                                    .segments
                                                {
                                                    // * If necessary make an array with another chrono types and add in utils.rs
                                                    let chrono_date_time =
                                                        String::from(
                                                            "DateTime",
                                                        );
                                                    if path_seg
                                                        .ident
                                                        .to_string()
                                                        == chrono_date_time
                                                    {
                                                        let path_seg_args =
                                                            path_seg.arguments;
                                                        match path_seg_args {
                                                            syn::PathArguments::AngleBracketed(abgca) => {
                                                                let ga = abgca.args.first().unwrap();
                                                                match ga {
                                                                    syn::GenericArgument::Type(typ) => {
                                                                        match typ {
                                                                            syn::Type::Path(type_path) => {
                                                                                // let utc = type_path.path.get_ident().unwrap().to_string();

                                                                                match type_path.path.get_ident()
                                                                                {
                                                                                    Some(_) => {
                                                                                        let option_type =
                                                                                            String::from("Option<DateTime<Utc>>");
                                                                                        types.push(option_type);
                                                                                    }
                                                                                    None => {
                                                                                        panic!(
                                                                                            "Type not allowed"
                                                                                        )
                                                                                    }
                                                                                }

                                                                            }
                                                                            _ => panic!(
                                                                                "Line: {}, Column: {}, File: {}",
                                                                                line!(),
                                                                                column!(),
                                                                                file!())
                                                                        }
                                                                    }
                                                                    _ => panic!(
                                                                        "Line: {}, Column: {}, File: {}",
                                                                        line!(),
                                                                        column!(),
                                                                        file!())
                                                                }

                                                            },
                                                            _ => panic!("Rusqdrive doesn't support this type")
                                                        }
                                                    } else {
                                                        // * Recognize simple Option -> Option<T>
                                                        match type_path
                                                            .path
                                                            .get_ident()
                                                        {
                                                            Some(typ) => {
                                                                // dbg!(&typ);
                                                                // if typ.to_string()
                                                                //     == String::from(
                                                                //         "Utc",
                                                                //     )
                                                                // {
                                                                //     let option_type = format!(
                                                                //         "Option<DateTime<Utc>>",
                                                                //         // typ.to_string()
                                                                //     );
                                                                //     dbg!(&option_type);

                                                                //     types.push(
                                                                //         option_type,
                                                                //     );
                                                                // } else {
                                                                let option_type =
                                                                    format!(
                                                            "{}<{}>",
                                                            option,
                                                            typ.to_string()
                                                        );
                                                                types.push(
                                                                    option_type,
                                                                );
                                                                // }
                                                            }
                                                            None => {
                                                                panic!(
                                                            "Only primitives types is supported and Option<type>"
                                                        )
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => panic!(
                                            "Line: {}, Column: {}, File: {}",
                                            line!(),
                                            column!(),
                                            file!()
                                        ),
                                        }
                                    }
                                    _ => panic!(
                                        "Line: {}, Column: {}, File: {}",
                                        line!(),
                                        column!(),
                                        file!()
                                    ),
                                }
                            }
                        }

                        syn::PathArguments::None => {
                            match type_path.path.get_ident() {
                                Some(typ) => {
                                    types.push(typ.to_string());
                                }
                                None => {
                                    panic!("Type not allowed")
                                }
                            }
                        }
                        _ => panic!(
                            "Line: {}, Column: {}, File: {}",
                            line!(),
                            column!(),
                            file!()
                        ),
                    }
                }
            }
            _ => panic!("Only for primitive types"),
        }
    }

    let mut names = vec![];

    match crud_name.as_ref() {
        "create_table" => {
            // * OBTAIN ALL NAMES(keys) STRUCT
            let fns = &fields
                .expect(&format!(
                    "Line: {}, Column: {}, File: {}",
                    line!(),
                    column!(),
                    file!(),
                ))
                .clone();
            for field_named in fns.iter().skip(1).clone() {
                let name = field_named
                    .ident
                    .clone()
                    .expect(&format!(
                        "Line: {}, Column: {}, File: {}",
                        line!(),
                        column!(),
                        file!(),
                    ))
                    .to_string();
                names.push(name);
            }

            // * OBTAIN ALL NOT_NULL'S
            let all_not_nulls = filter_path("not_null", fields);
            let all_not_nulls_to_str =
                vec_bool_to_vec_string(all_not_nulls, "NOT NULL", "");

            // * OBTAIN ALL UNIQUE'S
            let all_uniques = filter_path("unique", fields);
            let all_uniques_to_str =
                vec_bool_to_vec_string(all_uniques, "UNIQUE", "");

            let sql_types = convert_typ_struct_to_sql3_type(types);

            let create_table_vec: Vec<u8> = vec![0; fields_len];

            let create_tables = create_table_vec
                .iter()
                .zip(names.iter())
                .zip(sql_types.iter())
                .zip(all_not_nulls_to_str.iter())
                .zip(all_uniques_to_str.iter())
                .map(|((((_, c), t), n), u)| CreateTable {
                    col: c.to_owned(),
                    typ: t.to_owned(),
                    not_null: n.to_owned(),
                    unique: u.to_owned(),
                })
                .collect::<Vec<CreateTable>>();

            let mut create_table_query = format!("CREATE TABLE IF NOT EXISTS {} (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,",  quote!(#tablename_derive_name));

            for idx in 0..create_tables.len() {
                let ct_row = create_tables[idx].clone();
                if idx < create_tables.len() - 1 {
                    create_table_query.push_str(&format!(
                        "{} {} {} {},",
                        ct_row.col, ct_row.typ, ct_row.not_null, ct_row.unique
                    ));
                } else {
                    create_table_query.push_str(&format!(
                        "{} {} {} {}",
                        ct_row.col, ct_row.typ, ct_row.not_null, ct_row.unique
                    ));
                }
            }

            let mut normalize_query = create_table_query.trim_end().to_string();

            normalize_query.push_str(");");

            // dbg!(&normalize_query);

            let result_quote = quote! {
                pub fn create_table(&self, conn: &DbConnection) -> RusqliteResult<()> {
                    conn.execute(
                       #normalize_query,
                        [],
                    )?;

                    Ok(())
                }
            };

            result_quote.into()
        }
        "read_count" => {
            let result_quote = quote! {
                pub fn read_count(&self,conn: &DbConnection) -> RusqliteResult<u32> {
                    // *  READ COUNT
                    let query_count = format!("SELECT COUNT(ALL) FROM {}", #tablename_derive_name);

                    let count: RusqliteResult<u32> =
                        conn.query_row(&query_count, [], |r| r.get(0));

                    count
                }
            };

            result_quote.into()
        }
        "read_all" => {
            let read_all_rows = quote! {
                pub fn read_all(
                    &self,
                    conn: &DbConnection,
                ) -> RusqliteResult<Vec<#struct_name>> {
                    let query = format!("SELECT * FROM {}",  #tablename_derive_name);
                    let mut stmt = conn.prepare(&query)?;
                    let data_iter = stmt.query_map([], |row| {
                        Ok(#struct_name{
                            #(#read_all_names)*
                        })
                    })?;

                    let mut data_vec = vec![];

                    for data in data_iter {
                        data_vec.push(data?);
                    }

                    Ok(data_vec)
                }
            };

            read_all_rows.into()
        }
        "insert_one" => {
            let interrogations_fields = struct_fields.iter().skip(2).map(|f| f);

            let mut interrogations = vec![];
            let interrogations_fields_len = interrogations_fields.len();

            // *  GENERATE VALUES
            for i in 0..=interrogations_fields_len {
                if i <= interrogations_fields_len - 1 {
                    interrogations.push(format!("?{},", i + 1));
                } else {
                    interrogations.push(format!("?{}", i + 1));
                }
            }

            let values = interrogations.join("");

            // * GENEREATE NAMES
            let only_names_len = only_names.len();

            let mut names = vec![];
            for (idx, item) in only_names.clone().enumerate() {
                if idx < only_names_len - 1 {
                    names.push(format!("{},", item.to_string()));
                } else {
                    names.push(format!("{}", item.to_string()));
                }
            }

            let normalize_names = names.join("");

            let result_quote = quote! {
                pub fn insert_one(
                    &self,
                    conn: &DbConnection,
                    data: &#struct_name,
                ) -> RusqliteResult<String> {
                    match conn.execute(
                        &format!("INSERT INTO {} ({}) VALUES ({})",  #tablename_derive_name, #normalize_names,  #values),
                        params![#(#params_fields)*],
                    ) {
                        Ok(_) => Ok("CREATED".into()),
                        Err(error) => Err(error),
                    }
                }
            };

            result_quote.into()
        }
        "update_one" => {
            // let params_len = params_fields.len();
            let names_len = only_names.len();

            let mut names_and_values = vec![];

            for (idx, item) in only_names.enumerate() {
                if idx < names_len - 1 {
                    names_and_values.push(format!(
                        "{}=?{},",
                        item.to_string(),
                        idx + 1
                    ));
                } else {
                    names_and_values.push(format!(
                        "{}=?{}",
                        item.to_string(),
                        idx + 1
                    ));
                }
            }

            let names_and_values_result = names_and_values.join("");

            let result_quote = quote! {

                pub fn update_one(
                    &self,
                    conn: &DbConnection,
                    data: &#struct_name,
                    id: u32,
                ) -> RusqliteResult<String> {
                    let query = format!("UPDATE {} SET {} WHERE id=?{}",  #tablename_derive_name, #names_and_values_result, #names_len + 1);

                    match conn.execute(&query, params![#(#params_fields)*id])
                    {
                        Ok(_) => Ok("UPDATED".into()),
                        Err(error) => Err(error),
                    }
                }


            };

            result_quote.into()
        }
        "delete_one" => {
            let result_quote = quote! {
                pub fn delete_one(&self, conn: &DbConnection, id: u32) -> RusqliteResult<String> {
                    let query = format!("DELETE FROM {} WHERE id=?1", #tablename_derive_name);
                    match conn.execute(&query, params![id]) {
                        Ok(_) => Ok("DELETED".into()),
                        Err(error) => Err(error),
                    }
                }
            };

            result_quote.into()
        }
        _ => todo!(),
    }
}

fn obtain_data_from_struct(
    ast: &DeriveInput,
    fields: &Fields,
    struct_attrs: &Vec<Attribute>,
    crud_name: String,
) -> TokenStream2 {
    match fields {
        Fields::Named(ref fields) => {
            // * Check firs field is `id`
            let field_named = &fields.named;

            if field_named
                .first()
                .unwrap()
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                != String::from("id")
            {
                panic!("Is obligated first field be id. Try: `id: i32` or `id: Option<i32>`");
            } else {
                mount_struct(&ast, Some(field_named), struct_attrs, crud_name)
            }
        }
        _ => panic!(
            r#"#[derive(Rusqdrive)] is only defined for structs, not for enums!"#
        ),
    }
}

fn obtain_struct(
    ast: &DeriveInput,
    struct_attrs: &Vec<Attribute>,
    crud_name: String,
) -> TokenStream2 {
    let result = match ast.data {
        Data::Struct(ref ds) => {
            obtain_data_from_struct(&ast, &ds.fields, struct_attrs, crud_name)
        }
        _ => unimplemented!("Not implement for Enum and Union"),
    };

    TokenStream2::from(result)
}

// * RUSQDRIVE DERIVE
#[proc_macro_derive(Rusqdrive, attributes(rusqdrive, tablename))]
pub fn rusqdrive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_attrs = &ast.attrs;

    let create_table =
        obtain_struct(&ast, struct_attrs, String::from("create_table"));
    let read_count =
        obtain_struct(&ast, struct_attrs, String::from("read_count"));
    let read_all = obtain_struct(&ast, struct_attrs, String::from("read_all"));
    let insert_one =
        obtain_struct(&ast, struct_attrs, String::from("insert_one"));
    let update_one =
        obtain_struct(&ast, struct_attrs, String::from("update_one"));
    let delete_one =
        obtain_struct(&ast, struct_attrs, String::from("delete_one"));

    let result_quote = quote! {
        impl  #struct_name {

            #create_table

            #read_count

            #read_all

            #insert_one

            #update_one

            #delete_one
        }

    };

    result_quote.into()
}
