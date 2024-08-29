use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Extractor)]
pub fn extractor_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the identifier of the struct being derived
    let trait_name = Ident::new(&format!("{}Extractor", &input.ident), Span::call_site());
    let struct_name = input.ident;

    // Get the list of fields of the struct
    let fields = if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named
                .named
                .iter()
                .map(|field| (field.ident.clone().unwrap(), field.ty.clone()))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let field_type = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();
    let field_name = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let field_name_ref = fields
        .iter()
        .map(|(name, _)| Ident::new(&format!("{}_ref", name), Span::call_site()))
        .collect::<Vec<_>>();
    let field_name_mut = fields
        .iter()
        .map(|(name, _)| Ident::new(&format!("{}_mut", name), Span::call_site()))
        .collect::<Vec<_>>();

    // Generate the trait definition
    let trait_def = quote! {
        pub trait #trait_name {
            #(fn #field_name(self) -> #field_type;)*
            #(fn #field_name_ref(&self) -> &#field_type;)*
            #(fn #field_name_mut(&mut self) -> &mut #field_type;)*
        }
    };

    // Generate the implementation of the trait for the struct
    let impl_block = quote! {
        impl #trait_name for #struct_name {
            #(fn #field_name(self) -> #field_type {
                self.#field_name
            })*
            #(fn #field_name_ref(&self) -> &#field_type {
                &self.#field_name
            })*
            #(fn #field_name_mut(&mut self) -> &mut #field_type {
                &mut self.#field_name
            })*
        }
    };

    // Combine the trait definition and the implementation into the final output
    let output = quote! {
        #trait_def
        #impl_block
    };

    // Return the generated code as a TokenStream
    output.into()
}
