use rand::Rng;
// 素体
#[derive(Debug, Clone)]
struct PrimeField {
    char: u16,
    num: i64,
}
impl PrimeField {
    fn new(char: u16, num: i64) -> PrimeField {
        let new_num = num % char as i64;
        PrimeField {
            char,
            num: new_num,
        }
    }
    fn add(&self, other: &PrimeField) -> PrimeField {
        PrimeField {
            char: self.char,
            num: (self.num + other.num) % self.char as i64,
        }
    }
    fn sub(&self, other: &PrimeField) -> PrimeField {
        let mut minus_num = (self.num - other.num) % self.char as i64;
        if minus_num < 0 {
            minus_num += self.char as i64;
        }
        PrimeField {
            char: self.char,
            num: minus_num,
        }
    }
    fn mul(&self, other: &PrimeField) -> PrimeField {
        let mut mul_num = (self.num * other.num) % self.char as i64;
        if mul_num < 0 {
            mul_num += self.char as i64;
        }

        PrimeField {
            char: self.char,
            num: mul_num,
        }
    }
    fn pow(&self, other: i64) -> PrimeField {
        PrimeField {
            char: self.char,
            num: (self.num.pow(other as u32)) % self.char as i64,
        }
    }
    fn div(&self, other: &PrimeField) -> PrimeField {
        let mut t = self.extended_euclidean(self.char as i64, other.num);
        if t < 0 {
            let mut i = 1;

            while (t + i * self.char as i64) < 0 {
                i += 1;
            }
            t = (t + i * self.char as i64) % self.char as i64;
        }
        PrimeField {
            char: self.char,
            num: (self.num * t) % self.char as i64,
        }
    }
	// ユークリッドの互除法
    fn extended_euclidean(&self, u: i64, v: i64) -> i64 {
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
struct FiniteField {
    char: u16,
    elements: Vec<PrimeField>,
}
impl FiniteField {
    fn new(char: u16, elements: Vec<PrimeField>) -> FiniteField {
        FiniteField {
            char,
            elements,
        }
    }
    fn add(&self, other: FiniteField) -> FiniteField {
        let mut result: Vec<PrimeField> = Vec::new();
		let length = self.elements.len();
		for i in 0..length {
			result.push(self.elements[i as usize].add(&other.elements[i as usize]));
		}
        FiniteField {
            char: self.char,
            elements: result,
        }
    }
    fn sub(&self, other: FiniteField) -> FiniteField {
        let mut result: Vec<PrimeField> = Vec::new();
		let length = self.elements.len();
        for i in 0..length {
            result.push(self.elements[i as usize].sub(&other.elements[i as usize]));
        }
        FiniteField {
            char: self.char,
            elements: result,
        }
    }
    fn mul(&self, other: FiniteField) -> FiniteField {
        let mut result: Vec<PrimeField> = Vec::new();

		let length = self.elements.len();
        for i in 0..length {
            result.push(self.elements[i as usize].mul(&other.elements[i as usize]));
        }
        FiniteField {
            char: self.char,
            elements: result,
        }
    }
    fn toVec(&self) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::new();

		let length = self.elements.len();
        for i in 0..length {
			result.push(self.elements[i as usize].num as u16);
        }
        result
    }
}

// 掃き出し法
fn sweep_method(mut H:Vec<Vec<PrimeField>>) -> Vec<Vec<PrimeField>>{
	let n = H.len();
	let m = H[0].len();

	// 復号行列を掃き出し法で変形
    for i in 0..n - 1 {
        // 0の場合は交換
        for j in i..n {
            if H[i as usize][i as usize].num != 0 {
                break;
            } else {
                let tmp = H[i as usize].clone();
                H[i as usize] = H[j as usize].clone();
                H[j as usize] = tmp;
            }
        }
        // 1になるように掛ける

        let head = &H[i as usize][i as usize].clone();
        for j in 0..m{
            let h_ij = &H[i as usize][j as usize];
            H[i as usize][j as usize] = h_ij.div(head);
        }
        let mut h_xi: Vec<PrimeField> = Vec::new();
        for k in 0..n {
            h_xi.push(H[k as usize][i as usize].clone());
        }
        // 0になるように引く
        for j in 0..m{
            // k列全ての値を取得する
            let h_ij = &H[i as usize][j as usize].clone();
            for k in 0..n {
                if i == k {
                    continue;
                }

                let h_kj = &H[k as usize][j as usize];
                let h_ki = h_xi[k as usize].clone();
                H[k as usize][j as usize] = h_kj.sub(&h_ki.mul(h_ij));
            }
        }
    }
	H
}
// u係数のx変数多項式
fn function(x: &PrimeField, u: &FiniteField, char: &u16) -> PrimeField {
	let length = (u.toVec().len()) as usize;
	let mut result = PrimeField {
		char: *char,
		num: 0,
	};
	for i in 0..length {
		let tmp = &x.pow(i.try_into().unwrap()).mul(&u.elements[i as usize]);
		result = result.add(tmp);
	}
	result
}

// 符号化
fn reed_solomon_encode(
    P: &Vec<PrimeField>,
    origin_sentense: FiniteField,
    char: &u16,
) -> FiniteField {
	let length = P.len();
	let mut u = Vec::new();
    for i in 0..char - 1 {
        let temp = function(&P[i as usize], &origin_sentense, char);
        u.push(temp);
    }
    FiniteField {
        char: *char,
        elements: u,
    }
}

// 行列の各要素をPrimeFieldからi64に変換することでprintln!で見やすくする
fn matrix_visualize(matrix: &Vec<Vec<PrimeField>>, n: &u16, l_0: &u16, l_1: &u16) -> Vec<Vec<i64>> {
    let mut h_num: Vec<Vec<i64>> = Vec::new();
    for i in 0..*n {
        let mut tmp: Vec<i64> = Vec::new();
        for j in 0..l_0 + l_1 + 2 {
            tmp.push(matrix[i as usize][j as usize].num);
        }
        h_num.push(tmp);
    }
    h_num
}

fn main() {
	// ここからパラメータ

    let char = 17; // 標数
    // 送りたい文章
    let origin_sentense = FiniteField::new(
        char,
        vec![
            PrimeField::new(char, 0),
            PrimeField::new(char, 16),
            PrimeField::new(char, 3),
            PrimeField::new(char, 7),
            PrimeField::new(char, 5),
            PrimeField::new(char, 8),
            PrimeField::new(char, 14),
            PrimeField::new(char, 1),
        ],
    );
    println!("送信したい文章: {:?}", origin_sentense.toVec());

	// 各数値の計算
    let length = (origin_sentense.toVec().len()) as u16;
	let n = &char - 1; // 符号込の長さ
	let k = &length; // 文章の長さ
	let d = n - k + 1; // 最小距離
	let t = (n - k) / 2; // エラーを訂正できる数

	let l_0 = &n - 1 - &t;
	let l_1 = &n - 1 - &t - (k - 1);
	println!("[{},{},{}]-_{}code", n, k, d, char);

	let mut P = Vec::new();
	// 原始根を１つ固定
	let primitive_element: i64 = 3;

	// 原始根を生成
	for i in 0..char - 1 {
		P.push(PrimeField::new(char, primitive_element.pow(i.into())));
	}
	P = FiniteField::new(char, P).elements;
	// 符号化
	let u = reed_solomon_encode(&P, origin_sentense, &char);
	println!("送信語:{:?}", u.toVec());

	// 送信でエラーを起こす
	let mut u_received = u.toVec();
	let mut rng = rand::thread_rng();
	let error_count = rng.gen_range(1, 10);
	println!("エラーの数:{}", error_count);

	for _i in 0..error_count {
		let error_position = rng.gen_range(0, n);
		let error_value = rng.gen_range(0, char);
		u_received[error_position as usize] = error_value;
	}

	// 受信語
	let u_received = FiniteField::new(
		char,
		u_received
			.into_iter()
			.map(|x| PrimeField::new(char, x as i64))
			.collect(),
	);
	println!("受信語: {:?}", u_received.toVec());

	// シンドローム
	/*
	let mut S:Vec<PrimeField> = Vec::new();
	for i in 1..n-k{
	S.push(function(&P[i as usize], &u_received, &char, &n));
}
	println!("S:{:?}",S);
	 */

	// 復号行列の作成
	let mut H: Vec<Vec<PrimeField>> = Vec::new();
	for i in 0..n {
		let mut tmp: Vec<PrimeField> = Vec::new();
		for j in 0..l_0 + 1 {
			tmp.push(P[i as usize].pow(j.into()));
		}
		for j in 0..l_1 + 1 {
			tmp.push(u_received.elements[i as usize].mul(&P[i as usize].pow(j.into())));
		}
		H.push(tmp);
	}

	// 掃き出し法
	H = sweep_method(H);
	

	// 行列のランク
	let mut rank = 0;
	for i in 0..n {
		if H[i as usize][i as usize].num != 0 {
			rank += 1;
		}
	}

	// 非零解の１つを求める
	let mut Q: Vec<PrimeField> = Vec::new();
	for i in 0..rank {
		let temp = H[i as usize][rank..].to_vec();
		let mut answer = PrimeField::new(char, 0);
		for j in 0..temp.len() {
			answer = answer.sub(&temp[j as usize].mul(&PrimeField::new(char, 1)));
		}
		Q.push(answer);
	}
	for _i in rank..(l_0 + l_1 + 2) as usize {
		Q.push(PrimeField::new(char, 1));
	}

	let Q_num: Vec<PrimeField> = Q.iter().map(|x| PrimeField::new(char, x.num)).collect();

	// 誤り位置多項式を求める

	// 次数の調整
	let mut Q0_temp = Q_num[0..(l_0 + 1) as usize]
        .to_vec()
        .into_iter()
        .rev()
        .collect::<Vec<PrimeField>>();
    for _i in 0..(l_0 + 1) as usize {
        if Q0_temp[0].num == 0 {
            Q0_temp.remove(0);
        } else {
            break;
        }
    }

    let mut Q1_temp = Q_num[(l_0 + 1) as usize..(l_0 + l_1 + 2) as usize]
        .to_vec()
        .into_iter()
        .rev()
        .collect::<Vec<PrimeField>>();
    for _i in 0..(l_1 + 1) as usize {
        if Q1_temp[0].num == 0 {
            Q1_temp.remove(0);
        } else {
            break;
        }
    }

    let mut Q0 = FiniteField::new(char,  Q0_temp);
    let Q1 = FiniteField::new(
        char,
        Q_num[(l_0 + 1) as usize..(l_0 + l_1 + 2) as usize]
            .to_vec()
            .into_iter()
            .rev()
            .collect::<Vec<PrimeField>>(),
    );

    // Q0をQ1で割った商を求める
    let mut quotient: Vec<PrimeField> = Vec::new();

    for i in 0..Q0.elements.len() - Q1.elements.len() + 1 {
        let tmp = Q0.elements[i].div(&Q1.elements[0]);
        for j in 0..Q1.elements.len() {
            Q0.elements[i + j] = Q0.elements[i + j].sub(&Q1.elements[j].mul(&tmp));
        }

        quotient.push(tmp);
    }

    // マイナスにする
    for i in 0..quotient.len() {
        quotient[i] = quotient[i].mul(&PrimeField::new(char, -1));
    }

    // 逆順にする
    let quotient = quotient.into_iter().rev().collect::<Vec<PrimeField>>();
    let quotient: FiniteField = FiniteField::new(char,  quotient);

    let Q0_vec = Q0.toVec();
    let mut Q0_remainder = 0;
    for i in 0..Q0_vec.len() {
        Q0_remainder += Q0_vec[i];
    }

    if Q0_remainder != 0 {
        println!("復号不可能");
        std::process::exit(0);
    }
    let mut decode_code: Vec<PrimeField> = Vec::new();
    for i in 0..n {
        let tmp = function(
            &P[i as usize],
            &quotient,
            &char,
        );
        decode_code.push(tmp);
    }

    // 結果を表示
    let decode_code_num: Vec<u16> = decode_code.iter().map(|x| x.num as u16).collect();
    println!("復号:{:?}", decode_code_num);

    if decode_code_num == u.toVec() {
        println!("正解");
    } else {
        println!("不正解");
    }
}
