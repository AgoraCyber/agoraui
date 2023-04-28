use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, ItemStruct, LitStr};

pub struct Args {
    type_name: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Args {
            type_name: input.parse()?,
        })
    }
}

pub struct Component {
    system_type: LitStr,
    item: ItemStruct,
}

impl Component {
    pub fn new(args: Args, item: ItemStruct) -> Self {
        Component {
            system_type: args.type_name,
            item,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let attrs = &self.item.attrs;
        let struct_name = &self.item.ident;
        let vis = &self.item.vis;
        let fields = match &self.item.fields {
            syn::Fields::Named(fields) => fields,
            _ => {
                panic!("Only support named fields");
            }
        };

        let field_names = fields
            .named
            .iter()
            .map(|field| &field.ident)
            .collect::<Vec<_>>();

        let fields = &fields.named;

        let (impl_generics, ty_generics, where_clause) = self.item.generics.split_for_impl();

        let system_type = &self.system_type;

        let codes = quote! {
            #(#attrs)*
            #vis struct #struct_name #ty_generics #where_clause {
                comp_uuid: libecs::Uuid,
                #fields
            }

            impl #ty_generics #struct_name #impl_generics #where_clause {
                #vis fn new(#fields) -> Self {
                    Self {
                        comp_uuid: libecs::Uuid::new_v4(),
                        #(#field_names,)*
                    }
                }

                #vis fn new_with_id(id: libecs::Uuid, #fields) -> Self {
                    Self {
                        comp_uuid: id,
                        #(#field_names,)*
                    }
                }
            }

            impl #ty_generics libecs::component::Component for #struct_name #impl_generics #where_clause {
                fn id(&self) -> &libecs::Uuid {
                    &self.comp_uuid
                }

                fn system(&self) -> &libecs::Uuid {
                   static UUID: libecs::once_cell::sync::Lazy<Uuid> = once_cell::sync::Lazy::new(|| {
                        use libecs::sha3::Digest;

                        let mut hasher = libecs::sha3::Keccak256::new();

                        hasher.update(#system_type.as_bytes());

                        let buff: [u8; 32] = hasher.finalize().into();

                        libecs::Uuid::from_bytes(buff[0..16].try_into().unwrap())
                    });

                    &UUID
                }

                fn as_any(&self) -> &dyn std::any::Any {
                     self as &dyn std::any::Any
                }

                fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                     self as &mut dyn std::any::Any
                }
            }
        };

        codes.into()
    }
}
