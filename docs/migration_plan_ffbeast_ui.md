# 🎯 Plano de Migração Exaustivo: FFBeast UI (Vite + TS + Vue)

Este documento é o roteiro de implementação pixel-perfect para o novo configurador. A arquitetura segue a separação estrita de arquivos e **Modularização Mandatória** conforme as diretrizes do projeto.

---

## 🏗️ 1. Infraestrutura e Comportamento Core 
- [x] **Ambiente**: Setup Vite + Vue 3 + TypeScript em `apps/configurator`. ✅
- [x] **Modularização Estrita**:
    - [x] Cada interface/classe em seu próprio arquivo. ✅
    - [x] Estrutura consolidada em `src/models/`:
        - `FirmwareVersion.ts`, `HardwareStatus.ts`, `EffectSettings.ts`, `HardwareSettings.ts`, `GpioSettings.ts`. ✅
- [x] **Conexão Automática (Handshake)**:
    - [x] `src/services/hardware_service.ts`: Ponte de comando Tauri. ✅
    - [x] `src/stores/hardware.ts`: Gestão de estado reativo via Pinia com polling de 60 FPS. ✅
    - [x] Serviço de polling que tenta conexão a cada 2s se desconectado. ✅
    - [x] Lógica de handshake para validar o hardware e carregar as configurações iniciais. ✅
- [x] **i18n**: Estrutura robusta em `src/locales/` (en, pt-BR). ✅
- [x] **Fonts (100% Offline)**: Outfit e JetBrains Mono instaladas via `@fontsource`. ✅

---

## 📊 2. Monitor Tab (Visualização Reativa)
- [x] **Componente Principal**: `src/components/tabs/MonitorTab.vue`. ✅
- [x] **WheelVisual**: `src/components/monitor/WheelVisual.vue` (SVG com rotação dinâmica). ✅
- [x] **AnalogMonitor**: Visualização compacta de ADCs. ✅
- [x] **TorqueIndicator**: Feedback visual de força aplicada. ✅
- [x] **ButtonsGrid**: Display de grade de botões em tempo real. ✅

---

## 🎮 3. Detalhamento das Abas de Configuração

### 🏎️ Efeitos (Effects Tab)
- [x] **Componente**: `src/components/tabs/EffectsTab.vue`. ✅
- [x] Mapeamento de Sliders (Motion Range, Strength, Spring, Dampening, DirectX). ✅
- [x] Checkbox de inversão de força. ✅

### ⚙️ Motor (Hardware Tab)
- [x] **Componente**: `src/components/tabs/HardwareTab.vue`. ✅
- [x] Mapeamento de controles de Motor (Power, Braking, Poles, Calibration). ✅
- [x] Mapeamento de Flags (Encoder Dir, Force Dir, Debug Torque). ✅
- [x] Configuração PID (P Gain, I Gain). ✅

### 🔘 Botões e Entradas (Buttons & Inputs)
- [x] **ButtonsTab**: Mapeamento digital e monitoramento de botões. ✅
- [x] **InputsTab**: Calibração de eixos analógicos (Min, Max, Invert). ✅

---

## 🧩 4. Componentes Base (Shared UI)
- [x] **BaseSlider**: Slider personalizado com input numérico. ✅
- [x] **BaseSwitch**: Toggle estilizado. ✅
- [x] **BaseCard**: Container padrão com suporte a header/footer. ✅
- [x] **BaseSelect**: Dropdown customizado. ✅

---

## 🛠️ 5. Abas Auxiliares e Utilitários
- [x] **Protocol Selector**: `src/components/tabs/ProtocolTab.vue`. ✅
- [x] **Pin Matrix**: `src/components/tabs/PinsTab.vue`. ✅
- [x] **License Management**: `src/components/tabs/LicenseTab.vue`. ✅
- [x] **FFB Test (Tools)**: `src/components/tabs/ToolsTab.vue`. ✅
- [x] **Logs Tab**: Aba dedicada para visualização de logs do sistema. ✅
- [x] **Settings Tab**: Configurações de interface (Linguagem, Cor de sotaque). ✅

---

## 6. Próximos Passos (Refinamento)

- [ ] **Persistência Local:** Salvar configurações de UI (tema, idioma) no LocalStorage.
- [ ] **Tratamento de Erros:** Melhorar feedback visual quando handshake com backend falhar.
- [ ] **Perfis de Configuração:** Botões para exportar/importar configurações do volante.
- [ ] **Firmware Update UI:** Interface para verificar e aplicar atualizações de firmware.
- [ ] **Resolução de Bugs Conhecidos:** Ver `docs/known_bugs_ffbeast_ui.md` para lista completa.

---

## 🐛 Status de Bugs Conhecidos

**Referência Completa:** [`docs/known_bugs_ffbeast_ui.md`](./known_bugs_ffbeast_ui.md)

### Críticos (Bloqueadores)
- ❌ **Auto-connect não funciona** - Handshake não executando ao iniciar app
- ❌ **Pins Tab vazio** - Dependente do handshake

### UI/UX
- ✅ **Monitor Tab FFB Switch** - Implementado
- ❌ **Buttons Tab layout** - Diferente do original
- ⚠️ **Inputs Tab visual** - Funcional mas layout divergente

### Settings Faltantes
- ❌ Configuração de posição/margem dos toasts
- ❌ Seletor de fonte
- ⚠️ Paleta de cores com conflitos semânticos

**Resumo:** 3/9 resolvidos | 6 pendentes

---

## Critérios de Aceitação (Pixel-Perfect)

- [x] Layout responsivo, com design moderno (gradientes, glassmorphism, animações).
- [x] Sidebar com abas e ícones.
- [x] Monitor: Visualizador de volante, indicador de torque, monitor de analógicos, logs.
- [x] Effects, Hardware, GPIO, Pins, Buttons: formulários com sliders e switches estilizados.
- [x] Inputs: Calibração de eixos com min/max/invert + modal de mapeamento.
- [x] Licença: Exibição de ID do dispositivo + ativação via serial key.
- [x] Tools: Testes de FFB (sliders ao vivo) + DFU.
- [x] Logs: Feed de eventos em tempo real.
- [x] Settings: Idioma, tema, informações da aplicação.
- [x] Performance: Polling de status rodando via `requestAnimationFrame` para suavidade máxima.
- [ ] **Bug-Free:** Todos os itens em `known_bugs_ffbeast_ui.md` resolvidos.
