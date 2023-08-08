extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::ItemStruct;

#[proc_macro_derive(Stateless)]
pub fn derive_stateless(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::StatelessView for #name #ty_generics #where_clause {
            fn build(&self,context: &mut dyn agoraui_compose::BuildContext) -> agoraui_compose::AnyView {
                self.build(context).into_any_view()
            }
        }

        impl #impl_generics agoraui_compose::View for #name #ty_generics #where_clause {
            fn into_any_view(self) -> agoraui_compose::AnyView {
                agoraui_compose::AnyView::Stateless(Box::new(self))
             }
        }
    }
    .into()
}

#[proc_macro_derive(Stateful)]
pub fn derive_stateful(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::StatefulView for #name #ty_generics #where_clause {
             fn create_view_state(&self) -> Box<dyn agoraui_compose::ViewState> {
                Box::new(self.create_view_state())
            }
        }

        impl #impl_generics agoraui_compose::View for #name #ty_generics #where_clause {
            fn into_any_view(self) -> agoraui_compose::AnyView {
                agoraui_compose::AnyView::Stateful(Box::new(self))
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

       impl #impl_generics agoraui_compose::ViewState for #name #ty_generics #where_clause {
            fn build(&mut self,context: &mut dyn agoraui_compose::BuildContext) -> agoraui_compose::AnyView {
                self.build(context).into_any_view()
            }
        }

    }
    .into()
}
