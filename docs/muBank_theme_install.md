# μBank JetBrains Theme — Instalação

## Arquivo

`muBank_jetbrains_theme.icls` — esquema de cores "μBank — Blindado por Design"

## Como instalar

1. Abra o IntelliJ IDEA (ou qualquer IDE JetBrains: CLion, RustRover, GoLand, PyCharm)
2. Vá em **File → Settings** (ou `Ctrl+Alt+S`)
3. Navegue para **Editor → Color Scheme**
4. Clique no ícone de engrenagem ⚙️ ao lado do seletor de esquema
5. Selecione **Import Scheme → IntelliJ IDEA color scheme (.icls)**
6. Escolha o arquivo `muBank_jetbrains_theme.icls`
7. O tema aparecerá como "μBank — Blindado por Design"
8. Aplique e feche

## Paleta

| Cor | Hex | Uso no tema |
|-----|-----|-------------|
| Preto absoluto | `#000000` | Fundo do editor, console |
| Cinza-chumbo | `#111116` | Painéis, abas, superfícies |
| Verde terminal | `#00FF41` | Keywords, cursor, destaque, git added |
| Dourado frio | `#D4AF37` | Strings, constantes, git renamed |
| Vermelho sangue | `#FF0040` | Erros, git deleted, search result |
| Ciano militar | `#00E5FF` | Números, macros, tipos, links, git modified |
| Cinza comentário | `#6A9955` | Comentários, atributos, git untracked |
| Branco suave | `#E0E0E0` | Texto corrido, identificadores |

## Cobertura

- **Rust** — keywords, macros, lifetimes, closures, raw strings, attributes
- **TOML** — Cargo.toml (keys, strings, numbers, booleans)
- **Markdown** — headings, lists, code blocks, links, tables, bold/italic
- **JSON** — keys, strings, numbers, booleans, null
- **Git** — diff highlighting (added/deleted/modified/renamed)
- **IDE** — console, tooltips, popups, tabs, gutters, tree selection, search results
