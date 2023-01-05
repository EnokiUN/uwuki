use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Attribute, Error, ItemFn, Lit, MetaNameValue, ReturnType};

// In my defence this is not supposed to be used outside of uwuki so it is not my fault if you use
// it and it sucks, m'kay? ;-;
#[proc_macro_attribute]
pub fn command(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as ItemFn);
    let span = item.span();
    let name = item.sig.ident;
    let mut name_str = name.to_string();
    let args = item.sig.inputs;
    let block = item.block;
    let ret = match item.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, t) => {
            quote!(#t)
        }
    };

    let mut description = None;
    let mut usage = None;

    for attr in item.attrs.iter().filter(|a| a.path.is_ident("uwuki")) {
        let args: MetaNameValue = match attr.parse_args() {
            Ok(arg) => arg,
            Err(err) => return err.into_compile_error().into(),
        };
        match args.path.get_ident() {
            Some(ident) => {
                let attr_name = ident.to_string();
                match attr_name.as_str() {
                    "name" => {
                        if let Lit::Str(lit) = args.lit {
                            name_str = lit.value();
                        } else {
                            return Error::new(
                                args.lit.span(),
                                "The supplied argument must be a string literal",
                            )
                            .into_compile_error()
                            .into();
                        }
                    }
                    "description" => {
                        if let Lit::Str(lit) = args.lit {
                            description = Some(lit.value());
                        } else {
                            return Error::new(
                                args.lit.span(),
                                "The supplied argument must be a string literal",
                            )
                            .into_compile_error()
                            .into();
                        }
                    }
                    "usage" => {
                        if let Lit::Str(lit) = args.lit {
                            usage = Some(lit.value());
                        } else {
                            return Error::new(
                                args.lit.span(),
                                "The supplied argument must be a string literal",
                            )
                            .into_compile_error()
                            .into();
                        }
                    }
                    _ => {
                        return Error::new(args.span(), "Unknown attribute name")
                            .into_compile_error()
                            .into()
                    }
                }
            }
            None => {
                return Error::new(args.span(), "Attribute must have a path")
                    .to_compile_error()
                    .into()
            }
        }
    }

    let attrs: Vec<Attribute> = item
        .attrs
        .into_iter()
        .filter(|a| !a.path.is_ident("uwuki"))
        .collect();

    let command_path = quote!(crate::commands::Command);

    let description = match description {
        Some(description) => description,
        None => {
            return Error::new(span, "You must supply a command description")
                .into_compile_error()
                .into()
        }
    };
    let usage = match usage {
        Some(usage) => usage,
        None => {
            return Error::new(span, "You must supply a command usage")
                .into_compile_error()
                .into()
        }
    };

    let struct_name = format_ident!("{}_COMMAND", name.to_string().to_uppercase());
    quote!(
        #(#attrs)*
        fn #name<'a>(#args) -> std::pin::Pin<Box<dyn std::future::Future<Output = #ret> + Send + 'a>>{
            Box::pin(async move { #block })
        }

        pub static #struct_name: #command_path = #command_path {
            name: #name_str,
            description: #description,
            usage: #usage,
            func: #name,
        };
    )
    .into()
}
