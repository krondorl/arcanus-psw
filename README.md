# Arcanus Psw

Generate strong passwords, which you can remember.

## Motivation

The aim of this project is to deepen my Rust language skills.

## Features

- Entropy check
- File handling
- Handle newlines for Windows, Linux and Mac
- Written in the Rust programming language
- Unit tests

## Technical Requirements

`Rust >= v1.74.1`

## How to Use

First, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your computer.

Open a terminal:

- Windows: run `Windows Terminal`, `cmd`, or `PowerShell`.
- Mac: run `Terminal`.
- Linux: run `Terminal`.

In the project root folder, run: `cargo run`

## Specification

- Vowels: `a, e, i, o, u`
- Consonants: `b, c, d, f, g, h, j, k, l, m, n, p, q, r, s, t, v, w, x, y, z`
- Numbers: `0-9`
- Special characters: `!, +, \, #, /, $, ?`
- Length: 16-64 characters
- Combination of vowels and consonants
- Lowercase and uppercase

## Examples

```
WukovianeMako18!
MeriKunoMata32#?
XuniWabeTim1928!
FaweXanaToboYisoXiyoSareDahaY08/
```

## Research

I used ideas from:

- [Irma Šlekytė: Password entropy: Definition and formula](https://nordvpn.com/blog/what-is-password-entropy/)
  (Last access: January 1st, 2024.)
- [How to Calculate Password Entropy?](https://generatepasswords.org/how-to-calculate-entropy/) (Last access: January 1st, 2024.)
- [Aranza Trevino: Top Five Password Security Tips](https://www.keepersecurity.com/blog/2023/07/18/top-five-password-security-tips/) (Last access: December 25th, 2023.)
- [Neil J. Rubenking: 3 Simple Tricks for Remembering Strong Passwords](https://www.pcmag.com/how-to/tricks-for-remembering-strong-passwords) (Last access: December 25th, 2023.)

## License

Please see [LICENSE file](LICENSE).

## History

I started the project on 25th December, 2023.
