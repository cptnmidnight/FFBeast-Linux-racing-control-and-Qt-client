# 🎯 Plano de Migração Exaustivo: FFBeast UI (Vite + TS + Vue)

Este documento é o roteiro de implementação pixel-perfect para o novo configurador. A arquitetura segue a separação estrita de arquivos e **Modularização Mandatória** conforme as diretrizes do projeto.

---

## 🏗️ 1. Infraestrutura e Comportamento Core
- [x] **Ambiente**: Setup Vite + Vue 3 + TypeScript em `apps/configurator`. ✅
- [x] **Modularização Estrita**:
    - [x] Cada interface/classe em seu próprio arquivo. ✅
    - [x] Estrutura inicial em `src/models/`:
        - `FirmwareVersion.ts`, `HardwareStatus.ts`, `EffectSettings.ts`, `HardwareSettings.ts`, `GpioSettings.ts`. ✅
- [x] **Conexão Automática (Handshake)**:
    - [x] `src/services/hardware_service.ts`: Ponte de comando Tauri.
    - [x] `src/stores/hardware.ts`: Gestão de estado reativo via Pinia.
    - [x] Serviço de polling que tenta conexão a cada 2s se desconectado.
    - [x] Lógica de handshake para validar o hardware e carregar as configurações iniciais.
- [x] **i18n**: `src/locales/en.json` e `src/locales/pt-BR.json`.
- [x] **Fonts (100% Offline)**: Outfit e JetBrains Mono instaladas via `@fontsource`.

---

## 📊 2. Monitor Tab (O Coração da UI)
### Visualização Reativa (60FPS Target)
- [x] **Componente**: `src/components/tabs/MonitorTab.vue`.
- [x] **WheelVisual**: `src/components/monitor/WheelVisual.vue` (SVG com rotação dinâmica).
- [x] **AnalogMonitor**: `src/components/monitor/AnalogMonitor.vue` (Grade de barras verticais).
- [x] **TorqueIndicator**: `src/components/monitor/TorqueIndicator.vue`.
- [x] **ButtonsDisplay**: `src/components/monitor/ButtonsGrid.vue`.

### Painel de Log (Monitor Widget)
- [x] **LogsWidget**: `src/components/monitor/LogsWidget.vue`.
    - [x] Estilo "In-Card" com auto-scroll e cores por nível.

---

## 🎮 3. Detalhamento das Abas de Configuração

### 🏎️ Efeitos (Effects Tab)
- [x] **Componente**: `src/components/tabs/EffectsTab.vue`.
- [x] Mapeamento de Sliders (Motion Range, Strength, Spring, Dampening, DirectX).
- [x] Checkbox de inversão de força.

### ⚙️ Motor (Hardware Tab)
- [x] **Componente**: `src/components/tabs/HardwareTab.vue`.
- [x] Mapeamento de controles de Motor (Power, Braking, Poles, Calibration).
- [x] Mapeamento de Flags (Encoder Dir, Force Dir, Debug Torque).
- [x] Configuração PID (P Gain, I Gain).

---

## 🧩 4. Componentes Base (Shared UI)
- [x] **BaseSlider**: `src/components/common/BaseSlider.vue`.
- [x] **BaseSwitch**: `src/components/common/BaseSwitch.vue`.
- [x] **BaseCard**: `src/components/common/BaseCard.vue`.
- [x] **BaseSelect**: `src/components/common/BaseSelect.vue`.

---

## 🛠️ 5. Outras Funcionalidades
- [x] **Protocol Selector**: `src/components/tabs/ProtocolTab.vue`.
- [x] **Pin Matrix**: `src/components/tabs/PinsTab.vue`.
- [x] **License Management**: `src/components/tabs/LicenseTab.vue`.
- [x] **FFB Test (Tools)**: `src/components/tabs/ToolsTab.vue`.

---

## 🚦 Critérios de Aceite Pixel-Perfect
1. **Scrollbar**: Réplica exata via `src/styles/scrollbar.css`.
2. **Offline Total**: Zero chamadas externas de assets/fontes.
3. **Modularização**: Nenhum arquivo deve conter mais de uma interface pública ou componente complexo.
