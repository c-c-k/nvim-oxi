use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, Path, Token, parse_macro_input, parse_quote};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};

#[allow(dead_code, reason = "disabled")] // TODO: Adjust to nvimo
#[inline]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attributes);

    let entrypoint = parse_macro_input!(item as ItemFn);

    let plugin_name = &entrypoint.sig.ident;

    let lua_module =
        Ident::new(&format!("luaopen_{plugin_name}"), Span::call_site());

    let nvimo = attrs.nvimo;

    quote! {
        #entrypoint

        #[unsafe(no_mangle)]
        unsafe extern "C" fn #lua_module(
            state: *mut #nvimo::lua::ffi::State,
        ) -> ::core::ffi::c_int {
            #nvimo::entrypoint::entrypoint(state, #plugin_name)
        }
    }
    .into()
}

#[allow(unused, reason = "unused by default")] // TODO: Adjust to nvimo
#[derive(Default)]
struct Attributes {
    nvimo: Nvimo,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut has_parsed_nvimo = false;

        while !input.is_empty() {
            let keypair = input.parse::<Attribute>()?;

            match keypair {
                Attribute::Nvimo(nvimo) => {
                    if has_parsed_nvimo {
                        return Err(DuplicateError(nvimo).into());
                    }
                    this.nvimo = nvimo;
                    has_parsed_nvimo = true;
                },
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(this)
    }
}

#[allow(unused, reason = "unused by default")] // TODO: Adjust to nvimo
enum Attribute {
    Nvimo(Nvimo),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Nvimo>().map(Self::Nvimo)
    }
}

pub(crate) struct Nvimo {
    key_span: Span,
    value: Path,
}

impl Default for Nvimo {
    #[inline]
    fn default() -> Self {
        Self { key_span: Span::call_site(), value: parse_quote!(::nvimo) }
    }
}

impl Parse for Nvimo {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            value: input.parse::<Keyed<Self>>()?.value,
        })
    }
}

impl KeyedAttribute for Nvimo {
    const KEY: &'static str = "nvimo";

    type Value = Path;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl ToTokens for Nvimo {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens);
    }
}
