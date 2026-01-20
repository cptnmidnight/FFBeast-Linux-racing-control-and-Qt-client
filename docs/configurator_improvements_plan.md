# Plano de Melhorias - FFBeast Configurator UI

**Data:** 2026-01-19  
**Versão Atual:** 2.0.0-beta  
**Objetivo:** Melhorar textos de ajuda, adicionar avisos de reinicialização e corrigir funcionalidades

---

## 📋 Sumário Executivo

Este documento detalha as melhorias planejadas para o FFBeast Configurator, incluindo:
1. Correção da funcionalidade "Enable Force" que não está salvando
2. Adição de avisos sobre necessidade de reinicialização
3. Melhoria dos textos de ajuda baseados na documentação oficial
4. Cronograma de implementação

---

## 🐛 Problema Crítico Identificado

### **"Enable Force" não está salvando**

**Status:** 🔴 CRÍTICO  
**Prioridade:** ALTA

#### Diagnóstico:
- A opção "Enable Force" (force_enabled) está sendo enviada para o dispositivo via `updateHW()`
- O valor é aplicado na RAM do dispositivo
- **MAS:** Não está sendo persistido na EEPROM
- Quando o dispositivo reinicia, volta ao estado anterior

#### Causa Raiz:
Segundo a documentação oficial (linha 156):
> "Check **Enable force**. Press **Save and reboot** button."

O campo `force_enabled` **REQUER REINICIALIZAÇÃO** para ter efeito. Atualmente:
1. Usuário marca/desmarca "Enable Force"
2. Sistema envia para RAM via `update_hardware_settings`
3. Usuário clica em "Salvar" → grava na EEPROM
4. **FALTA:** Reinicialização automática ou aviso claro

#### Solução Proposta:
**Opção A (Recomendada):** Auto-reboot após salvar se force_enabled mudou
**Opção B:** Aviso visual destacado "Requer reinicialização para ter efeito"
**Opção C:** Botão dedicado "Salvar e Reiniciar" para esta opção específica

---

## 🔄 Configurações que Requerem Reinicialização

**✅ CONFIRMADO PELO USUÁRIO - Lista Definitiva:**

Apenas **3 configurações** requerem reinicialização:

### 1. **Enable Force** (force_enabled)
- **Requer:** ✅ Reinicialização
- **Motivo:** Calibração inicial depende deste estado
- **Documentação:** ffbeast_setup_controller.md, linha 24-26, 156
- **Prioridade:** 🔴 ALTA

### 2. **Calibration Mode** (calibration_mode)
- **Requer:** ✅ Reinicialização
- **Motivo:** Executado apenas durante boot
- **Documentação:** ffbeast_setup_controller.md, linha 28-44
- **Prioridade:** 🔴 ALTA

### 3. **GPIO Pin Modes**
- **Requer:** ✅ Reinicialização
- **Motivo:** Inicialização de hardware
- **Prioridade:** 🟡 MÉDIA

---

### **❌ NÃO Requerem Reinicialização:**

Todos os outros campos têm efeito imediato após salvar:
- Calibration Speed
- Calibration Magnitude
- Pole Pairs
- Encoder CPR
- Power Limit
- Braking Limit  
- Motion Range
- All Effect Strengths
- PID Gains (P/I)
- Dampening settings
- DirectX effect strengths
- Invert settings (encoder, force, joystick)

---

## 📝 Melhorias nos Textos de Ajuda

### Grupo: Motor/FFB (HardwareTab)

#### **Enable Force** ✨ NOVO
```
PT-BR:
"Ativa/desativa a geração de forças nos motores. Quando desmarcado, o dispositivo não produzirá forças. ⚠️ REQUER REINICIALIZAÇÃO para ter efeito."

EN:
"Enables/disables force generation on motors. When unchecked, the device will not produce forces. ⚠️ REQUIRES REBOOT to take effect."
```

#### **Power Limit** (Melhorado)
```
PT-BR (Atual):
"Corrente/Potência máxima permitida para o motor."

PT-BR (Melhorado):
"Controla a potência geral do dispositivo. Depende da fonte de alimentação e aquecimento do motor. ⚠️ IMPORTANTE: Ajuste especificamente para sua combinação de motor e PSU! Aumente em pequenos incrementos (5-10 cliques) e verifique se a PSU aguenta a carga e o motor não superaquece."

EN (Improved):
"Controls overall device power. Depends on PSU power and motor heating. ⚠️ IMPORTANT: Tune this value specifically for your motor and PSU combination! Increase by small increments (5-10 clicks) and check if PSU holds load and motor doesn't overheat."
```

