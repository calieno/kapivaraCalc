# Relatório de Engenharia Reversa - HP 49G Emulator (`YUser.exe` & `rom.e49`)

**Ferramenta utilizada:** REA (*Reverse Engineer Anything*) & Antigravity Analysis Framework  
**Alvo Principal:** `YUser.exe` (PE32 x86 GUI), `YUserfast.exe`, `Yorkem.dll`, `YMHelper.dll`  
**Alvo ROM:** `rom.e49` (HP 49G ROM VGER v1.19-6)  
**Data da Análise:** Outubro de 2026  

---

## 1. Visão Geral do Alvo

O emulador oficial da calculadora HP 49G distribuído pela Hewlett-Packard em 1999 é composto por:
1. **`YUser.exe` / `YUserfast.exe`**: Executável de interface gráfica Win32 (MFC 4.2 / MSVC 6.0), responsável pela janela em formato de calculadora, carregamento do bitmap de fundo (`HP49.BMP`), captura de eventos de mouse e teclado, desenho do LCD e comunicação serial virtual.
2. **`Yorkem.dll`**: Núcleo do emulador de hardware. Emula a CPU **Saturn** (arquitetura de 4 bits / 64 bits de registros da HP), controlador de interrupções, temporizadores de hardware e registradores de barramento.
3. **`YMHelper.dll`**: Camada de suporte de memória, carregamento e paginação dos bancos de ROM/RAM (Port 0: 256KB, Port 1: 128KB, Port 2: 128KB) e rotinas de transferência de arquivos (Kermit / XModem).
4. **`HP49.BMP`**: Skin de alta definição (347 × 751 pixels, 24 bpp BGR). As linhas 0 a 739 contêm o chassi da calculadora; as linhas 740 a 751 contêm os glifos dos annunciators (indicadores LCD).
5. **`rom.e49`**: Imagem da ROM da HP 49G com 4.194.304 bytes (4 MiB).

---

## 2. Análise do Executável `YUser.exe`

### 2.1 Metadados PE e Headers
- **Tipo:** Portable Executable 32-bit (PE32) para plataforma Intel 80386 (x86).
- **Subsystem:** Windows GUI (`0x0002`).
- **Timestamp de Compilação:** `0x37DFBEB5` -> Quarta-feira, 15 de Setembro de 1999, 15:47:33 UTC.
- **Entry Point:** `0x00403756`.
- **Seções:**
  - `.text`: `0x00401000` - `0x00409000` (Código x86 MSVC 6.0 / MFC 4.2)
  - `.rdata`: `0x00409000` - `0x0040C000` (Tabelas de IAT, RTTI, constantes)
  - `.data`: `0x0040C000` - `0x00435000` (Variáveis globais, buffers DIB, estados de tecla)
  - `.rsrc`: `0x00435000` - `0x00436000` (Ícone `IDR_MAINFRAME`, aceleradores)

### 2.2 Diferenças entre `YUser.exe` e `YUserfast.exe`
A comparação de binários via REA revelou apenas uma instrução divergente no ponto de configuração do emulador:
- `YUser.exe` em `0x00401A24`: invoca `_YMConfigure(1, 1400)` -> Emulação com temporização real ajustada a ~4 MHz (ciclo Saturn original).
- `YUserfast.exe` em `0x00401A24`: invoca `_YMConfigure(2, 1)` -> Emulação em modo turbo / máxima velocidade da CPU hospedeira.

### 2.3 Janela Principal e Skinning
- A janela é criada com classe personalizada `YUserClass` e estilo `WS_POPUP | WS_VISIBLE | WS_SYSMENU`.
- Não possui bordas padrão do Windows (`WS_BORDER` / `WS_CAPTION` desabilitados), com tamanho fixo de **347 × 740 pixels**.
- O bitmap `HP49.BMP` é carregado e renderizado via `BitBlt` em um HDC de memória em background.
- **Translucidez ao Pressionar Teclas:**
  Quando o usuário clica ou pressiona uma tecla, a rotina em `0x00402170` itera pelos pixels da bounding box da tecla (com cantos arredondados de raio 5px) e aplica a fórmula de blend:
  $$\text{Pixel}_{\text{highlight}} = \left( \frac{R}{2} + 64, \frac{G}{2} + 64, \frac{B}{2} + 64 \right)$$
  gerando o feedback visual tátil realista do botão iluminado.

