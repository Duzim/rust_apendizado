# [Recursos de Linguagem Funcional: Iteradores e Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html#functional-language-features-iterators-and-closures)

## [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

As *`closures`* do Rust são funções anônimas que você pode armazenar em uma variável ou passar como argumentos para outras funções. Você pode criar a *`closure`* em um ponto e, posteriormente, invocá-la em outro local para executá-la em um contexto diferente. Ao contrário das funções, as *`closures`* podem capturar valores do escopo em que foram definidas. Demonstraremos como essas características das *`closures`* possibilitam a reutilização de código e a personalização de comportamento.