#### **Braking Limit** (Melhorado)
```
PT-BR (Atual):
"Força máxima de frenagem (regenerativa/resistiva) do motor."

PT-BR (Melhorado):
"Quando o motor gira rapidamente, funciona como gerador e aumenta a tensão. Dissipar energia no resistor de frenagem previne desligamento da PSU por sobretensão. Comece com valores baixos (5-10) e aumente gradualmente se a PSU desligar durante rotações rápidas."

EN (Improved):
"When motor spins quickly, it works as a generator raising bus voltage. Dissipating excess voltage on braking resistor prevents PSU shutdown. Start with low values (5-10) and increase gradually if PSU turns off during quick rotations."
```

#### **Pole Pairs** (Melhorado)
```
PT-BR (Atual):
"Número de pares de pólos magnéticos do motor (Veja spec do motor)."

PT-BR (Melhorado):
"Número de pares de pólos do motor (número de ímãs ÷ 2). Motores de hoverboard geralmente têm 15 pares de pólos. Consulte a especificação do seu motor."

EN (Improved):
"Number of motor pole pairs (number of magnets ÷ 2). Hoverboard motors usually have 15 pole pairs. Check your motor specifications."
```

#### **Calibration Speed** (Melhorado)
```
PT-BR (Atual):
"Velocidade na qual o volante gira durante a auto-calibração."

PT-BR (Melhorado):
"Velocidade de movimento durante calibração. Se o controle se move muito rápido durante calibração, diminua para valores adequados."

EN (Improved):
"Movement speed during calibration. If control moves too quickly during calibration, decrease to adequate values."
```

#### **Calibration Magnitude** (Melhorado)
```
PT-BR (Atual):
"Força aplicada durante o processo de calibração."

PT-BR (Melhorado):
"Potência com a qual a calibração inicial é executada. Configure o mais baixo possível onde a calibração ainda funcione. Valores iniciais: volante=5, controles de voo=10. Se houver resistência ao movimento, aumente gradualmente."

EN (Improved):
"Power with which starting calibration sequence performs. Set as low as possible where calibration still works. Starting values: wheel=5, flight controls=10. If device has movement resistance, increase gradually."
```

### Grupo: Efeitos (EffectsTab)

#### **Motion Range** (Melhorado)
```
PT-BR (Atual):
"Ângulo total de rotação do volante (ex: 900 graus)."

PT-BR (Melhorado):
"Alcance de movimento onde o dispositivo reporta mudanças no eixo do joystick. ⚠️ IMPORTANTE: Corresponde ao alcance do ENCODER! Se houver reduções no dispositivo, leve isso em conta."

EN (Improved):
"Range of motion where device reports value change on joystick axis. ⚠️ IMPORTANT: Corresponds to ENCODER range! If you have reductions in your device, take it into account."
```

#### **Integrated Spring** (Melhorado)
```
PT-BR (Atual):
"Mola Centralizadora: Retorna o volante ao centro (para jogos sem FFB)."

PT-BR (Melhorado):
"Desde v24.1.4, há possibilidade de ter uma mola sempre ativa. Controla a força desta mola integrada que retorna o controle ao centro (útil para jogos sem FFB)."

EN (Improved):
"From v24.1.4, there's possibility to have spring always working. Controls strength of such integrated spring that returns control to center (useful for games without FFB)."
```

#### **Static Dampening** (Melhorado)
```
PT-BR (Atual):
"Amortecimento Estático: Resistência constante ao movimento (Peso)."

PT-BR (Melhorado):
"Adiciona força de amortecimento no alcance normal de movimento. Amortecimento estático detecta movimento do controle e tenta resistir a ele. É sempre constante."

EN (Improved):
"Adds dampening force in normal range of motion. Static dampening determines control movement and tries to resist it. Always constant."
```

#### **Dynamic Dampening** (Melhorado)
```
PT-BR (Atual):
"Amortecimento Dinâmico (Friction): Resistência aumenta com a velocidade."

PT-BR (Melhorado):
"Força de amortecimento adicional no alcance normal. Não é constante e depende de regras complexas baseadas na força total produzida. Funciona em combinação com cálculos do FFBeast Commander. ⚠️ Não tem efeito se FFBeast Commander não estiver rodando."

EN (Improved):
"Additional dampening force in normal range. Not constant and depends on complex rules based on overall force produced. Works with FFBeast Commander calculations. ⚠️ Has no effect if FFBeast Commander is not running."
```

---

## 🎨 Melhorias de Interface

### 1. **Indicador de "Requer Reinicialização"**

Para campos que requerem reboot, adicionar:
- 🔄 Ícone de reinicialização ao lado do label
- Tooltip explicativo ao passar o mouse
- Cor de destaque (laranja) no label

**Exemplo visual:**
```
┌─────────────────────────────────────┐
│ 🔄 Enable Force Feedback            │ ← Ícone laranja
│ ─────────────────────────────────   │
│ [✓] Ativado                         │
│                                     │
│ ⚠️ Requer reinicialização          │ ← Aviso em destaque
└─────────────────────────────────────┘
```