### 2.4 Geometria e Mapeamento da Matriz de Teclas (54 Teclas)
Foram mapeadas 54 teclas físicas com suas coordenadas absolutas $(x, y, w, h)$, raio e mapeamento matricial Saturn (Linha, Coluna):

| Tecla / Função | X | Y | Largura | Altura | Row | Col | Virtual Key (VK) |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **F1 (A)** | 42 | 213 | 37 | 20 | 0 | 0 | VK_F1 |
| **F2 (B)** | 87 | 213 | 37 | 20 | 0 | 1 | VK_F2 |
| **F3 (C)** | 132 | 213 | 37 | 20 | 0 | 2 | VK_F3 |
| **F4 (D)** | 177 | 213 | 37 | 20 | 0 | 3 | VK_F4 |
| **F5 (E)** | 222 | 213 | 37 | 20 | 0 | 4 | VK_F5 |
| **F6 (F)** | 267 | 213 | 37 | 20 | 0 | 5 | VK_F6 |
| **APPS** | 42 | 240 | 37 | 20 | 1 | 0 | 'A' |
| **MODE** | 87 | 240 | 37 | 20 | 1 | 1 | 'M' |
| **TOOL** | 132 | 240 | 37 | 20 | 1 | 2 | 'T' |
| **D-Pad UP** | 241 | 239 | 24 | 19 | 1 | 4 | VK_UP |
| **D-Pad LEFT** | 205 | 258 | 24 | 19 | 2 | 3 | VK_LEFT |
| **D-Pad DOWN**| 241 | 277 | 24 | 19 | 3 | 4 | VK_DOWN |
| **D-Pad RIGHT**| 277 | 258 | 24 | 19 | 2 | 5 | VK_RIGHT |
| **ENTER** | 42 | 585 | 37 | 42 | 7 | 0 | VK_RETURN |
| **ON** | 42 | 689 | 37 | 20 | 8 | 0 | VK_F12 |
| **Left Shift (Red/Orange)** | 42 | 481 | 37 | 20 | 5 | 0 | VK_SHIFT |
| **Right Shift (Cyan/Blue)** | 42 | 533 | 37 | 20 | 6 | 0 | VK_CONTROL |
| **ALPHA** | 98 | 481 | 37 | 20 | 5 | 1 | VK_CAPITAL |
| ... *(demais teclas numéricas, operadores +, -, *, /, HIST, EVAL, SYMB)* | | | | | | | |

---

## 3. Análise da ROM `rom.e49`

### 3.1 Formato de Empacotamento de Nibbles
A ROM possui exatamente 4.194.304 bytes. Na arquitetura Saturn da HP, a memória é orientada a *nibbles* (4 bits).  
O arquivo `rom.e49` utiliza o esquema de 1 nibble por byte (os 4 bits menos significativos contêm o dado, e os 4 bits superiores são zero).  
Ao compactar dois bytes consecutivos:
$$\text{Byte} = (b_0 \ \& \ \text{0x0F}) \mid ((b_1 \ \& \ \text{0x0F}) \ll 4)$$
obtemos a imagem binária real de **2.097.152 bytes (2 MiB)** da memória Flash da HP 49G.

### 3.2 Identificação e Assinatura
- Versão: `Version HP49-C Revision #1.19-6` (codinome de desenvolvimento "VGER").
- Autores identificados nos strings da ROM:
  - Bernard Parisse (autor do motor algébrico simbólico **Erable CAS**)
  - Mika Heiskanen (bibliotecas de gráficos e algoritmos matemáticos)
  - Claude-Nicolas Fiechter
  - Hewlett-Packard Corvallis & ACO Team (França)

