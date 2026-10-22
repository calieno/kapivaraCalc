# Rusty HP-49G (Calculadora Gráfica & Simbólica Moderna)

> **Recriação moderna e elegante da lendária calculadora científica HP 49G em Rust ("Rusty").**  
> Todas as funções matemáticas, rotinas CAS (Computer Algebra System), álgebra linear, TVM financeiro, constantes físicas, conversor de unidades, servidor web embutido de altíssimo desempenho e interface glassmorphic neumorphic estão **integrados em um único arquivo de código-fonte (`src/main.rs`)**.

---

## 🚀 Destaques do Projeto

- **Arquitetura 100% Single-File:** Motor matemático, pilha RPN de 4 níveis, avaliador de expressões, servidor web e UI estão integrados em [`src/main.rs`](src/main.rs) com zero dependências externas pesadas.
- **Design Moderno & Elegante:**
  - Chassi com acabamento em titânio e cantos arredondados contínuos.
  - Tela LCD/OLED de alto contraste com luminescência âmbar/verde esmeralda e annunciators dinâmicos (`RAD`, `DEG`, `DEC`, `HEX`, `SHIFT`, `ALPHA`, `BUSY`).
  - Botões táteis com física realista, feedback sonoro sintético (Web Audio API) e vibração háptica no celular.
  - 6 Softkeys contextuais dinâmicos (F1 a F6) para navegação rápida entre categorias (`MATH`, `CAS`, `MATR`, `TVM`, `STAT`, `CONST`, `UNIT`).
- **Poder Matemático Herdado da ROM `rom.e49` (VGER 1.19-6):**
  - **Pilha RPN HP:** Níveis 1 (X), 2 (Y), 3 (Z) e 4 (T) com histórico, `SWAP`, `ROLL`, `DROP` e `UNDO`.
  - **Álgebra Linear & Matrizes:** Determinante (`DET`), Transposta (`TRAN`), Inversa (`INV`), Multiplicação de matrizes e Resolução de Sistemas $A \cdot X = B$.
  - **Cálculo Numérico & CAS:** Derivadas numéricas de alta ordem $f'(x)$ e $f''(x)$, Integração definida adaptativa (Regra dos 3/8 de Simpson) e expansão de Taylor.
  - **Localizador de Raízes Polinomiais:** Algoritmo Durand-Kerner (raízes reais e complexas no plano Argand-Gauss).
  - **Finanças (TVM - Time Value of Money):** Solver completo para $N$, $I\%YR$, $PV$, $PMT$, $FV$ em modo Begin/End.
  - **Equation Library:** Constantes universais ($c$, $h$, $\hbar$, $G$, $g_0$, $e$, $m_e$, $m_p$, $k_B$, $N_A$, $R$, $\varepsilon_0$, $\mu_0$, $\sigma$).
  - **Conversor Multidimensional de Unidades:** Comprimento, Massa, Temperatura, Pressão, Energia e Potência.
  - **Modos de Base de Programador:** `DEC`, `HEX`, `OCT` e `BIN`.

---

## 💻 Como Executar

### 1. Modo Gráfico Moderno (Desktop / Browser)
Ao iniciar o executável, o Rusty HP-49G inicia o servidor embutido e abre automaticamente seu navegador padrão com a calculadora pronta para uso:
```bash
cargo run --release
```
Ou execute o binário compilado:
```bash
./dist/rusty_hp49_linux_x86_64
```
Acesse no seu computador ou celular conectado na mesma rede:  
👉 **`http://localhost:4949`**

### 2. Modo Terminal / CLI (Linha de Comando RPN)
Para executar diretamente no terminal ou em scripts:
```bash
cargo run --release -- --cli
```
Exemplo de sessão:
```text
RPN> 12
1: 12
RPN> 34
2: 12
1: 34
RPN> +
1: 46
RPN> 100
RPN> CONV_M_FT
[Result: 100 m = 328.083990 ft]
```

---

## 📱 Suporte Multiplataforma (Windows, Mac, Linux, Android e iPhone)

### 🐧 Linux
Binário compilado estaticamente com `musl` (funciona em qualquer distribuição Linux, Ubuntu, Debian, Fedora, Arch, Alpine, etc.):
```bash
./build_linux.sh
```
Arquivo gerado: `dist/rusty_hp49_linux_x86_64`

### 🪟 Windows (.exe)
```cmd
build_windows.bat
```
Arquivo gerado: `dist/rusty_hp49_windows_x64.exe`

### 🍏 macOS (Universal Binary x86_64 + Apple Silicon M1/M2/M3/M4)
```bash
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/rusty_hp49 target/aarch64-apple-darwin/release/rusty_hp49 -output dist/rusty_hp49_macos_universal
```

### 🤖 Android (APK)
1. **Compilação Nativa APK:**
   ```bash
   ./build_android.sh
   ```
   Gera o pacote `dist/rusty_hp49.apk` pronto para instalar.
2. **PWA Standalone (1-Clique sem loja):**
   Abra o endereço da calculadora no Chrome no celular Android e selecione **"Adicionar à tela inicial"** ou **"Instalar Aplicativo"**. O app funciona 100% offline via Service Worker!

### 📱 iPhone / iOS
1. **Projeto iOS Nativo:**
   Código Swift com `WKWebView` de altíssima performance localizado em `rusty_hp49/ios/`. Abra o projeto no Xcode para compilar e gerar o arquivo `.ipa`.
2. **PWA Standalone para iPhone (Recomendado):**
   Abra no Safari no iPhone e toque em **Compartilhar -> "Adicionar à Tela de Início"**. O Rusty HP-49G se comporta como um app nativo sem barras do navegador, com suporte a gestos e vibração háptica!

---

## ⚙️ Integração Contínua (GitHub Actions)

O repositório já inclui um fluxo de trabalho completo em [`.github/workflows/build.yml`](../.github/workflows/build.yml). A cada commit ou push para a branch `main`:
1. Compila o binário para **Linux** (`x86_64-unknown-linux-musl`).
2. Compila o executável para **Windows** (`rusty_hp49_windows_x64.exe`).
3. Compila o binário Universal para **macOS** (Intel + Apple Silicon).
4. Compila o pacote instalável **Android APK** (`rusty_hp49_android.apk`).
5. Empacota o bundle **iOS para iPhone**.
Todos os executáveis ficam disponíveis na aba **Actions / Artifacts** do GitHub!
