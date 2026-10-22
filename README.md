# HP 49G - Reconstrução Histórica (C/C++) & Nova Calculadora em Rust (Rusty HP-49G)

<p align="center">
  <b>Engenharia reversa completa do emulador oficial HP 49G (<code>YUser.exe</code> & <code>rom.e49</code>) e recriação moderna em Rust com suporte multiplataforma.</b>
</p>

---

## 📑 Sumário

1. [Visão Geral do Projeto](#-visão-geral-do-projeto)
2. [Parte 1: Engenharia Reversa & Reconstrução C/C++ (`hp49_reconstructed`)](#1️⃣-parte-1-engenharia-reversa--reconstrução-cc-hp49_reconstructed)
3. [Parte 2: Recriação Moderna em Rust (`rusty_hp49`)](#2️⃣-parte-2-recriação-moderna-em-rust-rusty_hp49)
4. [Estrutura do Repositório](#-estrutura-do-repositório)
5. [Compilação & Executáveis (Windows, Mac, Linux, Android, iOS)](#-compilação--executáveis)
6. [Integração Contínua (GitHub Actions)](#-integração-contínua-github-actions)

---

## 🎯 Visão Geral do Projeto

Este repositório cumpre dois objetivos principais:
1. **Engenharia Reversa com REA (*Reverse Engineer Anything*):**
   Desmontagem e reconstrução do código-fonte C/C++ do emulador oficial da Hewlett-Packard (`YUser.exe`, `Yorkem.dll`, `YMHelper.dll`) e extração do motor algébrico da ROM de 2 MiB Flash (`rom.e49`).
2. **Nova Calculadora Rusty HP-49G:**
   Recriação da HP 49G em **Rust**, integrando todas as rotinas matemáticas em um **único arquivo de código-fonte** (`main.rs`), com design elegante neumorphic/glassmorphic, e empacotamento para **Windows, macOS, Linux, Android (APK) e iPhone (iOS)**.

---

## 1️⃣ Parte 1: Engenharia Reversa & Reconstrução C/C++ (`hp49_reconstructed`)

Localizado em [`hp49_reconstructed/`](hp49_reconstructed/):
- **Código Reconstruído:**
  - `include/YUser.h`: Estrutura da janela Win32, mapeamento dos 54 botões físicos, matriz de linhas/colunas do Saturn e aceleradores.
  - `include/YorkeM.h`: 38 funções exportadas pela `Yorkem.dll` (emulação do processador Saturn, registradores de 64 bits, temporizadores e MMU).
  - `include/YMHelper.h`: 38 funções exportadas pela `YMHelper.dll` (gerenciador de memória Flash, Port0, Port1, Port2 e comunicação serial Kermit).
  - `src/YUser.cpp`: Implementação completa da janela Win32, laço de mensagens, processamento de cliques do mouse com cálculo de cantos arredondados, efeito de brilho translúcido original $(R/2 + 64, G/2 + 64, B/2 + 64)$ e desenho do display LCD.
- **Relatório Completo:**
  Consulte [`hp49_reconstructed/REVERSE_ENGINEERING_REPORT.md`](hp49_reconstructed/REVERSE_ENGINEERING_REPORT.md) para análise dos metadados PE, desempacotamento de nibbles da ROM `rom.e49` e tabela matricial das 54 teclas.

---

## 2️⃣ Parte 2: Recriação Moderna em Rust (`rusty_hp49`)

Localizado em [`rusty_hp49/`](rusty_hp49/):
- **Single-File Architecture:** Toda a pilha matemática, servidor HTTP embutido e interface gráfica estão condensados em [`rusty_hp49/src/main.rs`](rusty_hp49/src/main.rs).
- **Design Elegante & Moderno:**
  - Chassi de titânio escuro com display OLED/LCD de alta luminescência.
  - Annunciators dinâmicos (`RAD`, `DEG`, `HEX`, `DEC`, `SHIFT`, `ALPHA`, `BUSY`).
  - Pilha RPN de 4 níveis (T, Z, Y, X) com histórico infinito de `UNDO`.
  - Softkeys interativas F1 a F6 com submenus dinâmicos (`MATH`, `CAS`, `MATR`, `TVM`, `STAT`, `CONST`, `UNIT`).
- **Funções Matemáticas Integradas:**
  - Aritmética e funções transcendentais (trigonométricas, hiperbólicas, logaritmos, potências).
  - Álgebra Linear: Determinante, Inversa, Transposta e resolução de sistemas lineares.
  - Cálculo Numérico: Derivada $f'(x)$, Segunda Derivada $f''(x)$, Integral Definida adaptativa e Taylor.
  - Solução de Raízes Polinomiais: Algoritmo Durand-Kerner (raízes reais e complexas).
  - Solver Financeiro TVM (Time Value of Money): $N$, $I\%YR$, $PV$, $PMT$, $FV$.
  - Tabela de 14 constantes físicas fundamentais e conversor de unidades de 6 categorias.

---

## 📂 Estrutura do Repositório

```text
├── .github/
│   └── workflows/
│       └── build.yml             # Workflow de CI que compila Windows, Linux, Mac, Android e iOS
├── hp49_reconstructed/           # Código-fonte C/C++ reconstruído
│   ├── CMakeLists.txt            # Build moderno CMake
│   ├── Makefile                  # Build MinGW / GCC
│   ├── README.md
│   ├── REVERSE_ENGINEERING_REPORT.md
│   ├── include/
│   │   ├── YUser.h
│   │   ├── YorkeM.h
│   │   └── YMHelper.h
│   └── src/
│       └── YUser.cpp
├── rusty_hp49/                   # Calculadora moderna em Rust (Single-File)
│   ├── Cargo.toml
│   ├── README.md
│   ├── build_linux.sh            # Script de compilação Linux
│   ├── build_windows.bat         # Script de compilação Windows
│   ├── build_android.sh          # Script de compilação Android APK
│   ├── src/
│   │   └── main.rs               # CÓDIGO-FONTE ÚNICO (Motor + Servidor + UI + PWA)
│   ├── dist/
│   │   └── rusty_hp49_linux_x86_64 # Executável estático Linux compilado e testado
│   ├── android/                  # Projeto Android nativo (APK)
│   └── ios/                      # Projeto iOS nativo (iPhone)
├── HP49.BMP                      # Skin original HP 49G (347x751 px)
├── rom.e49                       # ROM original HP 49G VGER v1.19-6
├── YUser.exe                     # Binário original analisado
├── YUserfast.exe                 # Binário original turbo
├── Yorkem.dll                    # DLL do emulador Saturn original
└── YMHelper.dll                  # DLL de suporte de memória original
```

---

## 🚀 Compilação & Executáveis

### Executando a Calculadora em Rust Localmente:
```bash
# Modo Interface Moderna (Abre navegador automaticamente em http://localhost:4949)
cargo run --release

# Modo Terminal CLI / RPN Interativo
cargo run --release -- --cli
```

### Compilando para Cada Plataforma:
- **Linux:** Executar `./rusty_hp49/build_linux.sh` -> Gera `dist/rusty_hp49_linux_x86_64`.
- **Windows:** Executar `rusty_hp49/build_windows.bat` -> Gera `dist/rusty_hp49_windows_x64.exe`.
- **Android APK:** Executar `./rusty_hp49/build_android.sh` -> Gera `dist/rusty_hp49.apk`.
- **iPhone / iOS:** Abrir pasta `rusty_hp49/ios/` no Xcode ou utilizar instalação PWA de 1 clique no Safari!

---

## 🔄 Integração Contínua (GitHub Actions)

O workflow [`.github/workflows/build.yml`](.github/workflows/build.yml) compila automaticamente em máquinas limpas na nuvem a cada commit:
- 🪟 Windows `x86_64-pc-windows-msvc`
- 🐧 Linux `x86_64-unknown-linux-musl`
- 🍏 macOS Universal Binary (`x86_64` + `aarch64` Apple Silicon)
- 🤖 Android APK (`rusty_hp49_android.apk`)
- 📱 iPhone iOS Bundle

Todos os arquivos compilados ficam disponíveis diretamente para download na aba **Actions** do seu repositório GitHub!
