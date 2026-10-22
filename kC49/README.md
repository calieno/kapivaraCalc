# kapivaraCalc (kC49)

> **Calculadora científica, gráfica e simbólica moderna em Rust ("kC49") inspirada na clássica HP 49G.**  
> Todas as rotinas matemáticas, motor RPN, álgebra linear, cálculo simbólico/numérico CAS, TVM financeiro, constantes físicas, conversor de unidades e aplicativo de desktop com janela própria estão **integrados em um único arquivo de código-fonte (`src/main.rs`)**.

---

## 🚀 Destaques do Projeto

- **Aplicativo com Vida Própria (Desktop Standalone Window):**
  - No Windows, ao abrir o `kC49.exe`, o programa inicia uma **janela desktop dedicada e exclusiva** (sem abas de navegador, sem barra de endereços, sem controles externos).
  - Dimensões proporcionais à calculadora (435 × 845 px).
  - Fecha e minimiza como qualquer aplicativo nativo do sistema operacional.
- **Arquitetura 100% Single-File:** Motor matemático, pilha RPN de 4 níveis, avaliador de expressões e UI moderna estão condensados em [`src/main.rs`](src/main.rs).
- **Design Elegante & Moderno:**
  - Chassi com acabamento em liga de titânio escuro e cantos arredondados contínuos.
  - Tela OLED/LCD esmeralda de alto contraste com annunciators dinâmicos (`RAD`, `DEG`, `DEC`, `HEX`, `SHIFT`, `ALPHA`, `BUSY`).
  - Botões táteis com feedback sonoro sintético e vibração háptica no celular.
  - 6 Softkeys contextuais dinâmicas (F1 a F6) com submenus (`MATH`, `CAS`, `MATR`, `TVM`, `STAT`, `CONST`, `UNIT`).
- **Poder Matemático Herdado da ROM `rom.e49` (VGER 1.19-6):**
  - **Pilha RPN:** Níveis 1 (X), 2 (Y), 3 (Z) e 4 (T) com histórico, `SWAP`, `ROLL`, `DROP` e `UNDO`.
  - **Álgebra Linear & Matrizes:** Determinante (`DET`), Transposta (`TRAN`), Inversa (`INV`), Multiplicação de matrizes e Solução de Sistemas Lineares $A \cdot X = B$.
  - **Cálculo Numérico & CAS:** Derivadas numéricas $f'(x)$ e $f''(x)$, Integração definida adaptativa (Regra dos 3/8 de Simpson) e expansão em série de Taylor.
  - **Localizador de Raízes Polinomiais:** Algoritmo Durand-Kerner (raízes reais e complexas no plano Argand-Gauss).
  - **Finanças (TVM - Time Value of Money):** Solver completo para $N$, $I\%YR$, $PV$, $PMT$, $FV$ em modo Begin/End.
  - **Equation Library:** Constantes universais ($c$, $h$, $\hbar$, $G$, $g_0$, $e$, $m_e$, $m_p$, $k_B$, $N_A$, $R$, $\varepsilon_0$, $\mu_0$, $\sigma$).
  - **Conversor Multidimensional de Unidades:** Comprimento, Massa, Temperatura, Pressão, Energia e Potência.
  - **Modos de Base de Programador:** `DEC`, `HEX`, `OCT` e `BIN`.

---

## 💻 Como Executar

### 1. Modo Aplicativo Desktop (Janela Própria)
```bash
cargo run --release
```
Ou dê duplo clique em `kC49.exe` no Windows / execute o binário no Linux:
```bash
./dist/kC49_linux_x86_64
```
Abre instantaneamente em uma janela desktop exclusiva, sem barras de navegador.

### 2. Modo Terminal / CLI (Linha de Comando RPN)
```bash
cargo run --release -- --cli
```

---

## 📱 Suporte Multiplataforma (Windows, Mac, Linux, Android e iPhone)

### 🪟 Windows (.exe)
```cmd
build_windows.bat
```
Gera `dist/kC49.exe` e `dist/kC49_windows_x64.exe` (executável com janela desktop própria).

### 🐧 Linux
```bash
./build_linux.sh
```
Gera o binário estático `dist/kC49_linux_x86_64` (compatível com qualquer distribuição Linux).

### 🍏 macOS (Universal Binary x86_64 + Apple Silicon M1/M2/M3/M4)
```bash
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/kC49 target/aarch64-apple-darwin/release/kC49 -output dist/kC49_macos_universal
```

### 🤖 Android (APK)
```bash
./build_android.sh
```
Gera `dist/kC49.apk` pronto para instalar diretamente no smartphone Android.

### 📱 iPhone / iOS
Abra a pasta `kC49/ios/` no Xcode ou instale via Safari PWA tocando em **"Adicionar à Tela de Início"**.
