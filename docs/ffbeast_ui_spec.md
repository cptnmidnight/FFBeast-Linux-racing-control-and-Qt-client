# Especificação Geral: FFBeast UI (Configurator)

O **FFBeast UI** é a aplicação de configuração e monitoramento para o hardware de Force Feedback "FFBeast". Desenvolvido sobre a plataforma **Tauri v2**, utiliza tecnologias web padrão (HTML5, CSS3, ES6 Modules) para a interface e **Rust** para o backend de comunicação.

![Logo](../img/logo_app.png)

## Arquitetura

- **Frontend**: Single Page Application (SPA) sem frameworks pesados (Vanilla JS).
  - **Componentes**: Arquitetura modular baseada em classes (`MonitorComponent`, `HardwareTab`, etc.).
  - **Estilização**: CSS modular (`main.css`, `components.css`, `monitor.css`, etc.) com variáveis CSS para temas.
  - **Internacionalização**: Sistema próprio (`i18n.js`) suportando EN e PT-BR.
  - **Comunicação**: IPC via Tauri Commands para o backend Rust.

- **Backend (Tauri/Rust)**: 
  - Gerencia janela, sistem tray e comunicação HID.
  - Expõe comandos como `connect`, `get_hardware_settings`, `save_settings`.

## Funcionalidades Funcionais

### 1. Monitoramento (Dashboard)
- Visualização em tempo real da posição do volante (Animação SVG).
- Barra de Torque/Force Feedback ao vivo.
- Status do Firmware (Versão, Conexão).
- Status de Botões (Grid visual).
- Status de Eixos Analógicos (ADC).

### 2. Configuração de Hardware
- **Motor e Encoder**: Definição de CPR (Pulsos por rotação), Limite de Potência (%), Partes de Polos.
- **Calibração**: Reset de Centro, Calibração de ADC (Min/Max).

### 3. Efeitos de Força (FFB)
- Ajuste de ganhos globais e específicos.
- **Configurações**:
  - Total Strength (Ganho Geral)
  - Motion Range (Rotação em graus)
  - Damper, Friction, Inertia (Efeitos dinâmicos)
  - Spring, Constant, Periodic (Efeitos DirectX)

### 4. Pinos e Expansão (GPIO)
- Configuração de pinos do microcontrolador.
- Modos suportados: Botão, Eixo Analógico, Shifter SPI, PWM Freio, etc.
- Matriz de botões.

### 5. Ferramentas (Tools)
- Testes manuais de FFB (Sliders de força constante, senoide, etc.).
- Atualização de Firmware (DFU Mode).
- Logs do Sistema (Debug Console).

### 6. Sistema e Licença
- Ativação de licença via Serial Key.
- Exibição de Device ID único.
- Configurações do App (Tema, Idioma, Auto-scroll).

## Interface de Usuário (UX)

- Design Escuro (Dark Mode) com acentos em verde/roxo/azul configuráveis.
- Navegação lateral (Sidebar) com ícones.
- Feedback visual instantâneo (Toasts).
- Responsividade ajustada para janelas de desktop (min 1000x750).

## Estrutura de Pastas

```
apps/configurator/src
├── assets/          # Ícones, Fontes, Imagens
├── i18n/            # Arquivos JSON de tradução
├── js/
│   ├── components/  # Lógica de cada aba (HardwareTab.js, etc.)
│   ├── services/    # Singletons (HardwareService.js, i18n.js)
│   └── utils/       # Helpers (Logger.js, ui.js)
├── styles/          # CSS fragmentado
└── index.html       # Ponto de entrada
```
