# [Tratamento de erros](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

Rust agrupa erros em duas categorias principais: 
- **Recuperáveis**,  como um arquivo não encontrado, do qual só queremos relatar o problema ao usuário e tentar novamente a operação.
- **Irrecuperáveis**, são sempre sintomas de bugs, como tentar acessar um localização além do final de uma matriz e, portanto, queremos interromper imediatamente o programa.

## [Erros irrecuperáveis com panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)

A macro `panic!` para a execução do código, fazendo o programa parar de rodar.

> [Desenrolando a pilha ou abortando em resposta a um pânico](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic): Ao causar um pânico o rust limpa a `stack`, mas permite que não faça isso. Com a seguinte configuração no `Cargo.toml`. 
```toml
[profile.release]
panic = 'abort'
```

## [Erros recuperáveis com Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)