extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

#[proc_macro_derive(Composite)]
pub fn derive_composite(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::CompositeView for #name #ty_generics #where_clause {
            fn framework_build(&self) -> agoraui_compose::View {
                self.build().into_view()
            }
        }

        impl #impl_generics agoraui_compose::ToElement for #name #ty_generics #where_clause {
            fn to_element(&self, _view: agoraui_compose::View) -> agoraui_compose::Element {
                agoraui_compose::Element::Empty
            }
        }

        impl #impl_generics agoraui_compose::IntoView for #name #ty_generics #where_clause {
            fn into_view(self) ->  agoraui_compose::View {
                agoraui_compose::View::from_composite(self)
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

#[proc_macro_derive(CompositeWithState)]
pub fn derive_composite_with_state(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::CompositeWithStateView for #name #ty_generics #where_clause {
            fn framework_create_state(&self) -> Box<dyn agoraui_compose::State> {
                Box::new(self.create_state())
            }
        }

        impl #impl_generics agoraui_compose::ToElement for #name #ty_generics #where_clause {
            fn to_element(&self, _view: agoraui_compose::View) -> agoraui_compose::Element {
                agoraui_compose::Element::Empty
            }
        }

        impl #impl_generics agoraui_compose::IntoView for #name #ty_generics #where_clause {
            fn into_view(self) ->  agoraui_compose::View {
                agoraui_compose::View::from_composite_with_state(self)
            }
        }
    }
    .into()
}
