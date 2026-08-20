# rust-investment-wallet

Carteira de investimentos fullstack desenvolvida em Rust durante o **Santander Bootcamp Rust AI Developer**, da DIO.

A aplicação permite que usuários se cadastrem, façam login e registrem compras de ativos financeiros. O dashboard exibe os ativos adquiridos, o histórico de compras e a variação de valor de cada ativo.

## Funcionalidades

- Cadastro e autenticação de usuários com senha hash e JWT
- Dashboard de ativos com histórico de compras por ativo
- Registro de compras vinculadas ao usuário autenticado
- Cálculo de variação de valor (delta) entre o preço de compra e o valor atual do ativo
- API administrativa para criação e atualização de ativos
- Logout com limpeza de cookie de autenticação

## Tecnologias utilizadas

| Tecnologia | Finalidade |
|---|---|
| Rust (edition 2024) | Linguagem principal |
| Axum | Framework web (rotas, extractors, estado compartilhado) |
| Askama | Engine de templates HTML |
| Tokio | Runtime assíncrono |
| jwt-simple | Geração e validação de tokens JWT (HS256) |
| password-auth | Hash e verificação de senhas |
| axum-extra | Gerenciamento de cookies |
| serde / serde_json | Serialização e desserialização de dados |
| time | Manipulação de datas e formatação |
| tracing / tracing-subscriber | Logs e instrumentação |
| color-eyre / thiserror | Tratamento de erros |
| TailwindCSS (CDN) | Estilização do frontend |

## Como executar

Pré-requisitos: [Rust](https://www.rust-lang.org/tools/install) instalado.

```bash
git clone https://github.com/seu-usuario/rust-investment-wallet.git
cd rust-investment-wallet
cargo run
```

A aplicação será iniciada em `http://localhost:3000`.

Para criar ativos disponíveis para compra, utilize a API administrativa:

```bash
curl -X POST http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -H "Authorization: im-the-admin" \
  -d '{"name": "Bitcoin", "unit_value": 350000.00}'
```

## Melhorias implementadas

As seguintes melhorias foram implementadas como parte da evolução solicitada pela atividade:

### 1. Separação de login e cadastro

Na versão original, login e cadastro compartilhavam o mesmo formulário e endpoint. Se o usuário não existisse, o cadastro era feito automaticamente durante o login.

Na versão atual, os dois fluxos foram separados em páginas e rotas distintas:

- `GET /login` — exibe o formulário de login
- `POST /login` — realiza somente a autenticação
- `GET /register` — exibe o formulário de cadastro
- `POST /register` — realiza somente o cadastro de novo usuário

A página de login possui um link para a página de cadastro, e a página de cadastro possui um link de volta para o login. A lógica de autenticação, hash de senha e geração de JWT permanece inalterada.

### 2. Botão de logout no dashboard

O logout já existia na aplicação por meio da rota `GET /logout`. A melhoria consistiu em reposicionar o botão de logout para o header do dashboard (`/assets`), ao lado da mensagem de boas-vindas, tornando a ação de sair da conta visível e acessível ao usuário autenticado.

## Como testar as melhorias

Após iniciar a aplicação com `cargo run`:

1. Acesse `http://localhost:3000` — você será redirecionado para `/login`
2. Na página de login, clique em **create account** para ir à página de cadastro
3. Na página de cadastro (`/register`), crie um usuário com username e senha
4. Após o cadastro, você será redirecionado automaticamente para o dashboard (`/assets`)
5. No header do dashboard, verifique que o botão **logout** está visível ao lado de "welcome, [username]"
6. Clique em **logout** — você será redirecionado de volta para `/login`
7. Faça login com o usuário que acabou de criar para confirmar que a autenticação funciona corretamente

## Validação do código

```bash
cargo fmt       # formatação
cargo clippy    # análise estática
cargo build     # compilação
cargo test      # execução de testes (o projeto não possui testes automatizados)
```

Todos os comandos acima executam sem erros ou warnings.

## O que foi aprendido

- Construção de uma aplicação web fullstack utilizando Rust com Axum como framework
- Utilização de templates HTML com Askama para renderização server-side
- Implementação de autenticação baseada em JWT com cookies `HttpOnly`
- Hash seguro de senhas com a crate `password-auth`
- Gerenciamento de estado compartilhado com `Arc<Mutex<>>` no Axum
- Separação de rotas frontend (HTML) e API (JSON) na mesma aplicação
- Uso de extractors customizados do Axum para autenticação e acesso ao repositório
- Refatoração de fluxos de autenticação para separar responsabilidades entre login e cadastro
