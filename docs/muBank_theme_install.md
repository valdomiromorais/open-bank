# μBank JetBrains Theme — Instalação

## Temas disponíveis

| Arquivo | Nome | Estilo |
|---------|------|--------|
| `muBank_jetbrains_theme.icls` | μBank — Blindado por Design | Fundo preto absoluto, acentos agressivos |
| `muBank_nord_theme.icls` | μBank × Nord — Blindado por Design | Base Nord Darker (azul-escuro suave) + acentos μBank |

### Qual escolher?

- **μBank puro** — fundo `#000000` (preto absoluto), mais dramático, cansativo em longas horas, vibe red team máxima
- **μBank × Nord** — fundo `#2E3440` (azul-escuro Nord), mais confortável para maratona de código, mantém os acentos μBank (verde terminal, ouro, ciano)

## Como instalar

1. Abra o IntelliJ IDEA (ou qualquer IDE JetBrains: CLion, RustRover, GoLand, PyCharm)
2. **File → Settings** (ou `Ctrl+Alt+S`)
3. **Editor → Color Scheme**
4. Clique no ⚙️ ao lado do seletor de esquema
5. **Import Scheme → IntelliJ IDEA color scheme (.icls)**
6. Escolha o `.icls` desejado
7. Selecione o tema na lista e aplique

## Paleta híbrida (μBank × Nord)

| Função                     | Cor                  | Hex           | Origem    |
|----------------------------|----------------------|---------------|-----------|
| Fundo editor               | Azul petróleo escuro | `#2E3440`     | Nord      |
| Painéis, sidebar           | Azul petróleo médio  | `#3B4252`     | Nord      |
| Seleção                    | Azul petróleo claro  | `#434C5E`     | Nord      |
| Linhas, comentários        | Azul acinzentado     | `#4C566A`     | Nord      |
| Texto                      | Branco gelo          | `#D8DEE9`     | Nord      |
| **Keywords**               | **Verde terminal**   | **`#00FF41`** | **μBank** |
| **Strings, constantes**    | **Dourado frio**     | **`#D4AF37`** | **μBank** |
| **Números, tipos, macros** | **Ciano militar**    | **`#00E5FF`** | **μBank** |
| **Erros**                  | **Vermelho sangue**  | **`#FF0040`** | **μBank** |
| Lifetimes, atributos       | Lilás Nord           | `#B48EAD`     | Nord      |
| Cursor                     | Verde terminal       | `#00FF41`     | μBank     |

## Cobertura (ambos os temas)

- **Rust** — keywords, macros, lifetimes, closures, raw strings, attributes
- **TOML** — Cargo.toml (keys, strings, numbers, booleans)
- **Markdown** — headings, lists, code blocks, links, tables, bold/italic
- **JSON** — keys, strings, numbers, booleans, null
- **Git** — diff highlighting (added/deleted/modified/renamed)
- **IDE** — console, tooltips, popups, tabs, gutters, tree selection, search results
