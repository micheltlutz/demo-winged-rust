# demo-winged-rust

**Ao vivo: <https://demo-winged-rust.micheltlutz.me>**

Uma página *about.me* — avatar, bio, lista de links e rodapé — escrita em Rust com
[winged-rust](https://github.com/micheltlutz/winged-rust) e renderizada **duas vezes**:

1. no build, pelo binário nativo (`StaticSiteGenerator` grava `dist/`);
2. no seu navegador, pelo mesmo código compilado para WebAssembly.

A página compara os dois resultados byte a byte e mostra o veredito. Hoje, com este
conteúdo, são **13.292 bytes idênticos** e um módulo de **43,4 KB (≈20 KB gzipped)**.

Não há framework, runtime, template nem hidratação. O HTML que chega ao navegador já está
completo — o WebAssembly só existe para provar que ele poderia ter sido gerado ali mesmo.

## Como funciona

```
src/page.rs  →  pub fn document() -> Document      ← o único markup do projeto
      │
      ├─ src/bin/build_site.rs   (nativo, feature "site")
      │     dist/index.html, dist/parity/native.txt, sitemap.xml, robots.txt
      │
      └─ src/lib.rs              (cdylib, feature "wasm")
            #[wasm_bindgen] renderProfile() -> String
```

`web/boot.js` carrega o módulo, chama `renderProfile()`, baixa `parity/native.txt` — que é
byte a byte o mesmo arquivo que `index.html` — e compara.

Nada em `document()` pode depender do relógio, do sistema de arquivos ou do ambiente em
tempo de execução: se os dois renders divergirem por um byte, o selo fica vermelho, e é
exatamente para isso que ele serve.

## Rodando localmente

Requer Rust ≥ 1.85 e `wasm-pack`:

```sh
brew install wasm-pack        # ou: cargo install wasm-pack
./scripts/build.sh
./scripts/serve.sh            # http://localhost:8000
```

`file://` não carrega WebAssembly — use o `serve.sh` (ou qualquer servidor estático).

No macOS com o Rust do Homebrew, os binários ficam em `/opt/homebrew/opt/rustup/bin`, fora
do `PATH` padrão; o `build.sh` adiciona esse caminho sozinho quando `cargo` não aparece.

Sem browser, dá para conferir a paridade pelo Node:

```sh
node --input-type=module -e '
  import { readFileSync } from "node:fs";
  const { initSync, renderProfile } = await import("./dist/pkg/winged_demo.js");
  initSync({ module: readFileSync("dist/pkg/winged_demo_bg.wasm") });
  const native = readFileSync("dist/parity/native.txt", "utf8");
  console.log(renderProfile() === native ? "idênticos ✓" : "divergem ✗");
'
```

## Editando o conteúdo

Tudo o que é pessoal está em [`src/profile.rs`](src/profile.rs), numa constante só:
nome, `@handle`, bio, avatar e a lista de links. O `<title>`, as tags Open Graph, o
`sitemap.xml` e o próprio HTML saem dali.

O visual está em [`src/style.css`](src/style.css), embutido no HTML em tempo de compilação
com `include_str!` — a página é uma requisição só, e já vem com tema claro e escuro.

O build falha de propósito quando a auditoria de acessibilidade encontra algo (um `alt`
faltando, um link sem texto). Isso é `winged_rust::accessibility::audit` rodando dentro do
`build-site`.

## Deploy

A `dist/` é estática e usa **apenas caminhos relativos**, então o mesmo diretório serve nos
três destinos abaixo sem reescrita — inclusive sob um subcaminho como
`/demo-winged-rust/`.

Em todos eles, `DEMO_SITE_URL` é lido **em tempo de compilação** e define o
`<link rel="canonical">`, as tags `og:*` e o `sitemap.xml`. Mudou o domínio, refaça o
build. Sem a variável, o padrão é `https://demo-winged-rust.micheltlutz.me`, a casa
pública do demo — os outros destinos são espelhos e apontam o canonical para lá.

### GitHub Pages

O workflow [`.github/workflows/pages.yml`](.github/workflows/pages.yml) já faz tudo a cada
push na `main`: instala o toolchain com o alvo `wasm32-unknown-unknown`, roda `fmt` e
`clippy`, chama `./scripts/build.sh` e publica `dist/`.

Um passo é manual e o workflow não consegue fazer por você:

> **Settings → Pages → Source: `GitHub Actions`** (não "Deploy from a branch").

Sem isso o job de deploy falha com *Resource not accessible by integration*. Publica em
`https://micheltlutz.github.io/demo-winged-rust/`.

### Fly.io

[`deploy/Dockerfile`](deploy/Dockerfile) é multi-stage: compila com `rust:1-slim` +
`wasm-pack` e serve com `nginx:alpine`. O [`deploy/nginx.conf`](deploy/nginx.conf) garante
`Content-Type: application/wasm` e liga gzip para o módulo. O `boot.js` deste demo baixa o
`.wasm` como `ArrayBuffer` e não depende do MIME, mas qualquer host que sirva WebAssembly
como `application/octet-stream` quebra o caminho normal (`instantiateStreaming`) — por isso
o cabeçalho está fixado aqui e no Amplify.

```sh
fly launch --no-deploy        # cria o app; aceite o fly.toml existente
fly deploy
```

Antes do primeiro deploy, ajuste `app` e `build.args.DEMO_SITE_URL` no
[`fly.toml`](fly.toml) para o domínio que o Fly te der. A máquina dorme com
`min_machines_running = 0` e acorda na primeira requisição.

Para validar a imagem sem publicar nada:

```sh
docker build -f deploy/Dockerfile -t winged-demo .
docker run --rm -p 8080:80 winged-demo
```

> **Por que o `fly.toml` está versionado e o `amplify.yml` não.** O `fly deploy` lê o
> `fly.toml` do diretório de trabalho — sem ele no repositório, não há deploy reproduzível.
> O Amplify é o contrário: guarda a configuração no console e só a ignora quando existe um
> arquivo no repositório, que então sobrescreve o console a cada push.
>
> O `fly.toml` daqui não carrega segredo nenhum — só o nome do app, a região e o tamanho da
> máquina. Tokens ficam no `flyctl`, nunca no arquivo. Se você clonou isto para publicar a
> sua própria cópia, troque `app` por um nome livre e `build.args.DEMO_SITE_URL` pelo seu
> domínio, ou apague o arquivo e deixe o `fly launch` gerar um do zero.

### AWS Amplify

**Este repositório não tem `amplify.yml` de propósito.** O Amplify dá precedência ao
arquivo do repositório sobre as configurações do console — com ele versionado, todo push
sobrescreve o que você ajustar na interface. Aqui a configuração mora só no console, e o
repositório fica com o código.

Conecte o repositório e a branch `main` e conclua a criação do app. **Nada disto está no
assistente de criação** — os itens abaixo vivem no menu **Hosting** (*Hospedagem*, no
console em português) e só existem depois que o app foi criado.

Em **Hosting → Build settings → Edit**, cole:

```yaml
version: 1
frontend:
  phases:
    preBuild:
      commands:
        - curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
        - source "$HOME/.cargo/env"
        - curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
    build:
      commands:
        - source "$HOME/.cargo/env"
        - export DEMO_SITE_URL="${DEMO_SITE_URL:-https://$AWS_BRANCH.$AWS_APP_ID.amplifyapp.com}"
        - ./scripts/build.sh
  artifacts:
    baseDirectory: dist
    files:
      - '**/*'
  cache:
    paths:
      - $HOME/.cargo/registry/**/*
      - $HOME/.cargo/git/**/*
      - target/**/*
```

Em **Hosting → Custom headers** (*Hospedagem → Cache e cabeçalhos personalizados*) →
**Edit**, cole:

```yaml
customHeaders:
  - pattern: '/pkg/*.wasm'
    headers:
      - key: Content-Type
        value: application/wasm
```

O `pattern` segue a forma `'/path/*.ext'` da [referência da
AWS](https://docs.aws.amazon.com/amplify/latest/userguide/custom-header-YAML-format.html),
que é a documentada para casar extensão dentro de um diretório. Salvar não basta: é preciso
um **redeploy** para o cabeçalho valer.

Vale dizer que isto é cinto-e-suspensório. O `boot.js` instancia o módulo a partir de um
`ArrayBuffer`, então a página funciona mesmo sem o cabeçalho — ele existe para quem trocar
o `boot.js` pelo caminho normal (`instantiateStreaming`), que recusa qualquer MIME que não
seja `application/wasm`.

Três notas:

- Este demo roda em `demo-winged-rust.micheltlutz.me`, apontado para a branch `main`.
  Depois de mapear o domínio em **Hosting → Custom domains**, defina
  `DEMO_SITE_URL = https://demo-winged-rust.micheltlutz.me` em **Hosting →
  Environment variables → Manage variables** — ele é lido em tempo de compilação e define o
  `<link rel="canonical">`, as tags `og:*` e o `sitemap.xml`.
- Sem essa variável, o buildspec cai na URL da branch
  (`$AWS_BRANCH.$AWS_APP_ID.amplifyapp.com`), que é o que você quer em branches de
  preview.
- O cache de `~/.cargo` e `target/` está no bloco acima: o primeiro build leva alguns
  minutos instalando o toolchain, os seguintes são rápidos.

Se você preferir versionar mesmo assim — num fork, por exemplo —, é só salvar o primeiro
bloco como `amplify.yml` na raiz e o segundo como `customHttp.yml`. O Amplify passa a ler
os dois do repositório e ignora o console. Vale saber que a própria AWS recomenda o
caminho oposto para os cabeçalhos: tirá-los do buildspec e mantê-los no console ou no
`customHttp.yml`, nunca no `amplify.yml`.

## Referências

A documentação que foi consultada para montar cada parte deste repositório. Todos os links
foram verificados.

**AWS Amplify**

- [Cabeçalhos personalizados — visão geral](https://docs.aws.amazon.com/amplify/latest/userguide/custom-headers.html)
- [Definindo cabeçalhos personalizados](https://docs.aws.amazon.com/amplify/latest/userguide/setting-custom-headers.html) — o passo a passo do console e do `customHttp.yml`, e a regra de que o arquivo no repositório sobrescreve o console
- [Formato do YAML de cabeçalhos](https://docs.aws.amazon.com/amplify/latest/userguide/custom-header-YAML-format.html) — as formas de `pattern` que são documentadas
- [Migrando cabeçalhos para fora do buildspec](https://docs.aws.amazon.com/amplify/latest/userguide/migrate-custom-headers.html) — por que a AWS recomenda não deixá-los no `amplify.yml`
- [Configurações de compilação](https://docs.aws.amazon.com/amplify/latest/userguide/build-settings.html) — inclui a precedência do `amplify.yml` sobre o console
- [Imagem e ambiente de compilação](https://docs.aws.amazon.com/amplify/latest/userguide/using-build-image-settings.html)
- [Variáveis de ambiente](https://docs.aws.amazon.com/amplify/latest/userguide/setting-env-vars.html)
- [Domínio próprio via Route 53](https://docs.aws.amazon.com/amplify/latest/userguide/to-add-a-custom-domain-managed-by-amazon-route-53.html)

**Fly.io**

- [Referência do `fly.toml`](https://fly.io/docs/reference/configuration/)
- [Servindo conteúdo estático](https://fly.io/docs/languages-and-frameworks/static/)

**GitHub Pages**

- [Escolhendo a origem de publicação](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site) — é onde se troca de branch para **GitHub Actions**

**Rust e WebAssembly**

- [winged-rust](https://github.com/micheltlutz/winged-rust) — a biblioteca
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) — o empacotador usado pelo `scripts/build.sh`
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) — o que exporta `renderProfile` para o JavaScript
- [Carregando e executando WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Loading_and_running) — por que o MIME importa no caminho `instantiateStreaming`

## Licença

MIT, como o winged-rust.
