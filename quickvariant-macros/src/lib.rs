use syn::punctuated::Punctuated;

#[proc_macro]
pub fn make_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(input),
            "Macro input cannot be empty. Please specify at least one type."
        ).to_compile_error().into();
    }
    let types = syn::parse_macro_input!(
        input with Punctuated::<syn::Type, syn::token::Comma>::parse_terminated
    );
    let sizes = types.iter().map(|ty| {
        quote::quote! { std::mem::size_of::<#ty>() }
    });
    let aligns = types.iter().map(|ty| {
        quote::quote! { std::mem::align_of::<#ty>() }
    });
    let ids = types.iter().map(|ty| {
        quote::quote! { std::any::TypeId::of::<#ty>() }
    });
    let errmsg = syn::LitStr::new(
        "Macro input cannot be empty. Please specify at least one type.",
        proc_macro2::Span::call_site()
    );
    quote::quote! {
        {
            let max_size = [#(#sizes),*].iter().copied().max().expect(#errmsg);
            let max_align = [#(#aligns),*].iter().copied().max().expect(#errmsg);
            let mut types = std::collections::HashSet::new();
            for ty in [#(#ids),*] {
                types.insert(ty);
            }
            quickvariant::Variant::__new(max_size, max_align, types)
        }
    }.into()
}