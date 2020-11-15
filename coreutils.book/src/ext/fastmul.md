# 附录：快速乘算法

对于整数乘法，一种朴素的方法是：

$$
a * b = \underbrace{a + a + \cdots + a}_{b \text{个} a}
$$

有一种简化方法可以将时间复杂度从 \\(O(n)\\) 降低到 \\(O(\log{n})\\)

$$
\def\mul{\operatorname{mul}}
\mul(a,b) = \begin{cases}
    \mul(2a, \frac{b}{2}) & b \text{是偶数} \\\\
    \mul(2a, \lfloor\frac{b}{2}\rfloor) + a & b \text{是奇数} \\\\
\end{cases}
$$

## 递归实现

```rust,ignore
{{#include ../../../fastcalc/src/lib.rs:fastmul}}
```

## 循环实现

```rust,ignore
{{#include ../../../fastcalc/src/lib.rs:fastmul_norec}}
```
