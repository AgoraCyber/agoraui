extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

#[proc_macro_derive(Stateless)]
pub fn derive_composite(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::StatelessConfigration for #name #ty_generics #where_clause {
            fn framework_build(&self) -> agoraui_compose::View {
                self.build().into_view()
            }
        }

        impl #impl_generics agoraui_compose::ToElement for #name #ty_generics #where_clause {
            fn to_element(&self, view: agoraui_compose::View) -> agoraui_compose::Element {
                agoraui_compose::StatelessElement::from(view).into()
            }
        }

        impl #impl_generics agoraui_compose::IntoView for #name #ty_generics #where_clause {
            #[track_caller]
            fn into_view(self) ->  agoraui_compose::View {
                let caller = std::panic::Location::caller();
                agoraui_compose::stateless_to_view(caller,self)
            }
        }

        impl #impl_generics agoraui_compose::ToAny for #name #ty_generics #where_clause {
            fn to_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl #impl_generics agoraui_compose::AnyEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &dyn std::any::Any) -> bool {
                self == other.downcast_ref::<#name #ty_generics>().unwrap()
            }
        }

    }
    .into()
}

#[proc_macro_derive(State)]
pub fn derive_state(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::State for #name #ty_generics #where_clause {
            fn framework_build(&self) -> agoraui_compose::View {
                self.build().into_view()
            }
        }
    }
    .into()
}

#[proc_macro_derive(Stateful)]
pub fn derive_composite_with_state(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::StatefulConfigration for #name #ty_generics #where_clause {
            fn framework_create_state(&self) -> Box<dyn agoraui_compose::State> {
                Box::new(self.create_state())
            }
        }

        impl #impl_generics agoraui_compose::ToElement for #name #ty_generics #where_clause {
            fn to_element(&self, view: agoraui_compose::View) -> agoraui_compose::Element {
                agoraui_compose::StatefulElement::from(view).into()
            }
        }

        impl #impl_generics agoraui_compose::IntoView for #name #ty_generics #where_clause {
            #[track_caller]
            fn into_view(self) ->  agoraui_compose::View {
                let caller = std::panic::Location::caller();
                agoraui_compose::stateful_to_view(caller,self)
            }
        }

        impl #impl_generics agoraui_compose::ToAny for #name #ty_generics #where_clause {
            fn to_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl #impl_generics agoraui_compose::AnyEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &dyn std::any::Any) -> bool {
                self == other.downcast_ref::<#name #ty_generics>().unwrap()
            }
        }
    }
    .into()
}