### 3.3 Módulos Matemáticos Recuperados
1. **Erable CAS (Computer Algebra System):**
   - Resolução simbólica de equações lineares e não-lineares (`SOLVE`, `ZEROS`).
   - Diferenciação simbólica (`DERIV`) e integração (`INT`, `RISCH`).
   - Expansão de polinômios e fatoração (`EXPAND`, `FACTOR`, `COLLECT`).
   - Decomposição em frações parciais (`PARTFRAC`).
   - Séries de Taylor e limites (`TAYLOR`, `SERIES`, `LIMIT`).
2. **Biblioteca de Matrizes e Vetores:**
   - Determinante (`DET`), Inversa (`INV`), Transposta (`TRAN`).
   - Decomposição LU, Cholesky, QR e SVD.
   - Autovalores e Autovetores (`EGV`, `EGVL`).
   - Resolução de sistemas de equações lineares $A \cdot X = B$.
3. **Equações Financeiras (TVM - Time Value of Money):**
   - $N$ (Períodos), $I\%YR$ (Taxa de juros anual), $PV$ (Valor presente), $PMT$ (Pagamento/prestação), $FV$ (Valor futuro).
   - Modo Antecipado / Postecipado (*Begin / End*).
   - Amortização e cálculo de TIR / VPL (*NPV / IRR*).
4. **Equation Library & Constantes Físicas:**
   - 315 constantes físicas padronizadas (SI, Imperial): velocidade da luz $c$, constante de Planck $h$, gravitação $G$, carga elementar $e$, constante de Boltzmann $k$, constante dos gases $R$, número de Avogadro $N_A$, etc.
   - Conversor multidimensional de unidades (Comprimento, Área, Volume, Pressão, Energia, Potência, Temperatura).
5. **Estatística e Probabilidade:**
   - Combinações ($nCr$), Permutações ($nPr$), Fatorial ($n!$).
   - Distribuição Normal (PDF e CDF), t-Student, Chi-quadrado, Fisher-Snedecor.
   - Regressão linear, exponencial, logarítmica e potência.

---

## 4. Reconstrução do Código-Fonte (`hp49_reconstructed`)

Para preservar o projeto original e permitir sua compilação em ambientes modernos, os seguintes arquivos foram reconstruídos:
- `include/YUser.h`: Declaração dos 54 botões, estruturas de mensagens Win32, constantes de interface e protótipos de rotinas de desenho.
- `include/YorkeM.h`: Protótipos das 38 funções exportadas pela `Yorkem.dll` (emulação do processador Saturn).
- `include/YMHelper.h`: Protótipos das 38 funções da `YMHelper.dll` (gerenciador de memória, display DIB e portas de I/O).
- `src/YUser.cpp`: Implementação completa da janela Win32, laço de mensagens, processamento de cliques do mouse com cálculo de distância euclidiana para cantos arredondados, efeito de brilho translúcido, renderização de LCD e acoplamento com a máquina Saturn.
- `CMakeLists.txt` e `Makefile`: Scripts de compilação prontos para MinGW (Windows) e Linux.

---

## 5. Implementação Moderna em Rust (`rusty_hp49`)

Com base nas equações, algoritmos e matriz de teclas extraídos da ROM e da interface, implementou-se a calculadora moderna **Rusty HP-49G**:
- **Design Elegante e Moderno:** Interface visual dark glassmorphism/neumorphic inspirada na HP 49G e nas diretrizes de design contemporâneas.
- **Display RPN de 4 Níveis:** Pilha clássica RPN com níveis Z, Y, X e T, exibição de modos angulares (RAD/DEG/GRAD), base numérica (DEC/HEX/BIN/OCT) e annunciators visuais.
- **Motor Matemático Integrado:** Todo o poder algébrico da HP 49G (aritmética de precisão, matrizes, cálculo diferencial e integral, TVM financeiro, constantes físicas e conversores de unidade) integrado em um único arquivo Rust (`main.rs`).
- **Suporte Multiplataforma Completo:** Configurado para Windows, macOS, Linux, Android (APK) e iOS (iPhone).
