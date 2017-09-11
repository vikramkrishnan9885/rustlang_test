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
	// macros are expanded into source code that gets compiled 
	// with the rest of the program.

	println!("Hello World!");

	// We will now experiment with formatted printing
	// Printing is handled by a series of macros defined in std::fmt
	//, which contains several traits that govern the display of text
	// Base traits are: fmt::Debug that uses the {:?} marker that formats
	// text for debugging purposes and fmt::Display that formats text for users
	
	// Traits are collection of methods defined for an unkown type: self
	// They can access other methods declared in the same trait
	// Traits can be implemented for any data type
	// There’s a lot that’s possible with traits. We can add multiple traits to a single struct, 
	// you can see examples of this all over the Rust standard library. 
	// Creating multiple traits lets us define fine grained units of functionality 
	// and apply them wherever we need them. 

	// The three macros that are most commonly used are:
	// format! : write formatted text to String
	// print! : similar to print in Ruby and Java
	// println! : similar to println in Ruby and System.out.println in Java

	println!("{} days", 31); // Stringify the argument and replace {}
	println!(31);
	println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob"); // Positional arguments can be passeed
	println!("{variable1} {variable2} {variable3}"
			, variable1 = "This is the first variable"
			, variable2 = "This is the second variable"
			, variable3 = "This is the third variable"
			); // Using named arguments
	println!("{}, in binary {:b}, in hex {:x}, in octal {:o}, in float {4:.1$}", 11,11,11,11,11.0);
}
