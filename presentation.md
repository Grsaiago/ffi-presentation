---
title: Cruzando fronteiras com FFI
author: Gabriel Saiago
options:
  implicit_slide_ends: true
theme:
  override:
    code:
      alignment: left
---

O que é?
---

Chamar uma função que foi implementada numa linguagem X dentro de uma linguagem Y
<!-- pause -->

```c
// C
void manda_um_oi() {
  printf("oi de dentro do C");
}
```

<!-- pause -->

O objetivo é chamar em:

<!-- column_layout: [1, 1]-->

<!-- column: 0-->
```rust
// Rust
fn main() {
  manda_um_oi();
}
```

```js
// JavaScript
function main() {
  manda_um_oi()
}
```

<!-- column: 1-->
```go
// Golang
fn main() {
  manda_um_oi()
}
```

```python
# Python
def main():
  manda_um_oi()
```
<!-- end_slide -->

Mas antes, uma palavra dos nossos ~patrocinadores~ conceitos
---
<!-- pause -->

# ABI (Abstract Binary Interface) #
A ABI define as '_regras do jogo_', seja pra uma linguagem ou pra um sistema operacional.
<!-- column_layout: [1, 1]-->

# Calling Convention #
<!-- end_slide -->
