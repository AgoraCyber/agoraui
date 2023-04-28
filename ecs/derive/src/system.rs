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

pub struct System {
    system_type: LitStr,
    item: ItemStruct,
}

impl System {
    pub fn new(args: Args, item: ItemStruct) -> Self {
        System {
            system_type: args.type_name,
            item,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let struct_name = &self.item.ident;

        let item = &self.item;

        let (impl_generics, ty_generics, where_clause) = self.item.generics.split_for_impl();

        let system_type = &self.system_type;

        let codes = quote! {

            #item

            impl #ty_generics libecs::system::SystemId for #struct_name #impl_generics #where_clause {
                fn id() -> &'static libecs::Uuid {
                    static UUID: libecs::once_cell::sync::Lazy<Uuid> = once_cell::sync::Lazy::new(|| {
                        use libecs::sha3::Digest;

                        let mut hasher = libecs::sha3::Keccak256::new();

                        hasher.update(#system_type.as_bytes());

                        let buff: [u8; 32] = hasher.finalize().into();

                        libecs::Uuid::from_bytes(buff[0..16].try_into().unwrap())
                    });

                    &UUID
                }
            }
        };

        codes.into()
    }
}
