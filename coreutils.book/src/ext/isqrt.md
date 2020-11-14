# 附录：整数开方

> 参考原文：<http://www.nuprl.org/MathLibrary/integer_sqrt/>

对任意自然数 \\(x\in \mathbb{N}\\)，设它的整数平方根为 \\(r = \lfloor\sqrt{x}\rfloor\\)，则必然有 \\(r^2 \le x < (r + 1)^2\\) 。并且，有如下关系：

$$
\lfloor\sqrt{x+1}\rfloor = \begin{cases}
    \lfloor\sqrt{x}\rfloor
        & x+1 < (r+1)^2 \\\\
    \lfloor\sqrt{x}\rfloor + 1
        & x+1 \ge (r+1)^2
\end{cases}
$$

因此，就可以通过检查 `isqrt(x - 1)` 的值来求得 `isqrt(x)` 的值：

```rust,editable
fn isqrt(x: u64) -> u64 {
    match x {
        0 => 0,
        x => {
            let r = isqrt(x - 1);
            let rr = r + 1;
            if x < rr * rr {
                return r;
            } else {
                return rr;
            }
        }
    }
}

fn main() {
    println!("{}", isqrt(17));
}
```

只不过这种方法并不是最有效的，它要求得 \\(\sqrt{x}\\) ，就需要先求 \\(\sqrt{x-1}\\)，再之前是 \\(\sqrt{x-2}\\) 直到 \\(\sqrt{0}\\)。也就是说，时间复杂度为 \\(O(x)\\)。

通过数学上的「完全归纳法」，可以证明：

> 对任意自然数 \\(x, y \in \mathbb{N}\\)，如果有 \\(P(y) \Rightarrow P(x), y < x\\)，那么只要 \\(P(0)\\) 成立，则对所有自然数，都有 \\(P(x)\\) 成立。

而上面的简化法就是此完全归纳法在 \\(y = x - 1\\) 的特例。那么我们需要的新的 \\(y = f(x)\\) 关系应当是什么？为了简化算法，我们一般需要通过除法来减小操作数，设除数为 \\(b, b > 1\\)，则有：

$$
\operatorname{isqrt}(x) \sim \operatorname{isqrt}(\frac{x}{b})
$$

假设，存在 \\(r_0^2 \le \frac{x}{b} < (r_0 + 1)^2\\), \\(r^2 \le x < (r + 1)^2\\)，则有

$$
r^2 = br_0^2, (r+1)^2 < b(r_0 + 1)^2
$$

有

$$
r = \sqrt{b} r_0
$$

这样，就可取递归参数 \\(b = 4\\)，使得数值可以成倍地减少（二进制优化）：

```rust,editable
{{#include ../../../fastcalc/src/lib.rs:isqrt}}

fn main() {
    println!("{}", isqrt(100));
}
```

## 递归实现

```rust,ignore
{{#include ../../../fastcalc/src/lib.rs:isqrt}}
```

# n 次方

同理，对 \\(x^{\frac{1}{n}}\\) 也有

$$
r^n = b r_0^n \Rightarrow r = \sqrt[n]{b} r_0
$$

取 \\(b = 2^n\\)。

```rust,editable
{{#include ../../../fastcalc/src/lib.rs:iroot}}

fn main() {
    println!("{}", iroot(1024, 3));
}
```