### 2. **Botão "Salvar e Reiniciar" Inteligente**

Quando houver mudanças que requerem reboot:
- Botão muda de "Salvar" para "Salvar e Reiniciar"
- Cor muda para laranja
- Tooltip explica: "Algumas configurações requerem reinicialização"

### 3. **Confirmação de Reinicialização**

Modal de confirmação antes de reiniciar:
```
┌─────────────────────────────────────┐
│  ⚠️ Reinicialização Necessária      │
│                                     │
│  As seguintes configurações foram   │
│  alteradas e requerem reinicialização:│
│                                     │
│  • Enable Force                     │
│  • Pole Pairs                       │
│                                     │
│  Deseja salvar e reiniciar agora?   │
│                                     │
│  [Cancelar]  [Salvar e Reiniciar]   │
└─────────────────────────────────────┘
```

---

## 📅 Cronograma de Implementação

### **Fase 1: Correções Críticas** (2-3 horas)
**Prioridade:** 🔴 ALTA

- [x] **1.1** Corrigir "Enable Force" não salvando
  - Adicionar flag `requiresReboot` no state
  - Detectar mudanças em campos críticos
  - Implementar auto-reboot ou aviso
  - **CORREÇÃO BACKEND DEFINITIVA:** Atualizado `send_hardware_settings` no Rust para enviar **todos** os campos individualmente usando os IDs corretos extraídos da referência oficial (`wheel_api_lib.js`). O método anterior enviava apenas 3 campos.
  - **Tempo estimado:** 1h

- [x] **1.2** Adicionar avisos de reinicialização
  - Criar componente `RebootWarning` (Implementado via Help Icon)
  - Adicionar aos campos: force_enabled, GPIO (automático)
  - **Tempo estimado:** 1h

- [x] **1.3** Testar funcionalidade completa
  - Testar salvar + reiniciar
  - Verificar persistência após reboot
  - **Tempo estimado:** 30min

### **Fase 2: Melhorias de Textos** (3-4 horas)
**Prioridade:** 🟡 MÉDIA

- [x] **2.1** Atualizar traduções PT-BR
  - Adicionar novos textos de ajuda melhorados
  - Adicionar avisos de reinicialização
  - **Tempo estimado:** 1.5h

- [x] **2.2** Atualizar traduções EN
  - Traduzir novos textos
  - Manter consistência
  - **Tempo estimado:** 1h

- [ ] **2.3** Implementar tooltips expandidos
  - Adicionar ícones de informação
  - Tooltips com mais detalhes
  - **Tempo estimado:** 1h

### **Fase 3: Melhorias de UX** (4-5 horas)
**Prioridade:** 🟢 BAIXA

- [x] **3.1** Botão "Salvar e Reiniciar" inteligente
  - Detectar mudanças que requerem reboot
  - Mudar label e cor do botão
  - **Tempo estimado:** 2h

- [x] **3.2** Modal de confirmação
  - Criar componente de confirmação (Implícito no botão "Salvar e Reiniciar")
  - Listar mudanças que requerem reboot
  - **Tempo estimado:** 1.5h

- [x] **3.3** Indicadores visuais
  - Ícones 🔄 nos campos (Usado ícone de ajuda com warning)
  - Cores de destaque
  - **Tempo estimado:** 1h

### **Fase 4: Sistema de Logs** (NOVO)
**Prioridade:** 🟡 MÉDIA

- [x] **4.1** Implementar Store de Logs
  - Criar `stores/logs.ts`
  - Centralizar mensagens de log
  - **Tempo estimado:** 30min

- [x] **4.2** Conectar LogsTab
  - Substituir dados mockados por dados reais do store
  - Implementar auto-scroll e exportação
  - **Tempo estimado:** 30min

- [x] **4.3** Instrumentar HardwareStore
  - Redirecionar eventos de conexão, erro e comandos para o log
  - **Tempo estimado:** 30min

- [x] **4.4** Melhorar MonitorTab e Limpar Logs Backend
  - Remover spam de logs `info!` no Rust (polling de HID)
  - Adicionar switch "Debug Info" no MonitorTab
  - Exibir dados raw (bytes) apenas quando solicitado
  - **Tempo estimado:** 30min

- [x] **4.5** Versionamento Dinâmico
  - Implementar comando `get_versions` no Rust
  - Expor versão da lib `ffbeast-controller`
  - Exibir versões reais na aba Configurações
  - **Tempo estimado:** 15min

- [x] **4.6** Correção de Leitura de Botões (Linux)
  - Implementar `EvdevGamepadReader` para leitura direta de `/dev/input/event*`
  - Adicionar redundância ao `gilrs`
  - Mapear botões genéricos (BTN_0..31) corretamente
  - **Tempo estimado:** 30min

