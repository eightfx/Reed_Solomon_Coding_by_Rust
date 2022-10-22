// 素体
#[derive(Debug)]
#[derive(Clone)]
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
	fn pow(&self,other:i32)->PrimeField{
		PrimeField{char:self.char,num:(self.num.pow(other as u32)) %self.char as i32}
	}
	fn div(&self,other:&PrimeField)->PrimeField{
		let mut t = self.extended_euclidean(self.char as i32, other.num);
		if t<0{
			let mut i = 1;
			
			while (t+i*self.char as i32) < 0{
				i += 1;
			}
			t = (t+i*self.char as i32) % self.char as i32;
		}
		PrimeField{char:self.char,num:(self.num*t) %self.char as i32}
	}
	fn extended_euclidean(&self,u: i32, v: i32) -> i32 {
		let mut r0 = u;
		let mut r1 = v;
		let mut s0 = 1;
		let mut s1 = 0;
		let mut t0 = 0;
		let mut t1 = 1;
		while r1 != 0 {
			let q = r0 / r1;
			let r = r0 - q * r1;
			let s = s0 - q * s1;
			let t = t0 - q * t1;
			r0 = r1;
			s0 = s1;
			t0 = t1;
			r1 = r;
			s1 = s;
			t1 = t;
		}
		if t0 < 0 {
			t0 + u
		} else {
			t0
		}
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
	fn toVec(&self)->Vec<u8>{
		let mut result:Vec<u8> = Vec::new();
		for i in 0..self.elements.len(){
			result.push(self.elements[i as usize].num as u8);
		}
		result
	}
	
}

// u係数のx変数多項式
fn function(x:&PrimeField, u:&FiniteField, char:&u8, length:&u8) ->PrimeField{
	let mut result = PrimeField{char:*char,num:0};
	for i in 0..*length{
		let mut tmp = &x.pow(i.into()).mul(&u.elements[i as usize]);
		result = result.add(&tmp);

	}
	result
	
}
fn encode(P:&Vec<PrimeField>,origin_sentense:FiniteField, char:&u8, length:&u8)->FiniteField{
	let mut u = Vec::new();
	for i in 0..char-1{
		let mut temp = function(&P[i as usize], &origin_sentense, char, length);
		u.push(temp);
	}
	FiniteField{char:*char,length:*length,elements:u}

}
fn main() {
	let char = 11; // n+1
	let length =5;

	let n = &char-1; // 符号込の長さ
	let k = &length; // 文章の長さ
	let d = n - k + 1; // 最小距離
	let t = (n-k)/2;
	
	let l_0 = &n - 1 - &t;
	let l_1 = &n - 1 - &t - (k - 1);
	println!("n:{},k:{},d:{},t:{},l_0:{},l_1:{}",n,k,d,t,l_0,l_1);

	// 送りたい文章
	let origin_sentense = FiniteField::new(char,length,
										   vec![PrimeField::new(char,0),
												PrimeField::new(char,1),
												PrimeField::new(char,0),
												PrimeField::new(char,0),
												PrimeField::new(char,0)]);


	let mut P = Vec::new();
	// 原始根を１つ固定
	let primitive_element:i32 = 2;

	// 原始根を生成
	for i in 0..char-1{
		P.push(PrimeField::new(char,primitive_element.pow(i.into())));
	}
	P = FiniteField::new(char,length,P).elements;
	// 符号化
	let u = encode(&P,origin_sentense,&char,&length);

	// 送信でエラーを起こす
	//let mut u_received = u.toVec();
	let mut u_received = vec![5,9,0,9,0,1,0,7,0,5];
	

	let u_received = FiniteField::new(char, length, u_received.into_iter().map(|x|PrimeField::new(char,x as i32)).collect());
	
	// シンドローム
	/*
	let mut S:Vec<PrimeField> = Vec::new();
	for i in 1..n-k{
		S.push(function(&P[i as usize], &u_received, &char, &n));
	}
	println!("S:{:?}",S);
	 */	

	// 復号行列の作成
	let mut H:Vec<Vec<PrimeField>> = Vec::new();
	for i in 0..n{
		let mut tmp:Vec<PrimeField> = Vec::new();
		for j in 0..l_0+1{
			tmp.push(P[i as usize].pow(j.into()));
		}
		for j in 0..l_1+1{
			tmp.push(u_received.elements[i as usize].mul(&P[i as usize].pow(j.into())));
		}
		H.push(tmp);
		
	}

	// 復号行列を掃き出し法で変形
	for i in 0..n{
		// 0の場合は交換
		for j in i..n{
			if H[i as usize][i as usize].num != 0{
				break;
			}
			else{
				let tmp = H[i as usize].clone();
				H[i as usize] = H[j as usize].clone();
				H[j as usize] = tmp;
			}
		}
		// 1になるように掛ける
		for j in 0..l_0+l_1+2{

			let mut head = &H[i as usize][i as usize];
			let mut h_ij = &H[i as usize][j as usize];
			H[i as usize][j as usize] = h_ij.div(&head);
		}
		// 0になるように引く
		for j in 0..l_0+l_1+2{
			for k in 0..n{
				if k == i{
					continue;
				}
				let mut h_kj = &H[k as usize][j as usize];
				let mut h_ki = &H[k as usize][i as usize];
				let mut h_ij = &H[i as usize][j as usize];
				H[k as usize][j as usize] = h_kj.sub(&h_ki.mul(&h_ij));

				
			}

		}
	}

	// 行列のランク
	let mut rank = 0;
	for i in 0..n{
		if H[i as usize][i as usize].num != 0{
			rank += 1;
			}
	}

	// 非零解の１つを求める
	let mut Q:Vec<PrimeField> = Vec::new();
	for i in 0..rank{
		let mut temp = H[i as usize][rank..].to_vec();
		let mut answer = PrimeField::new(char,0);
		for j in 0..temp.len(){
			answer = answer.sub(&temp[j as usize].mul(&PrimeField::new(char,1)));
		}
		Q.push(answer);
	}
	for i in rank..(l_0+l_1+2) as usize{
		Q.push(PrimeField::new(char,1));
	}


	let Q_num:Vec<PrimeField> = Q.iter().map(|x|PrimeField::new(char,x.num)).collect();

	// 誤り位置多項式を求める
	let Q0 = FiniteField::new(char,length,Q_num[0..(l_0+1) as usize].to_vec().into_iter().rev().collect::<Vec<PrimeField>>());
	let Q1 = FiniteField::new(char,length,Q_num[(l_0+1) as usize ..(l_0+l_1+2) as usize].to_vec().into_iter().rev().collect::<Vec<PrimeField>>());
	
	

	// Q0をQ1で割った商を求める
	let mut Q0_div_Q1:Vec<PrimeField> = Vec::new();
	let mut i = 0;

	
	println!("Q:{:?}",Q0);
	println!("Q:{:?}",Q1);


}

