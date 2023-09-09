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

        impl #impl_generics agoraui_compose::view::StatelessConfiguration for #name #ty_generics #where_clause {
            fn framework_build(&self, element: &agoraui_compose::element::StatelessElement) -> agoraui_compose::view::View {
                self.build(element).into_view()
            }
        }

        impl #impl_generics agoraui_compose::view::IntoView for #name #ty_generics #where_clause {
            #[track_caller]
            fn into_view(self) ->  agoraui_compose::view::View {
                let caller: agoraui_compose::keypath::KeyPath = std::panic::Location::caller().into();
                agoraui_compose::view::View::Stateless((caller,self).into())
            }
        }

        impl #impl_generics agoraui_compose::view::ToAny for #name #ty_generics #where_clause {
            fn to_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl #impl_generics agoraui_compose::view::AnyEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &dyn std::any::Any) -> bool {
                self == other.downcast_ref::<#name #ty_generics>().unwrap()
            }
        }

    }
    .into()
}

#[proc_macro_derive(Render)]
pub fn derive_render_object(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let name = &item_struct.ident;

    quote! {

        impl #impl_generics agoraui_compose::view::RenderObjectConfiguration for #name #ty_generics #where_clause {
            fn framework_create_render_object(&self) -> agoraui_compose::render::RenderObject {
                self.create_render_object().into()
            }

            fn framework_render_object_children(&self) -> Vec<View> {
                self.render_object_children()
            }
        }

        impl #impl_generics agoraui_compose::view::IntoView for #name #ty_generics #where_clause {
            #[track_caller]
            fn into_view(self) ->  agoraui_compose::view::View {
                let caller: agoraui_compose::keypath::KeyPath = std::panic::Location::caller().into();
                agoraui_compose::view::View::RenderObject((caller,self).into())
            }
        }

        impl #impl_generics agoraui_compose::view::ToAny for #name #ty_generics #where_clause {
            fn to_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl #impl_generics agoraui_compose::view::AnyEq for #name #ty_generics #where_clause {
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

        impl #impl_generics agoraui_compose::view::State for #name #ty_generics #where_clause {
            fn framework_build(&self, element: element: &agoraui_compose::element::StatefulElement) -> agoraui_compose::View {
                self.build(element).into_view()
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

        impl #impl_generics agoraui_compose::view::StatefulConfiguration for #name #ty_generics #where_clause {
            fn framework_create_state(&self) -> Box<dyn agoraui_compose::view::State> {
                Box::new(self.create_state())
            }
        }

        impl #impl_generics agoraui_compose::view::IntoView for #name #ty_generics #where_clause {
            #[track_caller]
            fn into_view(self) ->  agoraui_compose::view::View {
                let caller: agoraui_compose::keypath::KeyPath = std::panic::Location::caller().into();
                 agoraui_compose::view::View::Stateful((caller,self).into())
            }
        }

        impl #impl_generics agoraui_compose::view::ToAny for #name #ty_generics #where_clause {
            fn to_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl #impl_generics agoraui_compose::view::AnyEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &dyn std::any::Any) -> bool {
                self == other.downcast_ref::<#name #ty_generics>().unwrap()
            }
        }
    }
    .into()
}
