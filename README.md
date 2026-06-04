[releases]: https://github.com/eduardobaginskicosta/devns/releases
[docker]: https://hub.docker.com/r/baginskistudio/devns
[repo]: https://github.com/eduardobaginskicosta/domainnamesystem
[kofi]: https://ko-fi.com/baginskistudio

[rust]: https://rust-lang.org/tools/install/
[pwsh]: https://learn.microsoft.com/en-us/powershell/scripting/install/linux-overview
[wsl]: https://learn.microsoft.com/en-us/windows/wsl/install

[social_insta]: https://www.instagram.com/eduardobaginskicosta/
[social_yt]: https://www.youtube.com/@baginskistudio
[social_in]: https://www.linkedin.com/in/eduardobaginskicosta/
[social_x]: https://www.x.com/baginskistudio

# DevNS (Development Name Server)
[**❤️&ensp;Apoie o desenvolvimento deste e outros projetos no Ko-Fi&ensp;❤️**][kofi]

O **DevNS** é um servidor **DNS** escrito em **Rust** com foco em ser leve, rápido
e simples de ser utilizado. Este projeto é uma refatoração do repositório
[**domainnamesystem**][repo], também mantido por mim, trazendo grandes melhorias de
segurança e performance, além de corrigir erros de escrita e leitura dos pacotes
que, por sua vez, ocasionavam a incompatibilidade com sistemas **POSIX** que
apresnetam uma rigidez maior em relação as especificações **RFC 1034** e
**RFC 1035**.

> **Este projeto tem o foco de servir a ambientes de desenvolvimento.
> Não utilize em servidores de produção, mesmo que este seja projetado
> para manter grandes cargas de solicitação.**

**🐋 INÍCIO RÁPIDO DOCKER:
` git clone https://github.com/eduardobaginskicosta/devns-docker `.**

## 📦 Execução Local e Container Docker

Ao contrário do **domainnamesystem** que possui apenas cunho acadêmico, distribuído
de maneira mais dispersa e ampla, este cumpre com o propósito de ser um servidor DNS
robusto para ambientes de desenvolvimento em grande escala que, por comodidade ou
necessidade, precisam de domínios internos customizados para evitar a utilização
direta de IPs. Pensando nos desenvolvovedores, trago neste projeto dois meios de
utilização sendo:

- **Execução Local :** baixe os arquivos compactados para **linux** e **windows**
  diretamente na página de [**releases**][releases] do GitHub, extraia os binários
  e o execute-os. Para mais informações, avançe para a sessão de como compilar o
  programa **localmente**.

- **Container Docker :** suba um container **Docker** utilizando a imagem oficial
  [**baginskistudio/devns**][docker]: ` docker pull baginskistudio/devns `. Para
  mais informações, avançe para a sessão de como compilar o container **Docker**.

## 🦀 Compilar Localmente

Primeiramente, o `rust`, o `mingw-w64` e o `git` são dependências obrigatórias no
desenvolvimento. Se não tiver, instalado por favor [**instale o Rust**][rust]
através do site oficial e o `git` e `mingw-w64` através do seu gereciador
de pacotes:

```bash
# ubuntu, debian, linux mint
sudo apt update
sudo apt install mingw-w64 git

# fedora, red hat
sudo dnf install mingw64-gcc mingw64-gcc-c++ git

# arch linux
sudo pacman -S mingw-w64-gcc git
```

Após a instalação das dependências, clone este repositório localmente na máquina
de desenvolvimento. Recomenda-se a utilização do sistema operacional **Linux**,
se utilizar o **Windows**, por favor, realize a instalação do subsistema do
Linux, o [**WSL**][wsl], e também o [**PowerShell**][pwsh].

```bash
git clone https://github.com/eduardobaginskicosta/devns.git
cd devns
```

Para realizar a build e executar o projeto, execute os seguintes comandos dentro
da pasta (normalmente, utilizam o `bash` como shell padrão):

```bash
# linux / windows native (release)
./scripts/build.sh --release

# linux x86_64 gnu (release)
./scripts/build.sh --linux --release

# windows x86_64 gnu (release)
./scripts/build.sh --windows --release
```

Se a build for bem sucedida, basta executar o binário como **administrador** no
Windows e como **sudo** no Linux, através do `run.sh`, como indicado a seguir:

