extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;


#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream
{ 
	// Parse Phase
	let derive_input = parse_macro_input!(input as DeriveInput);
	let ident = &derive_input.ident;
	
	// Generate Phase
	(quote! {
		impl Component for #ident {
			fn as_any(&self) -> &dyn std::any::Any					{ self }
			fn as_any_mut(&mut self) -> &mut dyn std::any::Any		{ self }
		}
	}).into()
}


