use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericParam,
    Generics,
};

extern crate proc_macro;

#[proc_macro_derive(MaxSize)]
pub fn derive_max_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Add a bound `T: MaxSize` to every type parameter T.
    add_trait_bounds(&mut input.generics);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate an expression to aggregate the max size of each field.
    let sum = overall_max_size(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics borsh_max_size::MaxSize for #name #ty_generics #where_clause {
            fn max_size() -> usize {
                #sum
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: MaxSize` to every type parameter T.
fn add_trait_bounds(generics: &mut Generics) {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(borsh_max_size::MaxSize));
        }
    }
}

fn overall_max_size(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => size_of_fields(&data.fields),
        Data::Enum(ref data) => {
            let recurse = data
                .variants
                .iter()
                .map(|variant| size_of_fields(&variant.fields));
            recurse.fold(
                quote!(0),
                |max_, expr| quote! { std::cmp::max(#max_, #expr)},
            )
        }
        Data::Union(ref data) => {
            let recurse = data.fields.named.iter().map(|f| {
                let type_ = &f.ty;
                quote_spanned! {f.span()=>
                    <#type_ as borsh_max_size::MaxSize>::max_size()
                }
            });
            recurse.fold(
                quote!(0),
                |max_, expr| quote! { std::cmp::max(#max_, #expr)},
            )
        }
    }
}

fn size_of_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => {
            // Expands to an expression like
            //
            //     0 + self.x.max_size() + self.y.max_size() + self.z.max_size()
            //
            // but using fully qualified function call syntax.
            let recurse = fields.named.iter().map(|f| {
                let type_ = &f.ty;
                quote_spanned! {f.span()=>
                    <#type_ as borsh_max_size::MaxSize>::max_size()
                }
            });
            quote! {
                0 #(+ #recurse)*
            }
        }
        Fields::Unnamed(ref fields) => {
            // Expands to an expression like
            //
            //     0 + self.0.max_size() + self.1.max_size() + self.2.max_size()
            let recurse = fields.unnamed.iter().map(|f| {
                let type_ = &f.ty;
                quote_spanned! {f.span()=>
                    <#type_ as borsh_max_size::MaxSize>::max_size()
                }
            });
            quote! {
                0 #(+ #recurse)*
            }
        }
        Fields::Unit => {
            // Unit structs don't take up space in the serialization.
            quote!(0)
        }
    }
}
