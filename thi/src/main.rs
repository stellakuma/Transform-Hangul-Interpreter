use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

fn main(){
 
	let c = "곰".nfd().collect::<String>();
	let i: [i32; ] =
	
	println!("{}", c);

}