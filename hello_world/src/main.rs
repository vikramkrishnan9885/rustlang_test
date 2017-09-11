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

use std::fmt; // This is used to import 'fmt'. This is similar to the import statement in python and Java

// We want to derive the fmt::Debug implemetation for Structure
#[derive(Debug)]
struct Structure(i32);
// However, we will implement our own fmt::Display for Structure
impl fmt::Display for Structure {
	// This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

// Now we will try and implement fmt::Disply for a collection like structure

struct List(Vec<i32>); // Define a structure named List containing a Vec

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	// Implementing fmt::Display for a structure where the elements must 
	// each be handled sequentially is tricky. The problem is that each 
	// write! generates a fmt::Result. Proper handling of this requires 
	// dealing with all the results. Rust provides the try! macro and 
	// alternatively the equivalent ? operator for exactly this purpose.
	// try!(write!(f, "{}", value)); or with the ? shorthand
	// write!(f,"{}", value)?;

	// Instead of taking Vec<i32>s as our arguments, we take a reference: 
	// &Vec<i32>. And instead of passing v1 and v2 directly, we pass &v1 
	// and &v2. We call the &T type a ‘reference’, and rather than owning 
	// the resource, it borrows ownership. A binding that borrows something 
	// does not deallocate the resource when it goes out of scope. 
	// This means that after the call to foo(), we can use our original 
	// bindings again. References are immutable, like bindings. 
	// There’s a second kind of reference: &mut T. A ‘mutable reference’ 
	// allows you to mutate the resource you’re borrowing. 
	let vec = &self.0;

    write!(f, "[")?;

    // Iterate over `vec` in `v` while enumerating the iteration
    // count in `count`.
    for (count, v) in vec.iter().enumerate() {
        // For every element except the first, add a comma.
        // Use the ? operator, or try!, to return on errors.
        if count != 0 { write!(f, ", ")?; }
        write!(f, "{}", v)?;
    }

    // Close the opened bracket and return a fmt::Result value
    write!(f, "]")
	}
}


// Playing around with basic Ownership related concepts

// Immutable references
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2

    // return the answer
    42
}

fn print_foo(){

	let v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];

	let answer = foo(&v1, &v2);

	println!("Meaning of life is {}", answer);
}

fn print_mutable(){
	// Mutable references
	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}
	println!("{}", x);
}

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

	// Now let us try println with structures
	println!("Now {:?} will print!", Structure(3));
	println!("Now {:?} will print!", Deep(Structure(7)));

	// Now let us try println Display with structures
	println!("Now {} will print!", Structure(3));

	// Collection type structures
	let v = List(vec![1, 2, 3]);
    println!("{}", v);

	// Testing Mutable and immutable references
	print_foo();
	print_mutable();
}
