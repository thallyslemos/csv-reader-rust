# CSV Reader

CSV Reader é uma aplicação Rust que permite ler, processar e exibir informações de arquivos CSV contendo dados de infrações de trânsito. A aplicação suporta várias operações como exibir todas as infrações, agrupar infrações por tipo ou UF, e listar códigos de infração únicos.

## Requerimentos

Para construir e executar este projeto, você precisará do Rust e do Cargo instalados em seu sistema. A versão mínima do Rust requerida é a 2021 edition. Você pode instalar o Rust através do [rustup](https://rustup.rs/).

## Dependências

Este projeto depende das seguintes crates:

- `csv` para leitura e manipulação de arquivos CSV.
- `serde` e `serde_derive` para serialização e deserialização de dados.
- `structopt` para análise de argumentos da linha de comando.

As dependências estão especificadas no arquivo `Cargo.toml` do projeto.

## Construção

Para construir o projeto, navegue até o diretório raiz do projeto no terminal e execute:

```bash
cargo build --release
```

Isso compilará o projeto no modo de release, otimizando o binário final.

## Execução

Após a construção, você pode executar o programa usando o Cargo ou diretamente pelo binário compilado. Aqui está como você pode executar o programa com o Cargo:

```bash
cargo run -- [OPERAÇÃO] [ARGUMENTOS]
```

### Operações Suportadas

- `exibir`: Exibe todas as infrações.
- `agrupar_por_tipo`: Agrupa e soma as quantidades de infrações por tipo.
- `agrupar_por_uf`: Agrupa e soma as quantidades de infrações por UF.
- `codigos_unicos`: Lista todos os códigos de infração únicos.

### Argumentos

- `--infracoes`: Caminhos para os arquivos CSV contendo os dados das infrações. Pode ser especificado múltiplas vezes para vários arquivos.
- `--descricao`: Caminho para o arquivo CSV contendo a descrição das infrações.

Exemplo de uso:


```bash
cargo run -- --infracoes assets/2023_01_infracoes_com_np.csv --infracoes assets/2023_02_infracoes_com_np.csv --infracoes assets/2023_03_infracoes_com_np.csv --descricao assets/tabela-codigo-infracoes-renainf.csv --operacao agrupar_por_tipo
```

Isso irá ler todos os registros de infracoes dos tres arquivos de infracoes e o arquivo tabela-codigo-infracoes-renainf.csv com o dicionário dos nomes das infracoes e irá agrupar por tipo