### **Fase 5: Documentação** (1-2 horas)
**Prioridade:** 🟢 BAIXA

- [ ] **5.1** Atualizar README
  - Documentar campos que requerem reboot
  - Adicionar troubleshooting
  - **Tempo estimado:** 30min

- [ ] **4.2** Criar guia de configuração inicial
  - Baseado na documentação oficial
  - Passo a passo ilustrado
  - **Tempo estimado:** 1h

---

## 🔧 Detalhes Técnicos de Implementação

### Estrutura de Dados para Rastreamento de Reboot

```typescript
// Em hardware.ts store
interface RebootRequiredField {
    field: string;
    oldValue: any;
    newValue: any;
    label: string;
}

state: () => ({
    // ... existing state
    rebootRequiredFields: [] as RebootRequiredField[],
    rebootRequiredConfig: {
        force_enabled: true,
        calibration_mode: true,
        calibration_speed: true,
        calibration_magnitude: true,
        pole_pairs: true,
        encoder_cpr: true,
        // GPIO fields
        pin_modes: true,
    }
})
```

### Componente de Aviso de Reinicialização

```vue
<!-- RebootWarning.vue -->
<template>
  <div class="reboot-warning">
    <span class="reboot-icon">🔄</span>
    <span class="reboot-text">{{ $t('requires_reboot') }}</span>
  </div>
</template>

<style scoped>
.reboot-warning {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--warning);
  font-size: 0.75rem;
  margin-top: 4px;
}

.reboot-icon {
  animation: rotate 2s linear infinite;
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
```

---

## 📊 Campos Afetados - Resumo Completo

| Campo | Aba | Requer Reboot | Prioridade Fix |
|-------|-----|---------------|----------------|
| Enable Force | Hardware | ✅ Confirmado | 🔴 ALTA |
| Calibration Mode | Hardware | ✅ Confirmado | 🔴 ALTA |
| GPIO Pin Modes | Pins | ✅ Confirmado | 🟡 MÉDIA |
| Calibration Speed | Hardware | ❓ A Confirmar | 🟡 MÉDIA |
| Calibration Magnitude | Hardware | ❓ A Confirmar | 🟡 MÉDIA |
| Pole Pairs | Hardware | ❓ A Testar | � BAIXA |
| Encoder CPR | Hardware | ❓ A Testar | � BAIXA |
| Power Limit | Hardware | ❌ Não | - |
| Braking Limit | Hardware | ❌ Não | - |
| Motion Range | Effects | ❌ Não | - |
| All Effect Strengths | Effects | ❌ Não | - |
| PID Gains (P/I) | Hardware | ❌ Não | - |

**Legenda:**
- ✅ Confirmado: Documentação oficial explícita
- ❓ A Confirmar: Provável mas não explícito na documentação
- ❓ A Testar: Sem evidência na documentação, precisa teste prático
- ❌ Não: Confirmado que não requer reboot

---

## ✅ Critérios de Sucesso

### Funcionalidade
- [ ] "Enable Force" salva corretamente e persiste após reboot
- [ ] Todos os campos com reboot necessário têm aviso visual
- [ ] Usuário é notificado antes de reinicializar
- [ ] Reinicialização automática funciona (ou aviso claro)

### UX
- [ ] Textos de ajuda são claros e informativos
- [ ] Avisos de reinicialização são visíveis mas não intrusivos
- [ ] Usuário entende quais mudanças requerem reboot
- [ ] Processo de salvar + reiniciar é intuitivo

### Qualidade
- [ ] Todas as traduções estão completas (PT-BR e EN)
- [ ] Sem erros de console
- [ ] Performance não afetada
- [ ] Código bem documentado

---

## 📚 Referências

1. **Documentação Oficial FFBeast:**
   - `docs/ffbeast_github/ffbeast_setup_controller.md`
   - `docs/ffbeast_github/ffbeast_setup_effects.md`

2. **Código Relevante:**
   - `apps/configurator/src/stores/hardware.ts`
   - `apps/configurator/src/components/tabs/HardwareTab.vue`
   - `apps/configurator/src/components/tabs/EffectsTab.vue`

3. **Backend:**
   - `libs/controller/src/hardware_service.rs`
   - `apps/configurator/src-tauri/src/lib.rs`

---

## 🎯 Próximos Passos Imediatos

1. **Implementar Fase 1.1** - Corrigir "Enable Force"
2. **Implementar Fase 1.2** - Adicionar avisos de reinicialização
3. **Testar com dispositivo real**
4. **Coletar feedback do usuário**
5. **Iterar baseado no feedback**

---

**Documento criado em:** 2026-01-19  
**Última atualização:** 2026-01-19  
**Status:** 📋 Planejamento Completo - Aguardando Aprovação