```bash
# linux native (release)
sudo ./scripts/run.sh --release

# linux x86_64 gnu (release)
sudo ./scripts/run.sh --linux --release

# windows x86_64 gnu (release)
sudo ./scripts/run.sh --windows --relase
```

> OBS : o binário tentará carregar os arquivos de **Zona DNS** a partir da pasta de
> execução, ou seja, se invocar o processo dentro da pasta raiz do sistema (`/`),
> ele tentará lêr os arquivos do caminho `/config`; se você mudar as configurações
> de diretório se atente a histo, caso contrário, basta executar estando dentro da
> pasta `docker` do repositório clonado, onde a pasta `config` está localizada ou,
> mova a mesma para a raiz do projeto.

Se executar corretamente, o **devns** irá escutar as requisições DNS na porta `53`
encima do seu IP atual, ou seja, o bind ocorrerá algo como `192.168.0.10:53`; para
saber qual IP o servidor está ouvindo a porta, basta verificar as configurações da
sua distribuição através do `ip addr` ou, se preferir, através dos logs iniciais
nas linhas `dns.bind` e `dns.port`.

Por padrão, o **devns** vem com zonas de demonstração pré-configuradas, sendo uma
destas as zonas `host`, `server`, `server.host` que apontam para o **localhost**
(ipv4: `127.0.0.1`, ipv6: `::1`); para testar se o servidor está funcionando
corretamente podemos utilizar os seguintes comandos para consultar a zona
(substitua o `DNS_IP` pelo IP que o servidor está sendo executado. Se estiver na
porta `53`, basta somente o IP, caso contrário, especifique a porta):

```bash
# linux & windows
nslookup host DNS_IP
nslookup host 192.168.0.10 # example

# linux
dig @DNS_IP host
dig @192.168.0.10 host # example
```

O resultado desssa requisição, no **Linux (Ubuntu)**, se estiver funcionando
corretamente o resultado deve ser algo semelhante a isto (pode alterar de acordo
com cada distribuição):

```txt
; <<>> DiG 9.20.18-1ubuntu2.1-Ubuntu <<>> @192.168.0.10 host
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 63151
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 1, ADDITIONAL: 1

;; QUESTION SECTION:
;host.				IN	A

;; ANSWER SECTION:
host.			3600	IN	A	127.0.0.1
host.			3600	IN	AAAA	::

;; AUTHORITY SECTION:
host.			3600	IN	NS	ns1.host.

;; ADDITIONAL SECTION:
host.			3600	IN	MX	10 mail.host.

;; Query time: 0 msec
;; SERVER: 192.168.0.10#53(192.168.0.10) (UDP)
;; WHEN: Thu Jun 04 14:57:37 -03 2026
;; MSG SIZE  rcvd: 129
```

Como esperado, o **devns** retornou os IPs de **localhost**, assim como previamente
declarado no arquivo de zona [`zone.dev`](./config/zone.dev). Os aquivos de
**Zona DNS** são simples e intuitivos de utilizar, para saber mais a respeito,
avançe para a seção das **Zonas DNS**.

> Para mais informações de como utilizar os scripts [`build.sh`](./scripts/build.sh)
> e [`run.sh`](./scripts/run.sh), por favor, leia os comentários incluídos nos
> cabeçalhos dos arquivos para uma lista completa de comandos ou, se preferir,
> avançe para a seção de comandos.

Algums comportamentos do `devns` podem ser modificados através das variáveis de
ambiantes passadas diretamente na linha de comando ou arquivos de script através
da exportação. Acompnahe a seguir a lista das variáveis disponíveis (compatíveis
com o container **Docker**):

- ` DEBUG_MODE ` : é uma variável **booleana** que define se o **devns** deve
  exibir as mensagens de depuração opcionais no terminal. Os valores aceitos são:
  - `1`, `TRUE`, `True`, `true` -- habilitar a depuração.
  - `0`, `FALSE`, `False`, `false` -- desabilitar a depuração.
  - Em caso de valor inválido -- habilita a depuração (comportamento padrão).

- ` MAX_MESSAGES ` : é uma variável **usize** que define a quantidade máxima de
  mensagens em fila que cada **Worker** do **devns** pode gerenciar ao mesmo tempo,
  sem que este entre na fila de espera. Somente aceita valores numéricos de `1`
  até `10000`, sendo o padrão `20`.

