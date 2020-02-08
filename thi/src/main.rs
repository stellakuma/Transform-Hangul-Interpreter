use korean::*;

fn main(){
 
	println!("{}", esr('갋'));

}

//끝소리 규칙 ( end sound rule )
/*

	::Example::
	esr( 'letter' );
	letter must be korean! ex) 갂

*/
fn esr( g: char ) -> String {

	let r = g.to_string();

	let c = r.disassemble();
	let mut s: Vec<char> = Vec::new();
	let mut a: bool = true;
	let mut replace: char = 'e';
 
	for p in c {
	
		if a == true {
	
			s = Vec::new();
			a = false;
		
		}
		s.push(p);
	
	}
	
	//3 umjeol
	if s.len() == 3 {

		replace = s[2];
		replace = match replace {
	
			'ㄲ'|'ㅋ' => 'ㄱ',
			'ㅆ'|'ㅅ'|'ㅈ'|'ㅊ'|'ㅌ'|'ㅎ' => 'ㄷ',
			'ㅍ' => 'ㅂ',
			_ => replace
			
		};
	
	//4 umjeol
	} else if s.len() == 4 {
	
		let r1 = s[2];
		let r2 = s[3];
		replace = match( r1, r2 ){
	
			( 'ㄱ', 'ㅅ' ) => 'ㄱ',
			( 'ㄴ', 'ㅈ' ) => 'ㄴ',
			( 'ㄹ', 'ㅅ' ) => 'ㄹ',
			( 'ㄹ', 'ㅌ' ) => 'ㄹ',
			( 'ㅂ', 'ㅅ' ) => 'ㅂ',
			( 'ㄹ', 'ㅁ' ) => 'ㅁ',
			( 'ㄹ', 'ㅍ' ) => 'ㅂ',
			//자음에 따른 보정 필요
			( 'ㄹ', 'ㄱ' ) => 'ㄱ',
			//밟, 넓에 대한 보정 필요
			( 'ㄹ', 'ㅂ' ) => 'ㄹ',
			_ => replace
	
		}
	
	}
	
	let mut result: [char; 3] = [ 'e', 'x', 'p' ];
	let mut counter: usize = 0;
	for piece in s {
		
		if counter == 2 {
	
			result[2] = replace;
			break;
	
		} else {
		
			result[counter] = piece;
			counter = counter + 1;
		 
		}
	}
	
	return result.iter().cloned().assemble();
	
	/*
	for test in result.iter(){
	
		println!("{}", test);
	
	}
	*/
	
	/* //제대로 분리됐는지 확인!!
	for p in s{
		println!('{}', p);
	}
	*/
}