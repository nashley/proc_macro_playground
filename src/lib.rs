use proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DeriveMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();

	return impl_display(&ast);
}

#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut output = input.clone(); // regurgitate the struct/enum definition, since proc_macro_attribute consumes it
	let attr_ast = syn::parse(attr).unwrap();
	let ast = syn::parse(input).unwrap();

	output.extend(impl_display_attr(&ast, &attr_ast));

	return output;
}

fn impl_display(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	return generate_output(name, stringify!(name));
}

fn impl_display_attr(ast: &syn::DeriveInput, attr_ast: &syn::ExprAssign) -> TokenStream {
	let name = &ast.ident;
	let left = &attr_ast.left;
	let right = &attr_ast.right;
	let expr = &**left;
	println!("expr: {:#?}", expr);
	match expr {
		syn::Expr::Path(expr) => match expr.path.segments[0].ident.to_string().as_str() {
			"name" => {
				return generate_output(name, right);
			}
			_ => panic!("Only `name` may be specified"),
		},
		_ => {
			panic!("idk how this branch is reached")
		}
	}
}

fn generate_output(
	name: impl quote::ToTokens,
	display_name: impl quote::ToTokens,
) -> proc_macro::TokenStream {
	let gen = quote! {
		impl Display for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "Hello! My name is \"{}\".", #display_name)
			}
		}
	};
	gen.into()
}

//#[cfg(test)]
//mod tests {
//    use crate::DeriveMacro;
//    #[derive(DeriveMacro)]
//    struct Derive{}
//
//    #[attribute_macro(name=2 * "asdf".len())]
//    struct Attribute{}
//
//    #[test]
//    fn it_works() {
//        assert_eq!(format!("{}", Derive{}), "Hello! My name is \"Derive\".");
//        assert_eq!(format!("{}", Attribute{}), "Hello! My name is \"8\".");
//    }
//}
