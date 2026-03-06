---
title: Cruzando fronteiras com FFI
author: Gabriel Saiago
theme:
  name: gruvbox-dark
  override:
    code:
      theme_name:  base16-eighties.dark
      alignment: left
      minmum_size: 90
    intro_slide:
      title:
        alignment: center
      footer: false
    footer:
      style: template
      center: "{title}"
      right: "{current_slide} / {total_slides}"
    palette:
      classes:
        text_highlight:
          background: "fabd2f"
          foreground: "282828"
---

\> O que é?
===

Chamar uma função que foi implementada numa linguagem X dentro de uma linguagem Y
<!-- pause -->

```c
void manda_um_oi() {
  printf("oi de dentro do C");
}
```

<!-- pause -->

O objetivo é chamar em:

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->
## Rust
```rust
fn main() {
  manda_um_oi();
}
```

## JavaScript
```js
function main() {
  manda_um_oi()
}
```

<!-- column: 1-->
## Golang
```go
func main() {
  manda_um_oi()
}
```

## Python
```python
def main():
  manda_um_oi()
```
<!-- end_slide -->

\> Mas antes, uma palavra dos nossos ~patrocinadores~ conceitos
==
<!-- pause -->

# ABI (Abstract Binary Interface) #
<!-- pause -->

A ABI define as '_regras do jogo_', ela '_amarra_' as regras do S.O,
da CPU e da implementação das especifições de uma linguagem.

> [!IMPORTANT]
> O Compilador escolhe a ABI de acordo com o target da compilação. Por ex, para x86-64, o seu compilador provavelmente vai seguir a System-V ABI

<!-- pause -->

## Diferentes linguagens representam estruturas de formas diferentes ##

<!-- pause -->

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

Struct em `C`

```c
/// #include<stdio.h>
/// #include<stdint.h>
typedef struct s_StructEmC {
  uint32_t  var1;
  uint16_t  var2;
  uint32_t  var3;
  uint16_t  var4;
} StructEmC;
/// int main(void) {
///   printf("O tamanho da struct em c é %ld\n", sizeof(StructEmC));
/// }
```
<!-- pause -->


| Offset | Bytes |    Campo    |
| :----: | :---: | :---------: |
|   0    |   4   | `var1: 32_t`|
|   4    |   2   | `var2: 16_t`|
|   6    |   2   | `[padding]` |
|   8    |   4   | `var3: 32_t`|
|   12   |   2   | `var4: 16_t`|
|   14   |   2   | `[padding]` |

<!-- pause -->

<!-- column: 1 -->
Struct em `Rust`

```rust
# #[allow(dead_code)]
pub struct StructEmRust {
  var1: u32,
  var2: u16,
  var3: u32,
  var4: u16,
}
# fn main() {
#   println!("O tamanho da struct em rust é {}", std::mem::size_of::<StructEmRust>());
# }
```

<!-- pause -->

| Offset | Tamanho  |   Campo     |
| :----: | :------: | :---------: |
| 0      | 4        | `var1: u32` |
| 4      | 4        | `var3: u32` |
| 8      | 2        | `var2: u16` |
| 10     | 2        | `var4: u16` |

<!-- end_slide -->

\> Mas antes, uma palavra dos nossos ~patrocinadores~ conceitos
===

# Name Mangling #

O compilador não pode definir duas funções com o mesmo nome!

> "Claro que não, Saiago!
> C++ tem overload de função e compila"


<!-- pause -->

## Exemplos ##

<!-- pause -->

Uma função com essas assinaturas:

```c++
int func_que_recebe(char input);
int func_que_recebe(std::string input);
int func_que_recebe(char *input);
```

<!-- pause -->

vira isso aqui na compilação:

<span class="text_highlight">i</span>func_que_recebe<span class="text_highlight">c</span>\
<span class="text_highlight">i</span>func_que_recebe<span class="text_highlight">str</span>\
<span class="text_highlight">i</span>func_que_recebe<span class="text_highlight">cptr</span>

* Não vira exatamente isso, a especificação real você pode ver no [source code do GCC](https://github.com/gcc-mirror/gcc/blob/master/gcc/cp/mangle.cc)

<!-- end_slide -->


\> Mão na massa
===

<!-- pause -->