- ` MAX_WORKERS ` : é uma variável **usize** que define a quantidade máxima de
  **Workers** que rodam juntos. Somente aceita valores numéricos de `1` até `256`,
  sendo o padrão `10`.

- ` PORT ` : é uma variável **u16** que define a porta que o servidor DNS deve
  ouvir as requisições. Somente aceita valores numéricos de `53` até `9000`,
  sendo o padrão `53`.

- ` DNS_SERVERS ` : é uma variável **Vec\<Ipv4Addr\>** que define os servidores
  DNS de lookup ao qual as requisições são repassadas caso o **devns** não esteja
  configurado para responder. Aceita uma **String** que pode conter um ou mais
  endereços IPV4, separados por ponto e vírgula, tal como `1.1.1.1`,
  `1.1.1.1;1.0.0.1` e assim por diante. O valor padrão é `1.1.1.1;1.0.0.1;8.8.8.8`,
  mas se não especificado assume os lookups padrões (`ROOT_SERVERS`):
  `198.41.0.4;1.1.1.1;1.0.0.1`.

Todas as variáveis de ambiente possuem valores padrão, definido por código, em caso
de erro de interpretação ou definição de valores errados. Nenhum alerta é desparado
no terminal, deve-se atentar se as configurações foram corretamente aplicadas na
mensagem de log inicial do **devns** com as configurações do servidor.

_**OBS :** este projeto possui **Zonas DNS** previamente configuradas na pasta
[`config`](./config) que, por padrão, são utilizados na construção do conatiner
**Docker**. Estão contidos exemplos básicos como **domínios** para desenvolvimento
local como o `host` e `server`, bem como exemplos de **sobreescrita de domínios**
demonstrado com os serviços de **Proteção Parental** através do DNS, dispoibilizado
pelo **Google**, **YouTube**, **Bing** e **DuckDuckGo**._

## 🐋 Compilar Container Docker

Para compilar para uma imagem do **Docker**, você deve ter as mesmas dependências
exigidas na compilação local, ou seja, os pacotes `rust`, `mingw-w64` e `git`,
instalados. Como anteriormente, recomendamos a utilização do sistema operacional
**Linux** ou, se for utilizar o **Windows**, realize a instalação do [**WSL**][wsl].

A compilação foi simplificada através do script `docker.sh` que, assim como os
demais, deve ser executado na pasta raiz do projeto. Os principais comandos
disponíveis são:

```bash
# rust build linux (release) + docker build + docker compose up
./scripts/docker.sh

# rust build linux (release) + docker build
./scripts/docker.sh --build-only

# docker compose down
./scripts/docker.sh --down
```

Para testar a container pré-configurado neste projeto, execute o `docker.sh`
diretamente, sem nenhum argumento adicional. Todo o código **rust** será compilado
em **release** para o **Linux x86_64 (GNU)**, a imagem será compilada localmente e
o container será subido automaticamente com o nome de `devns`. Ao subir, o servidor
DNS fará bind automático na porta `53` (se nada for alterado) no **host**, bastando
apenas referenciar o IP local, como no exemplo abaixo (considerando que o seu IP
seja `192.168.0.10`; verifique o seu IP através das configurações do sistema ou
do comando `ip addr` no **Linux**):

```bash
# linux & windows
nslookup host DNS_IP
nslookup host 192.168.0.10 # example

# linux
dig @DNS_IP host
dig @192.168.0.10 host # example
```

Se tudo funcionou corretamente, o resultado deve ser algo parecido com isso, no
**Linux (Ubuntu)**, considerando as configurações padrão:

```txt
; <<>> DiG 9.20.18-1ubuntu2.1-Ubuntu <<>> @192.168.0.10 host
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 63151
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 1, ADDITIONAL: 1

;; QUESTION SECTION:
;host.				IN	A

;; ANSWER SECTION:
host.			3600	IN	A	127.0.0.1
host.			3600	IN	AAAA	::

;; AUTHORITY SECTION:
host.			3600	IN	NS	ns1.host.

;; ADDITIONAL SECTION:
host.			3600	IN	MX	10 mail.host.

;; Query time: 0 msec
;; SERVER: 192.168.0.10#53(192.168.0.10) (UDP)
;; WHEN: Thu Jun 04 14:57:37 -03 2026
;; MSG SIZE  rcvd: 129
```

