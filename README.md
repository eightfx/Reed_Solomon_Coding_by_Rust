
# Table of Contents

1.  [これは何？](#orgc9c9c91)
2.  [使い方](#orgf6488bf)
3.  [使用しているクレート](#orge6743af)
4.  [理論的背景](#org34dd450)


<a id="orgc9c9c91"></a>

# これは何？

RustでReed-Solomon符号を実装してみました。
Reed-Solomon符号は、データの送受信時にエラーが発生した場合に、データを復元するための符号です。


<a id="orgf6488bf"></a>

# 使い方

1.  まずRustをインストールします。

2.  このリポジトリをクローンします。

3.  クローンしたディレクトリに移動して、cargo run &#x2013;releaseを実行します。

4.  以下のような出力が得られます。ただし、エラーはランダムに発生するので、実行するたびに出力が異なります。

成功した場合:
以下、66という文章を符号化してsendにして送信しました。
エラーが発生してreceivedになりましたが、decodeではsendと同じ文字列に復号できました。

    P:326451
    t:2
    l_0 = 3
    l_1 = 2
    [6,2,5]-_7code
    sentence: "66"
    send: "340215"
    received: "342615"
    Q: "3051341"
    Q0: "3051"
    Q1: "341"
    quotient: "11"
    remainder: "0"
    distance: 2
    decode: "340215"
    成功

失敗した場合:

    P:326451
    t:2
    l_0 = 3
    l_1 = 2
    [6,2,5]-_7code
    sentence: "41"
    send: "063125"
    received: "062423"
    Q: "5204521"
    Q0: "5204"
    Q1: "521"
    quotient: "64"
    remainder: "35"
    distance: 5
    復元不可能


<a id="orge6743af"></a>

# 使用しているクレート

自作したgalois<sub>fieldクレートを使用しています</sub>。
詳細はこちら
<https://crates.io/crates/galois_field>


<a id="org34dd450"></a>

# 理論的背景

$$ P = \{\alpha_1, \ldots, \alpha_n\} \subset \mathbb{F}_q $$

$$ L(m) : \mathbb{F}_q を係数とする最大m次の1変数多項式からなる線形空間とする。$$

$$ ただし、実務上はPを  \mathbb{F}_q  の原始根の集合(i.e. n = q - 1 )として扱うことが多い。$$

$$ n > mとする。$$

$$ Ev :  L(m) \rightarrow L(n) : f \longmapsto (f(\alpha_1), \ldots , f(\alpha_n)) とする。$$

$$ この写像は単射で、像は次数mのReed-Solomon codeと呼ばれる  [n,m+1,n-m]_q -code となる。$$

$$ 例えば、P = \{ x_1, \ldots, x_n \} として、送信したい文章を (r_0, \ldots, r_m) とする。$$

$$ u(x) = \sum r_i \cdot x^i とすると、文章は(u(x_1), \ldots, u(x_n))として符号化される。$$

