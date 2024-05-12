//! yes, i did make this exclusively so i don't have to type
//! `#[cfg(feature = "defmt")]` a bajillion times

#![no_std]

use proc_macro::TokenStream;
use quote::quote;

macro_rules! defmt_stmt {
    ($name: tt) => {
        #[proc_macro]
        pub fn $name(stream: TokenStream) -> TokenStream {
            let stream = proc_macro2::TokenStream::from(stream);

            quote! {
                {
                    #[cfg(feature = "defmt")]
                    ::defmt::$name!(#stream)
                }
            }.into()
        }
    };
}

// formatting

defmt_stmt!(format);

// simple stuff

defmt_stmt!(dbg);
defmt_stmt!(println);
defmt_stmt!(intern);
defmt_stmt!(internp);

// assertions

defmt_stmt!(assert);
defmt_stmt!(assert_eq);
defmt_stmt!(assert_ne);

// debugging

defmt_stmt!(debug_assert);
defmt_stmt!(debug_assert_eq);
defmt_stmt!(debug_assert_ne);

// logging

defmt_stmt!(trace);
defmt_stmt!(debug);
defmt_stmt!(info);
defmt_stmt!(warn);
defmt_stmt!(error);

// convenience

defmt_stmt!(panic);
defmt_stmt!(todo);
defmt_stmt!(unreachable);
defmt_stmt!(unwrap);
defmt_stmt!(write);
defmt_stmt!(bitflags);
