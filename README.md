# Gerador de Senhas

Um gerador de senhas seguro e personalizável, escrito em Rust. Este projeto permite que você gere senhas aleatórias com diferentes combinações de caracteres e as armazene, visualize ou remova conforme necessário.

## Recursos

- Geração de senhas com opções para incluir:
- Letras maiúsculas
- Números
- Caracteres especiais
- Armazenamento seguro de senhas em um arquivo
- Listagem de senhas armazenadas
- Remoção de senhas armazenadas
- Exportação de senhas para um arquivo
- Armazenamento de senhas utilizando hash para maior segurança

## Instalação

Para executar este projeto, você precisará do Rust e do Cargo instalados em seu sistema. Você pode instalar o Rust seguindo as instruções em [rustup.rs](https://rustup.rs/).

## Em seguida, compile e execute o projeto:
```sh
cargo run
```
## Uso
O gerador de senhas aceita os seguintes argumentos:

```sh
cargo run -- [OPÇÕES]
```
## Opções

``-t <TAMANHO>, --tamanho <TAMANHO>``: Define o tamanho da senha a ser gerada (padrão: 16).

``-m, --maiusculas``: Inclui letras maiúsculas na senha gerada.

``-n, --numeros``: Inclui números na senha gerada.

``-e, --especiais``: Inclui caracteres especiais na senha gerada.

``-l, --listar``: Lista as senhas armazenadas.

``-r <SENHA>, --remover <SENHA>``: Remove a senha especificada do armazenamento.

``-x <ARQUIVO>, --exportar <ARQUIVO>``: Exporta as senhas armazenadas para o arquivo especificado.

## Exemplos
### Para gerar uma senha com 16 caracteres, incluindo letras maiúsculas, números e caracteres especiais:

```sh

cargo run -- --tamanho 16 --maiusculas --numeros --especiais

```
### Para listar todas as senhas armazenadas:
```
cargo run -- --listar

```

### Para remover uma senha chamada ``senha123``:
```
cargo run -- --remover senha123

```

### Para exportar as senhas para um arquivo chamado senhas.txt:
```
cargo run -- --exportar senhas.txt

```

## Hashing de Senhas
Este gerador de senhas utiliza hashing para armazenar senhas de forma segura. Quando você gera uma nova senha e a armazena, ela é convertida em um hash, que é uma representação fixa da senha. Isso significa que, mesmo que o arquivo de senhas seja comprometido, as senhas reais não podem ser facilmente recuperadas.

## Como Funciona
``Hashing``: As senhas são transformadas em hashes utilizando algoritmos seguros, garantindo que as senhas não sejam armazenadas em texto claro.

``Verificação``: Ao verificar senhas (por exemplo, ao tentar remover uma senha), o sistema compara o hash da senha fornecida com o hash armazenado.
