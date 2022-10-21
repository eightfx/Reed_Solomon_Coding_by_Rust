// 素体
#[derive(Debug)]
struct PrimeField{
	char:u8,
	num:i32
}
impl PrimeField{
	fn new(char:u8,num:i32)->PrimeField{
		let new_num = num % char as i32;
		PrimeField{char:char,num:new_num}
	}
	fn add(&self,other:&PrimeField)->PrimeField{
		PrimeField{char:self.char,num:(self.num+other.num) %self.char as i32}
	}
	fn sub(&self,other:&PrimeField)->PrimeField{
		let mut minus_num = (self.num-other.num) %self.char as i32;
		if minus_num < 0{
			minus_num += self.char as i32;
		}
		PrimeField{char:self.char,num:minus_num}
	}
	fn mul(&self,other:&PrimeField)->PrimeField{
		PrimeField{char:self.char,num:(self.num*other.num) %self.char as i32}
	}
	fn div(&self,other:&PrimeField)->PrimeField{
		PrimeField{char:self.char,num:(self.num*other.num.pow(self.char as u32-2)) %self.char as i32}
	}
	fn pow(&self,other:i32)->PrimeField{
		PrimeField{char:self.char,num:(self.num.pow(other as u32)) %self.char as i32}
	}

}

// 素体の有限拡大体
#[derive(Debug)]
struct FiniteField{
	char:u8,
	length:u8,
	elements:Vec<PrimeField>
}
impl FiniteField{
	fn new(char:u8, length:u8, elements:Vec<PrimeField>)->FiniteField{
		FiniteField{char,length,elements}
	}
	fn add(&self,other:FiniteField)->FiniteField{
		let mut result:Vec<PrimeField> = Vec::new();
		for i in 0..self.length{
			result.push(self.elements[i as usize].add(&other.elements[i as usize]));
		}
		FiniteField{char:self.char,length:self.length,elements:result}
	}
	fn sub(&self,other:FiniteField)->FiniteField{
		let mut result:Vec<PrimeField> = Vec::new();
		for i in 0..self.length{
			result.push(self.elements[i as usize].sub(&other.elements[i as usize]));
		}
		FiniteField{char:self.char,length:self.length,elements:result}
	}
	fn mul(&self,other:FiniteField)->FiniteField{
		let mut result:Vec<PrimeField> = Vec::new();
		for i in 0..self.length{
			result.push(self.elements[i as usize].mul(&other.elements[i as usize]));
		}
		FiniteField{char:self.char,length:self.length,elements:result}
	}
	
}

fn function(x:&PrimeField, u:FiniteField, char:u8, length:u8) ->PrimeField{
	let mut result = PrimeField{char:char,num:0};
	for i in 0..length{
		result = result.add(&x.pow(i.into()).mul(&u.elements[i as usize]));
	}
	result
	
}
/*
fn encode(P:Vec<PrimeField>,origin_sentense:FiniteField)->FiniteField{
	let mut result:Vec<PrimeField> = Vec::new();
	for i in 0..origin_sentense.length{
		result.push(P[i as usize].mul(&origin_sentense.elements[i as usize]));
	}


}
 */
fn main() {
	let char = 5;
	let length = 2;
	let origin_sentense = FiniteField::new(char,length,vec![PrimeField::new(char,1),PrimeField::new(char,2)]);
	let mut P = Vec::new();
	// 原始根を作成
	for i in 0..char-1{
		P.push(PrimeField::new(5,2_i32.pow(i.into())));
	}
	//P = FiniteField::new(char,length,P).elements;

	println!("P = {:?}",P);
	let u1 = function(&P[0],origin_sentense,char,length);
	println!("{:?}",u1);


}

