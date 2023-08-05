extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{Fields, ItemStruct};

#[proc_macro_derive(Composable, attributes(state, observed))]
pub fn derive_view_builder(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    let mut state_fields = vec![];

    let mut observed_fields = vec![];

    if let Fields::Named(fields) = &item_struct.fields {
        for field in &fields.named {
            for attr in &field.attrs {
                if attr.path.is_ident("state") {
                    state_fields.push(field);
                }

                if attr.path.is_ident("observed") {
                    observed_fields.push(field);
                }
            }
        }
    }

    let state_funcs = state_fields
        .into_iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let set_name = format_ident!("set_{}", ident);

            let value_type = &field.ty;

            quote! {
                fn #set_name(&mut self, mut context: impl agoraui_compose::BuildContext, value: #value_type) {
                    self.#ident = value;
                    context.set_state();
                }
            }
            
        })
        .collect::<Vec<_>>();

    quote! {

        impl #impl_generics agoraui_compose::ComposableView for #name #ty_generics #where_clause {
            fn build(&mut self,context: &mut Box<dyn agoraui_compose::BuildContext>) -> agoraui_compose::AnyView {
                self.build(context).into_any_view()
            }
        }

        impl #impl_generics agoraui_compose::View for #name #ty_generics #where_clause {
            fn into_any_view(self) -> agoraui_compose::AnyView {
                agoraui_compose::AnyView::Composable(Box::new(self))
             }
        }

        impl #impl_generics #name #ty_generics #where_clause{
            #(#state_funcs)*
        }
    }
    .into()
}


#[proc_macro_attribute]
pub fn observable_object(_: TokenStream, item: TokenStream) -> TokenStream {

    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (_, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    let attrs = &item_struct.attrs;

    quote! {
        #(#attrs)*
        struct #name #ty_generics #where_clause {
           
        }
    }.into()
}