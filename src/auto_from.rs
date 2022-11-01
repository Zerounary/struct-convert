use darling::{FromDeriveInput, FromField, FromAttributes};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type,
    TypePath, Attribute,
};

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(convert))]
struct MetaOpts {
    #[darling(multiple)]
    into: Vec<String>,
    #[darling(multiple)]
    from: Vec<String>,
}

#[derive(Debug, Default, Clone, FromField, FromAttributes)]
#[darling(default, attributes(convert_field))]
struct FiledOpts {
    class: String,
    rename: String,
    custom_fn: String,
    ignore: bool,
    wrap: bool,
    unwrap: bool,
    option: bool,
    to_string: bool,
}

#[derive(Clone, Debug)]
struct Fd {
    name: Ident,
    opts: FiledOpts,
    multi_opts: Vec<FiledOpts>,
    multiple: bool,
    optional: bool,
    is_vec: bool,
}
/// 把一个 Field 转换成 Fd
impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, is_vec ,_) = get_option_inner(&f.ty);
        let opts = FiledOpts::from_field(&f).unwrap_or_default();
        let multi_opts = parse_attrs(&f.attrs);
        Self {
            // 此时，我们拿到的是 NamedFields，所以 ident 必然存在
            name: f.ident.unwrap(),
            optional,
            is_vec,
            opts,
            multi_opts,
            multiple: f.attrs.len() > 1
        }
    }
}

impl Fd {
    fn get_by_name(&self, name: String) -> FiledOpts {
        for opt in self.multi_opts.iter().filter(|o| o.class.eq(&name)) {
            return opt.clone();
        }
        panic!("not found opts {:}", name)
    }
}

fn parse_attrs(attrs: &[Attribute]) -> Vec<FiledOpts> {
    let mut result = vec![];
    for attr in attrs.iter().filter(|attr| attr.path.is_ident("convert_field")) {
        match FiledOpts::from_attributes(&[attr.clone().into()]) {
            Ok(f) => {
                result.push(f);
            },
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    result
}

#[derive(Debug)]
pub struct DeriveIntoContext {
    name: Ident,
    attrs: MetaOpts,
    fields: Vec<Fd>,
}

impl DeriveIntoContext {
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        let is_from = !self.attrs.from.is_empty();

        if is_from {
            TokenStream::from_iter(self.attrs.from.iter().map(|from| {
                let struct_name = Ident::new(&format!("{}", name), name.span());
                let source_name = Ident::new(&format!("{}", from), name.span());

                let assigns = self.gen_from_assigns(from.clone().to_string());

                quote! {
                        impl std::convert::From<#source_name> for #struct_name {
                            fn from(s: #source_name) -> Self {
                                #struct_name {
                                #(#assigns)*
                                ..#struct_name::default()
                            }
                        }
                    }
                }
            }))
        } else {
            TokenStream::from_iter(self.attrs.into.iter().map(|into| {
                let struct_name = Ident::new(&format!("{}", name), name.span());
                let target_name = Ident::new(&format!("{}", into), name.span());
                let assigns = self.gen_into_assigns(into.clone().to_string());

                quote! {
                    impl std::convert::From<#struct_name> for #target_name {
                        fn from(s: #struct_name) -> Self {
                            #target_name {
                                #(#assigns)*
                                ..#target_name::default()
                            }
                        }
                    }
                }
            }))
        }
    }

    fn gen_from_assigns(&self, sturct_name: String) -> Vec<TokenStream> {
        self.fields
            .clone()
            .into_iter()
            .map(
                |fd| {
                    let Fd {
                        name,
                        optional,
                        is_vec,
                        mut opts,
                        multiple,
                        ..
                    } = fd.clone();
                    if multiple {
                        opts = fd.get_by_name(sturct_name.clone());
                    }
                    let source_name: Ident = if opts.rename.is_empty() {
                        name.clone()
                    } else {
                        Ident::new(opts.rename.as_str(), name.span())
                    };

                    if !opts.custom_fn.is_empty() {
                        let custom = Ident::new(&opts.custom_fn.as_str(), name.span());
                        return quote! {
                            #name: #custom(&s),
                        };
                    }

                    if opts.ignore {
                        return quote!();
                    }

                    if opts.unwrap {
                        return quote! {
                            #name: s.#source_name.unwrap_or_default(),
                        };
                    }

                    if optional && opts.wrap {
                        return quote! {
                            #name: Some(s.#source_name),
                        };
                    }

                    if opts.to_string {
                        return quote! {
                            #name: s.#source_name.to_string(),
                        };
                    }

                    if is_vec {
                        return quote! {
                            #name: s.#source_name.into_iter().map(|a| a.into()).collect(),
                        };
                    }

                    quote! {
                        #name: s.#source_name.into(),
                    }
                },
            )
            .collect()
    }

    // 比如：#field_name: self.#field_name.take().ok_or(" xxx need to be set!")
    fn gen_into_assigns(&self, struct_name: String) -> Vec<TokenStream> {
        self.fields
            .clone()
            .into_iter()
            .map(
                |fd| {
                    let Fd {
                        name,
                        optional,
                        is_vec,
                        mut opts,
                        multiple,
                        ..
                    } = fd.clone();

                    if multiple {
                        opts = fd.get_by_name(struct_name.clone());
                    }
                    let target_name: Ident = if opts.rename.is_empty() {
                        name.clone()
                    } else {
                        Ident::new(opts.rename.as_str(), name.span())
                    };

                    if !opts.custom_fn.is_empty() {
                        let custom = Ident::new(&opts.custom_fn.as_str(), name.span());
                        return quote! {
                            #target_name: #custom(&s),
                        };
                    }

                    if opts.ignore {
                        return quote!();
                    }

                    if optional && opts.unwrap {
                        return quote! {
                            #target_name: s.#name.unwrap_or_default(),
                        };
                    }

                    if opts.option {
                        if optional {
                            return quote! {
                                #target_name: s.#name,
                            };
                        } else {
                            return quote! {
                                #target_name: Some(s.#name),
                            };
                        }
                    }

                    if opts.to_string {
                        return quote! {
                            #target_name: s.#name.to_string(),
                        };
                    }

                    if is_vec {
                        return quote! {
                            #target_name: s.#name.into_iter().map(|a| a.into()).collect(),
                        };
                    }

                    quote! {
                        #target_name: s.#name.into(),
                    }
                },
            )
            .collect()
    }
}

impl From<DeriveInput> for DeriveIntoContext {
    fn from(input: DeriveInput) -> Self {
        let attrs = match MetaOpts::from_derive_input(&input) {
            Ok(v) => v,
            Err(_e) => {
                panic!("not args");
            }
        };
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();
        Self {
            name,
            fields: fds,
            attrs,
        }
    }
}

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, bool, &Type) {
    // 首先模式匹配出 segments
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                // 如果 PathSegment 第一个是 Option，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (true, false, t);
            }
            if v.ident == "Vec" {
                // 如果 PathSegment 第一个是 Option，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (false, true, t);
            }
        }
    }
    (false, false, ty)
}