Como o esperado, o **devns** retornou os IPs do **localhost**, assim como
previamente configurado no arquivo de zona [`zone.dev`](./config/zone.dev). Os
aquivos de **Zona DNS** são simples e intuitivos de utilizar, para saber mais a
respeito, avançe para a seção das **Zonas DNS**.

Algums comportamentos do `devns` podem ser modificados através das variáveis de
ambiantes passadas diretamente na linha de comando ou arquivos de script através
da exportação. Acompnahe a seguir a lista das variáveis disponíveis (compatíveis
com a **execução local**):

- ` DEBUG_MODE ` : é uma variável **booleana** que define se o **devns** deve
  exibir as mensagens de depuração opcionais no terminal. Os valores aceitos são:
  - `1`, `TRUE`, `True`, `true` -- habilitar a depuração.
  - `0`, `FALSE`, `False`, `false` -- desabilitar a depuração.
  - Em caso de valor inválido -- habilita a depuração (comportamento padrão).

- ` MAX_MESSAGES ` : é uma variável **usize** que define a quantidade máxima de
  mensagens em fila que cada **Worker** do **devns** pode gerenciar ao mesmo tempo,
  sem que este entre na fila de espera. Somente aceita valores numéricos de `1`
  até `10000`, sendo o padrão `20`.

- ` MAX_WORKERS ` : é uma variável **usize** que define a quantidade máxima de
  **Workers** que rodam juntos. Somente aceita valores numéricos de `1` até `256`,
  sendo o padrão `10`.

- ` PORT ` : é uma variável **u16** que define a porta que o servidor DNS deve
  ouvir as requisições. Somente aceita valores numéricos de `53` até `9000`,
  sendo o padrão `53`.

- ` DNS_SERVERS ` : é uma variável **Vec\<Ipv4Addr\>** que define os servidores
  DNS de lookup ao qual as requisições são repassadas caso o **devns** não esteja
  configurado para responder. Aceita uma **String** que pode conter um ou mais
  endereços IPV4, separados por ponto e vírgula, tal como `1.1.1.1`,
  `1.1.1.1;1.0.0.1` e assim por diante. O valor padrão é `1.1.1.1;1.0.0.1;8.8.8.8`,
  mas se não especificado assume os lookups padrões (`ROOT_SERVERS`):
  `198.41.0.4;1.1.1.1;1.0.0.1`.

Todas as variáveis de ambiente possuem valores padrão, definido por código, em caso
de erro de interpretação ou definição de valores errados. Nenhum alerta é desparado
no terminal, deve-se atentar se as configurações foram corretamente aplicadas na
mensagem de log inicial do **devns** com as configurações do servidor.

> **Se utilizar o `docker-compose.yaml` deste projeto, todos os valores padrão estão
> previamente definidos. Fique atento a erros de digitação pois, nenhum alerta é
> emitido, sendo assumido os valores definidos em código.**

_**OBS :** este projeto possui **Zonas DNS** previamente configuradas na pasta
[`config`](./config) que, por padrão, são utilizados na construção do conatiner
**Docker**. Estão contidos exemplos básicos como **domínios** para desenvolvimento
local como o `host` e `server`, bem como exemplos de **sobreescrita de domínios**
demonstrado com os serviços de **Proteção Parental** através do DNS, dispoibilizado
pelo **Google**, **YouTube**, **Bing** e **DuckDuckGo**._

## ⚙️ Definição das Zonas DNS

Todas as **Zonas DNS** do servidor **devns** são declaradas através de arquivos
`zone.` dentro da pasta [`config`](./config), onde seguem uma estruturação simples
e rápida. Cada arquivo de zona pode conter **múltiplos domínios (`ZONE`)**
apontando para endereços **IPV4 (`A`)** e **IPV6 (`AAAA`)**, possuindo
**Servidores de Nome (`NS`)**, **Autoridade (`MX`)** e
**Tempo de Vida (`TTL`)** próprios.

