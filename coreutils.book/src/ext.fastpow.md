# 附录：快速幂算法

对整数求幂，一种朴素的想法就是：

$$
\def\pow{\operatorname{pow}}
\pow(b,e) =b^e = \underbrace{b \cdot b \cdot b \cdots b}_{n \text{个} b}
$$

假设一次乘法运算的时间复杂度为 $O(1)$，则一次求幂运算的时间复杂度为 $O(e)$。

但是，可以用一个递归方法简化：

$$
\def\pow{\operatorname{pow}}
\pow(b, e) = \begin{cases}
    \pow(b^2, \frac{e}{2}) & e \ \text{是偶数} \\
    \pow(b^2,
        \lfloor\frac{e}{2}\rfloor)
        \cdot b & e \ \text{是奇数} \\
\end{cases}
$$

每递归一次，指数缩小一半，因此时间复杂度变为 $O(\log n)$。这种算法就被称为 「快速幂」。

## 递归实现

```rust,ignore
{{#include ../../fastcalc/src/lib.rs:fastpow}}
```

## 循环实现

```rust,ignore
{{#include ../../fastcalc/src/lib.rs:fastpow_norec}}
```