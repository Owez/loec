# loec

Count and estimate the cost of lines of code (cloec) fast.

## Why this?

This repository is a fork of the now-abandoned but fantastic [`loc`](https://github.com/cgag/loc) crate with extra [COCOMO](https://en.wikipedia.org/wiki/COCOMO) estimation features built on top, which now allows this program to rival [`scc`](https://github.com/boyter/scc) in usability.

The only real reason to use this fork over the aforementioned `scc` is this is written in Rust and therefore ships as a static (a.k.a. single) binary instead of shipping with any real dependencies.

## Installation

- [**Download**]((https://github.com/owez/loec/releases))

Currently, the releases page linked above is the only way to download this package.

## Usage

By default, `loec` will count lines of code in a target directory:

``` shell
$ loec
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Lua                      2       387088        24193       193544       169351
 Rust                     4         1172          111           31         1030
 C                        4          700           75          155          470
 Markdown                 2          249           39            0          210
 Bourne Shell             4          228           41           27          160
 Ada                      2           53           12            9           32
 Toml                     1           26            4            2           20
 Gherkin                  1           12            2            2            8
 OCaml                    1           13            4            6            3
 Ruby                     1            4            0            2            2
 Handlebars               1            4            0            2            2
--------------------------------------------------------------------------------
 Total                   23       389549        24481       193780       171288
--------------------------------------------------------------------------------

```

You can also pass one or many targets for it to inspect

``` shell
$ loec ci benches
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Bourne Shell             4          228           41           27          160
 Rust                     1           17            4            0           13
--------------------------------------------------------------------------------
 Total                    5          245           45           27          173
--------------------------------------------------------------------------------
```

To see stats for *each file* parsed, pass the `--files` flag:

```sh
$ loec --files src
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
 Rust                     2         1028           88           29          911
--------------------------------------------------------------------------------
|src/lib.rs                         677           54           19          604
|src/main.rs                        351           34           10          307
```

By default, the columns will be sorted by `Code` counted in descending order. You can select a different column to sort
using the `--sort` flag:

``` shell
$ loec --files --sort Comment ci
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
 Bourne Shell             4          228           41           27          160
--------------------------------------------------------------------------------
|ci/before_deploy.sh                 68           15           13           40
|ci/install.sh                       60           13            6           41
|ci/script.sh                        41            8            8           25
|ci/utils.sh                         59            5            0           54

```

`loec` can also be called with regexes to match and/or exclude files.

``` shell
$ loec --include 'count'
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Rust                     2          144           23            2          119
--------------------------------------------------------------------------------
 Total                    2          144           23            2          119
```

``` shell
loec --exclude 'sh$'
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Lua                      2       387088        24193       193544       169351
 Rust                     4         1172          111           31         1030
 C                        4          700           75          155          470
 Markdown                 2          275           38            0          237
 Ada                      2           53           12            9           32
 Toml                     1           26            4            2           20
 Gherkin                  1           12            2            2            8
 OCaml                    1           13            4            6            3
 Handlebars               1            4            0            2            2
 Ruby                     1            4            0            2            2
--------------------------------------------------------------------------------
 Total                   19       389347        24439       193753       171155
--------------------------------------------------------------------------------
```


## Known Issues
Fortran has a rule that comments must start with the first character of a line. I only check if it's the first non-whitespace character of a line. I don't know
how often this is a problem in real code.  I would think not often.

Comments inside string literals: You can get incorrect counts if your code has something like this:

```
x = "/* I haven't slept \
for 10 days \
because that would be too long \
*/";
```

loec counts the first line and last lines correctly as code, but the middle
lines will be incorrectly counted as comments.

Ignored and hidden files:

By default, loec respects .gitignore/.ignore files, and ignores hidden files and directories.  You can count disregard
ignore files with `loec -u`, and include hidden files/dirs with `loec -uu`.

## Supported Languages

- ActionScript
- Ada
- Agda
- AmbientTalk
- ASP
- ASP.NET
- Assembly
- Autoconf
- Awk
- Batch
- Bourne Shell
- C
- C Shell
- C/C++ Header
- C#
- C++
- Clojure
- CoffeeScript
- ColdFusion
- ColdFusionScript
- Coq
- CSS
- CUDA
- CUDA Header
- D
- Dart
- DeviceTree
- Erlang
- Forth
- FORTRAN Legacy
- FORTRAN Modern
- F# (Fsharp)
- GLSL
- Go
- Groovy
- Handlebars
- Haskell
- Hex
- HTML
- Idris
- INI
- Intel Hex
- Isabelle
- Jai
- Java
- JavaScript
- JSON
- Jsx
- Julia
- Kotlin
- Lean
- Less
- LinkerScript
- Lisp
- Lua
- Make
- Makefile
- Markdown
- Mustache
- Nim
- Nix
- Objective-C
- Objective-C++
- OCaml
- OpenCL
- Oz
- Pascal
- Perl
- PHP
- Plain Text
- Polly
- PowerShell
- Prolog
- Protobuf
- Pyret
- Python
- Qcl
- QML
- R
- Razor
- reStructuredText
- Ruby
- RubyHtml
- Rust
- SaltStack
- Sass
- Scala
- SML
- Solidity
- SQL
- Stylus
- Swift
- Tcl
- Terraform
- TeX
- Toml
- TypeScript
- Tsx
- UnrealScript
- VimL
- Wolfram
- XML
- Yacc
- YAML
- Zig
- Z Shell

## Attributions

This project contains code from [Tokei](https://github.com/Aaronepower/tokei) by [Aaronepower](https://github.com/Aaronepower), [ripgrep](https://github.com/BurntSushi/ripgrep) by [BurntSushi](https://github.com/BurntSushi) and [scc](https://github.com/boyter/scc/) by [boyter](https://github.com/boyter/).
