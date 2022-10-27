use rand::Rng;
use galois_field::*;

fn hamming_distance(f:Polynomial, g:Polynomial) -> u32{
	let mut distance = 0;
	for i in 0..f.coef.len(){
		if !(f.coef[i].clone()-g.coef[i].clone()).is_0(){
			distance += 1;
		}
	}
	distance
	
}
fn poly_to_string(poly:Polynomial) -> String {
	let mut s = String::new();
	for i in 0..poly.coef.len() {
		let mut temp = 0;
		if let Element::PrimeField{element:e}= poly.coef[i].element{
			temp = e;
		}
		s = s + &temp.to_string();

	}
	s
}
// 行列の各要素をPrimeFieldからi64に変換することでprintln!で見やすくする
fn matrix_visualize(matrix: Matrix) -> Vec<Vec<i64>> {
    let mut h_num: Vec<Vec<i64>> = Vec::new();
    for i in 0..matrix.element.len() {
        let mut tmp: Vec<i64> = Vec::new();
        for j in 0..matrix.element[0].len() {
			if let Element::PrimeField{element:e} = matrix.element[i as usize][j as usize].element{
				tmp.push(e as i64);
			}
			
		}
        h_num.push(tmp);
    }
    h_num
}

fn main(){
	// メタパラメータ
	let char:u32 = 7;
	let length =2;


	let mut rng = rand::thread_rng();

	// 文章のランダム生成
	let mut sentence:Polynomial = Polynomial{
		coef : Vec::new(),
	};
	let mut sentence_str:String = String::new();
	for _ in 0..length{
		let num:i32 = rng.gen_range(0, char as i32);
		let element = FiniteField{
			char:char,
			element: Element::PrimeField { element: num.clone() }
		};
		
		sentence.coef.push(element);
		sentence_str.push_str(&num.to_string());
	}


	// 原始根を１つ固定
    let mut primitive_element:FiniteField = FiniteField{
		char:char,
		element: Element::PrimeField { element: 3 }
	};
	let origin_pe = primitive_element.clone();
	// 原始根から巡回群を作成
	let mut P_vec:Vec<FiniteField> = Vec::new();
	for i in 0..char - 1 {
		P_vec.push(primitive_element.clone());
		primitive_element = primitive_element * origin_pe.clone();
	}
	let P:Polynomial = Polynomial{
		coef:P_vec
	};
	println!("P:{}",poly_to_string(P.clone()));
	
	// 各数値の計算
	let n = P.coef.len(); // 符号長
	let k = length.clone(); // 文章の長さ
	let d = n - k + 1; // 最小距離
	let t = (n-k)/2; // エラー訂正能力
	let l_0 = n.clone() - 1 - t.clone();
	let l_1 = n.clone() - 1 - t.clone() - (k.clone() - 1);
	println!("t:{}",t);
	println!("l_0 = {}", l_0);
	println!("l_1 = {}", l_1);
	println!("[{},{},{}]-_{}code", n, k, d, char);
	println!("sentence: {:?}", sentence_str);

	// リードソロモン符号の生成
	let mut u:Polynomial = Polynomial{
		coef:Vec::new()
	};
	for i in 0..n{
		let temp = sentence.assign_value(P.coef[i].clone());
		u.coef.push(temp);
	}

	// 可視化
	let code = poly_to_string(u.clone());
	println!("send: {:?}", code);

	// エラーの生成

	let mut u_received:Polynomial = u.clone();
	let error_count = rng.gen_range(0,n);

	for _ in 0..error_count{
		let error_position = rng.gen_range(0,n);
		let error_value = rng.gen_range(0,char);
		u_received.coef[error_position] = FiniteField{
			char:char,
			element: Element::PrimeField { element: error_value as i32 }
		};
	}
	// let mut error_distance = 0;
	// for i in 0..u.coef.len(){
	// 	let temp = u.coef[i].clone() - u_received.coef[i].clone();
	// 	if let Element::PrimeField{element:e} = temp.element{
	// 		error_distance += e
	// 	}
	// }
	// println!("error_distance:{}",error_distance);

	// 可視化
	let received_code = poly_to_string(u_received.clone());
	println!("received: {:?}", received_code);
	
	
	// 復号行列の作成
	let mut H:Matrix = Matrix{
		element:Vec::new()
	};
	for i in 0..n{
		let mut tmp: Vec<FiniteField> = Vec::new();
		let mut P_i = P.coef[i].clone();
		let mut origin_P_i = P.coef[i].clone();
		for j in 0..l_0 + 1{
			if j == 0{
				tmp.push(P.coef[i].get_1());
			}else{
				tmp.push(P_i.clone());
				P_i = P_i * origin_P_i.clone();
			}
		}
		let mut P_i = P.coef[i].clone();
		let mut origin_P_i = P.coef[i].clone();
		for j in 0..l_1 + 1{
			if j == 0{
				tmp.push(u_received.coef[i].clone());
			}else{
				tmp.push(u_received.coef[i].clone() * P_i.clone());
				P_i = P_i * origin_P_i.clone();
			}
		}
		H.element.push(tmp);
	}

	
	H = H.sweep_method();

	// 行列のランク
	let mut rank = 0;
	for i in 0..n {
		if !H.element[i as usize][i as usize].is_0() {
			rank += 1;
		}
	}

	let mut Q:Polynomial = Polynomial{
		coef:Vec::new()
	};
	for i in 0..rank{
		let temp = H.element[i][rank..].to_vec();
		let mut answer = H.element[0][0].get_0();
		for j in 0..temp.len(){
			answer = answer - temp[j].clone();
		}
		Q.coef.push(answer);
	}
	for _ in rank..(l_0+l_1+2){
		Q.coef.push(H.element[0][0].get_1().clone());
	}
	println!("Q: {:?}", poly_to_string(Q.clone()));

	let mut Q0:Polynomial = Polynomial{
		coef:Q.coef[0..l_0+1].to_vec()
	};
	let mut Q1:Polynomial = Polynomial{
		coef:Q.coef[l_0+1..].to_vec()
	};

	println!("Q0: {:?}", poly_to_string(Q0.clone()));
	println!("Q1: {:?}", poly_to_string(Q1.clone()));
	let mut quotient = Q0.clone() / Q1.clone();
	let remainder = Q0.clone() % Q1.clone();
	println!("quotient: {:?}", poly_to_string(quotient.clone()));
	println!("remainder: {:?}", poly_to_string(remainder.clone()));
	for i in 0..quotient.coef.len(){
		quotient.coef[i] = -quotient.coef[i].clone();
	}
	// 復号可能か調べる : 余りが０なら復号可能
	let mut decodable_flag:bool = true;
	for i in 0..remainder.coef.len(){
		if !remainder.coef[i].is_0(){
			decodable_flag = false;
			break;
		}
	}

	let mut decode_sentence:Polynomial = Polynomial{
		coef:Vec::new()};

	for i in 0..n{
		decode_sentence.coef.push(quotient.assign_value(P.coef[i].clone()));
	}

	let distance = hamming_distance(decode_sentence.clone(),u_received.clone());
	println!("distance: {}", distance);
	if (decodable_flag == false) || (distance > (t as u32)){
		println!("復元不可能");
		std::process::exit(0);
	}



	let decode_sentence_str = poly_to_string(decode_sentence.clone());
	println!("decode: {:?}", decode_sentence_str);
	if poly_to_string(u) == decode_sentence_str{
		println!("成功");
	}
	else{
		println!("失敗");
	}
}
