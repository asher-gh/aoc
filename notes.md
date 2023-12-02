# Day 4

So I think, this problem will benefit from matrix logic.

I found an [excellent video](https://www.youtube.com/watch?v=XkY2DOUCWMU) by
Grant Sanderson

## Matrix multiplication

If $\mathbf{A}$ is an $m \times n$ matrix and $\mathbf{B}$ is an $n \times p$
matrix,

$$
\begin{aligned}
A =
\begin{bmatrix}
a_{11}	&   a_{11}  &	\dots	&   a_{1n} \\
a_{21}	&   a_{21}  &	\dots	&   a_{2p} \\
\vdots	&   \vdots  &	\ddots	&   \vdots \\
a_{m1}	&   a_{m2}  &	\dots	&   a_{mn} \\
\end{bmatrix}, & &
B =
\begin{bmatrix}
b_{11}	&   b_{12}  &	\dots	&   b_{1p} \\
b_{21}	&   b_{21}  &	\dots	&   b_{2p} \\
\vdots	&   \vdots  &	\ddots	&   \vdots \\
b_{n1}	&   b_{n2}  &	\dots	&   b_{np} \\
\end{bmatrix}
\end{aligned}
$$

the matrix product $\mathbf{C = AB}$ (denoted _without_ multiplication signs or
dots) is defined to be the $m \times p$ matrix. mat

$$
C =
\begin{bmatrix}
c_{11} & c_{12} & \dots & c_{1p} \\
c_{22} & c_{22} & \dots & c_{2p} \\
\vdots	&   \vdots  &	\ddots	&   \vdots \\
c_{m1}	&   c_{m2}  &	\dots	&   c_{mp} \\
\end{bmatrix}
$$

such that

$c_{ij} = a_{i1}b_{1j} + a_{i2}b_{2j} + \dots + a_{in}b_{nj} = \displaystyle\sum_{k=0}^n a_{ik}b{kj}$

for $i = 1, \dots, m$ and $j = 1, \dots, p$.

# day 6

The first part was simple. However, the second part used almost all of my RAM.
So instead of adding to the vector, I'll just keep track of the counts of each
number 1-8 and in each cycle I'll copy next to current.
