use proc_macro_playground::{attribute_macro, DeriveMacro};
use std::fmt::Display;

#[derive(DeriveMacro)]
struct DerivedMacroNameWorks {}

#[attribute_macro(name = "asdf2")]
struct AttributeMacroNameWorksWithStr {}

#[attribute_macro(name = 1 + 1)]
struct AttributeMacroNameWorksWithExpression {}

#[attribute_macro(name = test_mod::CONSTANT_STR)]
struct AttributeMacroNameWorksWithConstant {}

#[attribute_macro(name = 2 * "Some string here".len())]
struct AttributeMacroNameWorksWithMix {}

#[attribute_macro(name = true)]
struct AttributeMacroNameWorksWithBool {}

// This would cause a compiler error:
//#[attribute_macro(name = 2 * "Some string here")]
//struct AttributeMacroNameWorksWithFail {}

mod test_mod {
	pub const CONSTANT_STR: &str = "Look mom, no function!";
}

fn main() {
	let test = DerivedMacroNameWorks {};
	println!("{}", test);

	let another = AttributeMacroNameWorksWithStr {};
	println!("{}", another);

	let another = AttributeMacroNameWorksWithExpression {};
	println!("{}", another);

	let another = AttributeMacroNameWorksWithConstant {};
	println!("{}", another);

	let another = AttributeMacroNameWorksWithMix {};
	println!("{}", another);

	let another = AttributeMacroNameWorksWithBool {};
	println!("{}", another);
}
