use quote::quote;

/// Custom derive for `abscissa_core::runnable::Runnable`
pub fn derive_runnable(s: synstructure::Structure<'_>) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        quote! { #bi.run(system)?; }
    });

    s.gen_impl(quote! {
        gen impl Runnable for @Self {
            fn run(&self, system: System) -> anyhow::Result<()> {
                match *self { #body };
                Ok(())
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn derive_runnable_on_enum() {
        test_derive! {
            derive_runnable {
                enum MyRunnable {
                    A(VariantA),
                    B(VariantB),
                    C(VariantC),
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_Runnable_FOR_MyRunnable: () = {
                    impl Runnable for MyRunnable {
                        fn run(&self, system: System) -> anyhow::Result<()> {
                            match *self {
                                MyRunnable::A(ref __binding_0,) => {
                                    { __binding_0.run(system)?; }
                                }
                                MyRunnable::B(ref __binding_0,) => {
                                    { __binding_0.run(system)?; }
                                }
                                MyRunnable::C(ref __binding_0,) => {
                                    { __binding_0.run(system)?; }
                                }
                            };
                            Ok(())
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `abscissa` crate
        }
    }
}
