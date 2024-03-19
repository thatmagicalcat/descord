use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[derive(Debug, FromMeta)]
struct CommandArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    if function.sig.asyncness.is_none() {
        panic!("Function marked with `#[descord::command(...)]` should be async");
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let command_args: CommandArgs = match CommandArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let custom_prefix = command_args.prefix.is_some();
    let new_name = format!(
        "{}{}",
        command_args.prefix.as_ref().unwrap_or(&String::new()),
        command_args
            .name
            .unwrap_or_else(|| function.sig.ident.to_string())
    );

    let function_name = &function.sig.ident;
    let function_body = &function.block;
    let function_params = &function.sig.inputs;

    let error = || panic!("Expected `descord::prelude::MessageData` as the first argument");
    match function_params.first() {
        Some(param) => {
            let param = match param {
                syn::FnArg::Typed(x) => x,
                _ => panic!("`self` is not allowed"),
            };

            match *param.ty {
                syn::Type::Path(ref path) if path.path.is_ident("MessageData") => {}
                _ => error(),
            }
        }

        _ => error(),
    }

    let mut params = vec![];
    let mut param_types = vec![];

    for param in function_params.iter().skip(1) {
        let param = match param {
            syn::FnArg::Typed(x) => x,
            _ => panic!("`self` is not allowed"),
        };

        let syn::Pat::Ident(name) = &*param.pat else {
            panic!();
        };

        let type_ = (*param.ty).clone();

        param_types.push(match type_ {
            syn::Type::Path(ref path) if path.path.is_ident("String") => quote! { Type::String },
            syn::Type::Path(ref path) if path.path.is_ident("isize") => quote! { Type::Int },
            syn::Type::Path(ref path) if path.path.is_ident("bool") => quote! { Type::Bool },
            _ => panic!("Unknown parameter type"),
        });

        params.push(CommandParam {
            name: name.ident.clone(),
            type_,
        });
    }

    let mut stmts: Vec<proc_macro2::TokenStream> = vec![];
    for (idx, param) in params.iter().enumerate() {
        let CommandParam { name, type_ } = &param;

        let name = match type_ {
            syn::Type::Path(ref path) if path.path.is_ident("String") => {
                quote! { Value::String(ref #name) }
            }
            syn::Type::Path(ref path) if path.path.is_ident("isize") => {
                quote! { Value::Int(ref #name) }
            }
            syn::Type::Path(ref path) if path.path.is_ident("bool") => {
                quote! { Value::Bool(ref #name) }
            }

            _ => panic!(),
        };

        stmts.push(quote! {
            let #name = args[#idx];
        });
    }

    let mut let_stmts = proc_macro2::TokenStream::new();
    let_stmts.extend(stmts.into_iter());

    let expanded = quote! {
        fn #function_name() -> descord::Command {
            use std::any::Any;

            fn f(
                data: descord::prelude::MessageData,
                args: Vec<Value>
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmts
                    drop(args);

                    #function_body
                })
            }

            descord::Command {
                name: String::from(#new_name),
                args: vec![#(#param_types),*],
                handler_fn: f,
                custom_prefix: #custom_prefix,
            }
        }
    };

    TokenStream::from(expanded)
}

struct CommandParam {
    name: syn::Ident,
    type_: syn::Type,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Type {
    String,
    Int,
    Bool,
}

#[proc_macro]
pub fn register_all_commands(input: TokenStream) -> TokenStream {
    let file_path = if !input.is_empty() {
        parse_macro_input!(input as syn::LitStr).value()
    } else {
        String::from("src/main.rs")
    };

    let mut commands = vec![];

    let items = syn::parse_file(&std::fs::read_to_string(&file_path).unwrap())
        .unwrap()
        .items;

    for item in items {
        if let syn::Item::Fn(function) = item {
            if function.attrs.iter().any(|attr| attr.path().segments.last().map_or(false, |seg| seg.ident == "command")) {
                commands.push(function.sig.ident);
            }
        }
    }

    let expanded = quote! {
        client.register_commands([#(#commands()),*]);
    };

    TokenStream::from(expanded)
}