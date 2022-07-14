use quote::quote;

pub fn derive_has_deps(s: synstructure::Structure<'_>) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        quote! { #bi.deps() }
    });

    s.gen_impl(quote! {
        gen impl HasDeps for @Self {
            fn deps(&self) -> HashSet<TypeId> {
                match *self { #body }
            }
        }
    })
}
