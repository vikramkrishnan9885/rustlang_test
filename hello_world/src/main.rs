// This is how you create a comment

/*
This
is
a
multiline
comment
*/

#![crate_name = "hello_world"]
/// This is used for creating documentation (Literate Programming)
/// * Supports Markdown
/// If you pass --test to Rustdoc, it will also run the tests for you

// Create the main function

fn main(){

	// These statements will be executed when the compiled binary is called

	// Print to console

	// println! is a macro that prints text to the console.
	// Rust provides a powerful macro system that allows metaprogramming. 
	// As you've seen in previous chapters, macros look like functions, 
	// except that their name ends with a bang !, 
	// but instead of generating a function call, 
	// macros are expanded into source code that gets compiled with the rest of the program.

	println!("Hello World!");

}