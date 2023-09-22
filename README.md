# Projeto Rust - Listagem de Objetos no Amazon S3

Este é um projeto simples em Rust que lista objetos em um bucket no Amazon S3 e extrai IDs a partir dos nomes dos objetos.

## Pré-requisitos

Antes de começar, certifique-se de ter o seguinte instalado em seu sistema:

- Rust: [Instalação do Rust](https://www.rust-lang.org/tools/install)
- Git: [Instalação do Git](https://git-scm.com/downloads)

## Configuração

1. Configure suas credenciais da AWS:

   Configure suas credenciais AWS como variáveis de ambiente no seu sistema operacional ou use um arquivo de configuração. Você precisa definir as seguintes variáveis de ambiente:

   - `AWS_ACCESS_KEY_ID`: Sua chave de acesso da AWS.
   - `AWS_SECRET_ACCESS_KEY`: Sua chave de acesso secreta da AWS.
   - `AWS_DEFAULT_REGION`: A região da AWS em que seu bucket está localizado (por exemplo, `sa-east-1` para São Paulo).

2. Clone este repositório:

   ```bash
   git clone https://github.com/pedroborba05/list_s3_objects_rust.git