Os arquivos são lidos de forma recursiva dentro da pasta [`config`](./config)
desde que atendam ao requisito de iniciarem com `zone.` e sigam a estruturação
correta. Uma obervação importante é que o **devns** sempre irá ler a pasta
`config` relativo ao caminho do processo que o invoca, ou seja, se estiver na
**pasta raíz (`/`)** do sistema, o **devns** tentará ler os arquivos do caminho
relativo `/config`. Se a pasta das zonas etiverem em um caminho específico como
`/app/config`, acesse a pasta e, após, execute o binário; caso contrário, o
**devns** somente servirá de **lookup** de repasse. Para os passos seguintes,
será considerado a utilização do projeto atual.

Para declarar uma nova **Zona DNS** acesse a pasta [`config`](./config) e
crie um arquivo com `zone.` no início, ou seja, se irá criar uma zona para o
domínio `example.dev` crie o arquivo `zone.example.dev` (boas práticas) ou
`zone.example`. O arquivo pode estar contido na raiz da pasta ou dentro de
subpastas, se necessário, para uma melhor organização. O arquivo deve conter
a seguinte estruturação (o exemplo será com o `example.dev`):

```zone
@ ZONE: example.dev, example.local
@ TTL: 3600
@ NS: localhost
@ MX: mail.localhost
@ A: $LOCALHOST
@ AAAA: $LOCALHOST
```

No exemplo demonstrado acima, declaramos que os **domínios/zonas** `example.dev`
e `example.local` devem apontar para os endereços IPV4 e IPV6 do **localhost**.
As palavras-chave `$LOCALHOST` são substituídas automaticamente, sendo um atalho
mais seguro e confiável para apontamento local, sendo o recomendado. Por padrão,
devido ao objetivo de ser utilizado em ambientes de desenvolvimento, o cacheamento
é declaro para não mais de `3600`, o equivalente a **1 Hora**.

Este exemplo demonstra a declaração simples de uma **Zona DNS** mas podemos declarar
vários endereços IPV4 (`A`) e IPV6 (`AAAA`) através da declaração separada por
**vírgula (`,`)**, tal como feito no linha `ZONE`:

```zone
; ...
@ A: 172.66.147.243, 104.20.23.154
; ...
```

Bem como, pode-se declarar que a **zona** não possua endereços IPV4 ou IPV6
deixando os campos em branco. Importante: ao menos um dos campos deve ser declarado,
caso contrário, será retornado um erro pois o **devns** responderá com dados vazios,
algo que os resolvedores como `dig` e `nslookup` não aceitam, pois ele não repassará
para os **lookups** responderem.

### Sistema de Bloqueio Nativo

As **Zonas DNS** trazem um recurso nativo de bloqueio de domínios, desde que estes
não ocorram através do **DoH (DNS Over Https)**, onde todas as zonas que declaram
em algum lugar o IPV4 (`A`) ou o IPV6 (`AAAA`) como endereços vazios são
automaticamente **Recusados (`Refused`)**, o que acabada realizando o impedimento
de acesso.

```zone
; ...
@ A: 0.0.0.0
; or/and
@ AAAA: 0:0:0:0:0:0:0:0
; ...
```

## 📃 Licenciamento e Contribuição

Este projeto é licenciado através do BSD-3-Clause - leia o [**LICENSE**](./LICENSE)
para mais informações.

Sinta-se livre para contribuir com este projeto fazendo _fork_ do repositório,
deixando sua estrela, enviando _pull requests_, ou reportando uma _issue_. Se
você quiser adicionar recursos novo ou melhorar o código atual, abra uma
_issue_ ou faça um _pull request_, e ficarei feliz em revisar.

## ❤️ Apoiar o Desenvolvimento e Redes Sociais

Por favor, considere em [**me apoiar no Ko-Fi**][kofi] através de uma doação
única ou mensal e tenha acesso ao servidor do **Discord**, onde publico o
progresso de projetos em andamento, projetos futuros e muito mais.

Ou, se preferir, me siga nas redes sociais:
- [**Seguir no Instagram**][social_insta] -- publico fotos e momentos do meu dia a dia.
- [**Seguir no YouTube**][social_yt] -- publico vídeos a respeito de projetos no
  geral, não limitando-se apenas à programação.
- [**Seguir no Twitter (X)**][social_x] -- publico pronunciamentos curtos e rápidos.
- [**Seguir no LinkedIn**][social_in] -- contexto mais profissional, porém com
  conteúdo de qualidade.
