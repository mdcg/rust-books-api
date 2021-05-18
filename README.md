# rust-books-api
:books: A simple Books CRUD API written in Rust

## Rust Analyzer

- [https://rust-analyzer.github.io/](https://rust-analyzer.github.io/)

`rust-analyzer` é uma implementação do **Language Server Protocol** para a linguagem de programação Rust. Ele fornece recursos como conclusão e definição de goto para muitos editores de código, incluindo VS Code, Emacs e Vim.

## Configurando Banco de Dados

- [https://diesel.rs/](https://diesel.rs/)

```
cargo install diesel_cli --no-default-features --features postgres
```

*PS: Se ocorrer algum problema: `sudo apt install libpq-dev`*

Certifique-se de definir as variáveis ​​de ambiente corretamente no arquivo `.env` (use` .env-example` como base). Depois disso, execute o seguinte comando:

```
diesel setup
```

Para criar nosso schema inicial

```
diesel migration generate create_books
```

Após esse passo, precisamos adicionar a criação do nosso banco nos arquivos `migrations/<datetime>_create_books/down.sql` e `up.sql`.

Agora basta rodar as migrations:

```
diesel migration run
diesel migration redo
```

Precisamos gerar agora o schema do banco de dados dentro do arquivo `schema.rs`. Para isso, executamos o seguinte comando:

```
diesel print-schema > src/schema.rs
```

Esse comando irá gerar uma macro para nós.

A depender do tamanho do banco de dados, talvez seja melhor utilizar o `infer_schema!("dotenv:DATABASE_URL");` dentro de `schema.rs`. Mas para esse projeto, vamos ficar com a definição de cima.


## Referências

- [https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)
- [https://users.rust-lang.org/t/cannot-find-value-t-in-this-scope/46739/9](https://users.rust-lang.org/t/cannot-find-value-t-in-this-scope/46739/9)