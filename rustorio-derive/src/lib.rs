use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, DeriveInput, Ident, LitInt, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct RecipeItemAttrArgs(LitInt, Type);

impl Parse for RecipeItemAttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let amount = content.parse()?;
        let _ = content.parse::<Token![,]>()?;
        let ty = content.parse()?;
        Ok(Self(amount, ty))
    }
}

struct RecipeItemsAttr(Punctuated<RecipeItemAttrArgs, Token![,]>);

impl Parse for RecipeItemsAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(
            input.parse_terminated(RecipeItemAttrArgs::parse, Token![,])?,
        ))
    }
}

fn derive_recipe_process_attr(
    attr: &Attribute,
    attr_name: &str,
    item_type_name: &str,
    amount_type_name: &str,
    amount_const_name: &str,
    new_fn_name: &str,
    iter_fn_name: &str,
) -> (TokenStream, TokenStream) {
    let Ok(inner) = attr.parse_args::<RecipeItemsAttr>() else {
        panic!("Invalid \"{attr_name}\" args");
    };

    let per_type = inner
        .0
        .iter()
        .map(|RecipeItemAttrArgs(lit, ty)| {
            let amount = lit
                .base10_parse::<u32>()
                .unwrap_or_else(|_| panic!("Invalid amount in \"{attr_name}\" args"));
            (amount, ty)
        })
        .collect::<Vec<_>>();

    let amount_type_ident = Ident::new(amount_type_name, Span::call_site());
    let amount_const_ident = Ident::new(amount_const_name, Span::call_site());
    let amount_types = per_type.iter().map(|_| quote! {u32});
    let amounts = per_type.iter().map(|(amount, _)| amount);

    let item_type_ident = Ident::new(item_type_name, Span::call_site());
    let recipe_items = per_type
        .iter()
        .map(|(amount, ty)| quote! {::rustorio_engine::recipe::RecipeItem<#amount, #ty>});

    let new_fn_ident = Ident::new(new_fn_name, Span::call_site());
    let new_values = per_type
        .iter()
        .map(|_| quote! {::rustorio_engine::recipe::RecipeItem::default()});

    let iter_fn_ident = Ident::new(iter_fn_name, Span::call_site());
    let iter_values = (0..per_type.len())
        .map(|i| LitInt::new(&i.to_string(), Span::call_site()))
        .map(|i| {
            quote! {(
                Self::#amount_const_ident.#i,
                ::rustorio_engine::recipe::recipe_item_amount(&mut items.#i)
            )}
        });

    (
        quote! {
            type #item_type_ident = (#(#recipe_items,)*);

            type #amount_type_ident = (#(#amount_types,)*);
            const #amount_const_ident: Self::#amount_type_ident = (#(#amounts,)*);
        },
        quote! {
            fn #new_fn_ident() -> Self::#item_type_ident {
                (#(#new_values,)*)
            }

            fn #iter_fn_ident(
                items: &mut Self::#item_type_ident
            ) -> impl Iterator<Item = (u32, &mut u32)> {
                [#(#iter_values,)*].into_iter()
            }
        },
    )
}

fn derive_recipe_inner(input: DeriveInput) -> TokenStream {
    let mut inputs = None;
    let mut outputs = None;
    let mut ticks = None;
    for attr in &input.attrs {
        if attr.path().is_ident("recipe_inputs") {
            inputs = Some(derive_recipe_process_attr(
                attr,
                "recipe_inputs",
                "Inputs",
                "InputAmountsType",
                "INPUT_AMOUNTS",
                "new_inputs",
                "iter_inputs",
            ));
        } else if attr.path().is_ident("recipe_outputs") {
            outputs = Some(derive_recipe_process_attr(
                attr,
                "recipe_outputs",
                "Outputs",
                "OutputAmountsType",
                "OUTPUT_AMOUNTS",
                "new_outputs",
                "iter_outputs",
            ));
        } else if attr.path().is_ident("recipe_ticks") {
            ticks = Some(
                attr.parse_args::<LitInt>()
                    .expect("Invalid \"recipe_ticks\" value"),
            );
        }
    }
    let (inputs, inputs_ex) = inputs.expect("Missing \"recipe_inputs\" attribute");
    let (outputs, outputs_ex) = outputs.expect("Missing \"recipe_outputs\" attribute");
    let ticks = ticks.expect("Missing \"recipe_ticks\" attribute");

    let name = input.ident;
    quote! {
        impl ::rustorio_engine::recipe::Recipe for #name {
            const TIME: u64 = #ticks;

            #inputs
            #outputs
        }

        impl ::rustorio_engine::recipe::RecipeEx for #name {
            #inputs_ex
            #outputs_ex
        }
    }
}

#[proc_macro_derive(Recipe, attributes(recipe_inputs, recipe_outputs, recipe_ticks))]
pub fn derive_recipe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = derive_recipe_inner(input);
    proc_macro::TokenStream::from(output)
}
