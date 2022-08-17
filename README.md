# autodiff

Simple automatic differentiation.

## Roadmap

```
Graph
    Nodes
        * arithmetic ops: add, mul, sub, div
        * functions app:  neg, pow, (sin,cos,...), exp, ...
    Edges (u, v)
        * v = f(u)
```

Example:
```
y = 3x^2 + 4x - 2

sub(
    add(
        mul(3, pow(x, 2)),
        mul(4, x)
    ),
    2
)
```

To simplify, we'll assume each arithmetic node has 2 inputs.

* arithmetic ops:
```
d[add(l, r)] := add(d[l], d[r])
d[sub(l, r)] := sub(d[l], neg(d[r]))
d[mul(l, r)] := add(mul(d[l], r), mul(l, d[r]))
d[div(l, r)] := div(sub(mul(d[l], r), mul(l, d[r])), pow(r, 2))
```

* functions:
```
v = f(u); dv/du = d[v, u]

d[const]     := 0
d[neg(u)]    := neg(d[u])
d[pow(u, n)] := mul(n, pow(u, sub(n, 1)))
d[sin(u)]    := cos(u)
d[exp(u)]    := exp(u)
...
```

Worked example:
```
tanh(x) :=
    z1 = -2x
    z2 = exp(z1)
    z3 = (1-z2)/(1+z2)

z1 = mul(-2, x)
z2 = exp(z1)
z3 = div(
    l: sub(1, z2),
    r: add(1, z2)
)

tanh(x) = z3(z2(z1(x)))

d[tanh, x] = Chain[d[z3, z2], d[z2, z1], d[z1, x]]

d[z3, z2] =
    div(
        sub(
            mul(       -1 , add(1, z2) ),
            mul( sub(1, z2), 1         )
        ),
        pow(add(1, z2), 2)
    )
d[z2, z1] = exp(z1) = z2
d[z1, x]  = -2

d[tanh, x] =
    mul(
        mul(
            div(
                sub(
                    mul(-1, add(1, z2)),
                    mul(sub(1, z2), 1)
                ),
                pow(add(1, z2), 2)
            ),
            z2
        ),
        -2
    )
^-- eval

4z2 / (1+z2)^2 =
4exp(-2x) / (1+exp(-2x))^2
```
