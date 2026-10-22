# kapivaraCalc (kC49) & Engenharia Reversa HP 49G

<p align="center">
  <b>kapivaraCalc (kC49): Calculadora Científica e Gráfica Moderna em Rust com Aplicativo Nativo de Desktop e Multiplataforma.</b><br>
  <i>Inclui engenharia reversa completa do emulador oficial da HP 49G (<code>YUser.exe</code> & <code>rom.e49</code>).</i>
</p>

---

## 📑 Sumário

1. [Visão Geral do Projeto](#-visão-geral-do-projeto)
2. [Parte 1: kapivaraCalc (kC49) em Rust](#1️⃣-parte-1-kapivaracalc-kc49-em-rust)
3. [Parte 2: Reconstrução C/C++ da HP 49G (`hp49_reconstructed`)](#2️⃣-parte-2-reconstrução-cc-da-hp-49g-hp49_reconstructed)
4. [Estrutura do Repositório](#-estrutura-do-repositório)
5. [Compilação & Executáveis (Windows, Mac, Linux, Android, iOS)](#-compilação--executáveis)
6. [Integração Contínua (GitHub Actions)](#-integração-contínua-github-actions)

---

## 🎯 Visão Geral do Projeto

Este repositório reúne:
1. **kapivaraCalc (kC49):**
   Nova calculadora científica e simbólica desenvolvida em **Rust** em um **único arquivo de código-fonte** (`kC49/src/main.rs`). Possui design moderno em titânio e tela OLED, executando como um **aplicativo de desktop com vida própria** (janela dedicada independente, sem barras de navegador) no Windows, macOS e Linux, além de aplicativo nativo para **Android (`.apk`)** e **iPhone (iOS)**.
2. **Reconstrução Histórica C/C++ (`hp49_reconstructed`):**
   Engenharia reversa com a ferramenta REA (*Reverse Engineer Anything*) do emulador oficial da Hewlett-Packard de 1999 (`YUser.exe`, `Yorkem.dll`, `YMHelper.dll`) e extração do motor algébrico da ROM Flash de 2 MiB (`rom.e49`).

---

## 1️⃣ Parte 1: kapivaraCalc (kC49) em Rust

Localizado em [`kC49/`](kC49/):
- **Aplicativo Desktop com Vida Própria:** No Windows, ao executar `kC49.exe`, o programa abre em uma **janela desktop dedicada e exclusiva** (sem abas de navegador, sem barra de URL, com tamanho de 435 × 845 px).
- **Single-File Architecture:** Toda a pilha matemática, motor RPN, álgebra linear, cálculo CAS, finanças TVM, conversores de unidades e interface gráfica estão condensados em [`kC49/src/main.rs`](kC49/src/main.rs).
- **Display OLED & Pilha RPN de 4 Níveis:** Níveis T, Z, Y, X com histórico infinito de `UNDO`, `SWAP`, `ROLL`, `DROP` e annunciators dinâmicos (`RAD`, `DEG`, `HEX`, `DEC`, `SHIFT`, `ALPHA`, `BUSY`).
- **Softkeys Contextuais F1 a F6:** Menus rápidos para funções transcendentais (`MATH`), cálculo diferencial e integral (`CAS`), matrizes (`MATR`), finanças (`TVM`), estatística (`STAT`), constantes fundamentais (`CONST`) e conversões de unidades (`UNIT`).

---

## 2️⃣ Parte 2: Reconstrução C/C++ da HP 49G (`hp49_reconstructed`)

Localizado em [`hp49_reconstructed/`](hp49_reconstructed/):
- Código reconstruído fiel ao original da HP de 1999 (Win32, processador Saturn e barramento de memória).
- Relatório técnico detalhado em [`hp49_reconstructed/REVERSE_ENGINEERING_REPORT.md`](hp49_reconstructed/REVERSE_ENGINEERING_REPORT.md).

---

## 📂 Estrutura do Repositório

```text
├── .github/
│   └── workflows/
│       └── build.yml             # Workflow de CI que compila Windows, Linux, Mac, Android e iOS
├── kC49/                         # Nova calculadora kapivaraCalc em Rust (Single-File)
│   ├── Cargo.toml
│   ├── README.md
│   ├── build_linux.sh            # Script de compilação Linux
│   ├── build_windows.bat         # Script de compilação Windows
│   ├── build_android.sh          # Script de compilação Android APK
│   ├── src/
│   │   └── main.rs               # CÓDIGO-FONTE ÚNICO (Motor + Janela Desktop + UI)
│   ├── dist/
│   │   └── kC49_linux_x86_64     # Executável estático Linux compilado e testado
│   ├── android/                  # Projeto Android nativo (kC49.apk)
│   └── ios/                      # Projeto iOS nativo (iPhone)
├── hp49_reconstructed/           # Código-fonte C/C++ reconstruído
│   ├── CMakeLists.txt
│   ├── Makefile
│   ├── README.md
│   ├── REVERSE_ENGINEERING_REPORT.md
│   ├── include/
│   │   ├── YUser.h
│   │   ├── YorkeM.h
│   │   └── YMHelper.h
│   └── src/
│       └── YUser.cpp
├── HP49.BMP                      # Skin original HP 49G
├── rom.e49                       # ROM original HP 49G VGER v1.19-6
├── YUser.exe                     # Binário original analisado
├── YUserfast.exe                 # Binário original turbo
├── Yorkem.dll                    # DLL do emulador Saturn original
└── YMHelper.dll                  # DLL de suporte de memória original
```

---

## 🚀 Compilação & Executáveis

### Executando Localmente:
```bash
# Executa a janela desktop do kapivaraCalc (kC49)
cargo run --release

# Modo Terminal CLI / RPN Interativo
cargo run --release -- --cli
```

### Compilando para Cada Plataforma:
- **Windows:** Executar `kC49/build_windows.bat` -> Gera `dist/kC49.exe`.
- **Linux:** Executar `./kC49/build_linux.sh` -> Gera `dist/kC49_linux_x86_64`.
- **Android APK:** Executar `./kC49/build_android.sh` -> Gera `dist/kC49.apk`.
- **iPhone / iOS:** Abrir pasta `kC49/ios/` no Xcode ou utilizar instalação PWA de 1 clique no Safari!

---

## 🔄 Releases Automáticas (GitHub Actions)

A cada tag criada no Git (ex: `v0.0.1`, `v0.0.2`), o GitHub Actions compila automaticamente e publica em [Releases](https://github.com/calieno/kapivaraCalc/releases):
* 🪟 `kC49.exe` (Windows Standalone)
* 🐧 `kC49_linux_static` e `kC49_linux_x86_64` (Linux)
* 🍏 `kC49_macos_universal` (macOS Intel + Apple Silicon)
* 🤖 `kC49.apk` (Android)
* 📱 `kC49_ios_bundle.tar.gz` (iOS)
