# Plano de Refatoração - Tela de Configurações (Detalhado)

## 🎯 Objetivo
Migrar sistema de configurações do Configurator para o Launcher com todas as features avançadas.

---

## 📦 FASE 1: Componentes Base (Pré-requisito)
**Tempo estimado: 30min**

### 1.1 Criar BaseSlider.vue com Help Text
```vue
Props:
- modelValue: number
- label: string
- help?: string (tooltip/descrição)
- min?: number (default: 0)
- max?: number (default: 100)
- step?: number (default: 1)
- suffix?: string (ex: "°", "%")

Features:
- Tooltip com help text
- Valor numérico editável
- Visual igual ao configurator
```

### 1.2 Criar BaseSwitch.vue
```vue
Props:
- modelValue: boolean
- label: string
- help?: string

Features:
- Toggle moderno
- Help tooltip
```

### 1.3 Criar BaseCard.vue
```vue
Props:
- title: string

Features:
- Card com glassmorphism
- Footer slot opcional
```

**Arquivos a criar**:
- `/components/common/BaseSlider.vue`
- `/components/common/BaseSwitch.vue`
- `/components/common/BaseCard.vue`

---

## 📦 FASE 2: Wheel Settings Melhorado
**Tempo estimado: 45min**

### 2.1 Atualizar DefaultWheelSettings Model
```rust
// Adicionar novos campos ao model
pub struct DefaultWheelSettings {
    pub motion_range: u16,              // 180-1440
    pub total_force: u8,                // 0-100%
    pub integrated_spring_strength: u8, // 0-100%
    pub static_dampening_strength: u16, // 0-1000%
    pub dynamic_dampening_strength: u16,// 0-1000%
    pub soft_stop_strength: u8,         // 0-100%
    pub soft_stop_range: u8,            // 0-255°
    pub soft_stop_dampening: u16,       // 0-1000%
    pub direct_x_constant: u8,          // 0-100%
    pub direct_x_periodic: u8,          // 0-100%
    pub direct_x_spring: u8,            // 0-100%
    pub invert_game_force: bool,        // toggle
}

impl Default {
    motion_range: 900,  // ✅ FIX: 900 graus padrão
    // ... outros defaults
}
```

### 2.2 Refatorar Settings.vue - Seção Wheel
Usar BaseCard e BaseSlider igual ao EffectsTab.vue:

```vue
<BaseCard title="Configurações Gerais">
  <BaseSlider 
    v-model="settings.motion_range"
    label="Graus de Rotação"
    help="Define o ângulo máximo de rotação do volante"
    :min="180"
    :max="1440"
    :step="10"
    suffix="°"
  />
  <BaseSlider 
    v-model="settings.total_force"
    label="Força Total"
    help="Intensidade geral do Force Feedback"
    suffix="%"
  />
</BaseCard>

<BaseCard title="Soft Stop">
  <!-- 3 sliders conforme EffectsTab -->
</BaseCard>

<BaseCard title="Amortecimento">
  <!-- 3 sliders conforme EffectsTab -->
</BaseCard>

<BaseCard title="DirectX (Efeitos do Jogo)">
  <!-- 3 sliders + 1 switch -->
</BaseCard>
```

**Arquivos a modificar**:
- `/models/settings.rs` - Expandir struct
- `/views/Settings.vue` - Usar BaseSlider
- `/types/settings.ts` - Atualizar interface

---

## 📦 FASE 3: Sistema de Mapeamento de Eixos
**Tempo estimado**: 1h 30min

### 3.1 Backend - KeyboardService Integration

**Criar**: `/services/keyboard_service.rs` (copiar do configurator)
```rust
pub struct KeyboardService {
    active: Mutex<bool>,
    mappings: Mutex<Vec<KeyMapping>>,
    active_keys: Mutex<HashSet<String>>,
}

impl KeyboardService {
    pub fn set_mappings(&self, mappings: Vec<KeyMapping>)
    pub fn set_active(&self, active: bool)
    pub fn is_active(&self) -> bool
    pub fn process(&self, status: &WheelStatus)
}
```

**Novos comandos Tauri**:
```rust
#[tauri::command]
fn keyboard_service_start() -> Result<(), String>

#[tauri::command]
fn keyboard_service_stop() -> Result<(), String>

#[tauri::command]
fn keyboard_service_restart() -> Result<(), String>

#[tauri::command]
fn keyboard_service_is_active() -> Result<bool, String>

#[tauri::command]
fn set_keyboard_mapping(mappings: Vec<KeyMapping>) -> Result<(), String>

#[tauri::command]
fn get_keyboard_mapping() -> Result<Vec<KeyMapping>, String>
```

### 3.2 Frontend - InputsTab Component

**Criar**: `/components/AxisMapping.vue` (baseado em InputsTab.vue)

```vue
<template>
  <div class="axis-mapping">
    <!-- Grid de eixos -->
    <div class="axes-grid">
      <div v-for="axis in axes" class="axis-card">
        <div class="axis-header">
          <span>{{ axis.label }}</span>
          <input 
            v-model="axisNames[axis.index]"
            class="axis-name-input"
            placeholder="Nome customizado"
          />
          <button @click="editMapping(axis.index)">
            ⚙️ Editar
          </button>
        </div>

        <!-- Hardware calibration -->
        <BaseSlider 
          v-model="axis.min"
          label="Mínimo"
          :max="32767"
        />
        <BaseSlider 
          v-model="axis.max"
          label="Máximo"
          :max="32767"
        />
        <BaseSlider 
          v-model="axis.smoothing"
          label="Suavização"
          suffix="%"
        />
        <BaseSwitch 
          v-model="axis.inverted"
          label="Inverter Eixo"
        />
      </div>
    </div>

    <!-- Modal de mapeamento -->
    <BaseModal v-if="showModal">
      <!-- Joystick buttons -->
      <BaseSelect 
        v-model="mapping.btnLow"
        label="Botão quando Baixo"
        :options="buttonOptions"
      />
      <BaseSelect 
        v-model="mapping.btnHigh"
        label="Botão quando Alto"
        :options="buttonOptions"
      />

      <!-- Keyboard keys -->
      <BaseSelect 
        v-model="mapping.keyLow"
        label="Tecla quando Baixo"
        :options="keyOptions"
      />
      <BaseSelect 
        v-model="mapping.keyHigh"
        label="Tecla quando Alto"
        :options="keyOptions"
      />
    </BaseModal>
  </div>
</template>
```

**Arquivos a criar**:
- `/services/keyboard_service.rs` (backend)
- `/components/AxisMapping.vue` (frontend)
- `/types/keyboard.ts` (tipos)

### 3.3 KeyboardService Controls

**Adicionar à Settings.vue**:
```vue
<BaseCard title="Serviço de Mapeamento de Teclas">
  <div class="service-status">
    <span>Status: {{ serviceActive ? '🟢 Ativo' : '🔴 Inativo' }}</span>
  </div>

  <div class="service-controls">
    <button @click="startService">▶️ Iniciar</button>
    <button @click="stopService">⏹️ Parar</button>
    <button @click="restartService">🔄 Reiniciar</button>
  </div>

  <p class="help-text">
    O serviço de mapeamento converte eixos e botões do volante 
    em teclas do teclado para jogos que não suportam FFB nativamente.
  </p>
</BaseCard>
```

---

## 📦 FASE 4: Traduções PT-BR
**Tempo estimado: 20min**

### 4.1 Adicionar keys ao pt.json
```json
{
  "settings": {
    "title": "Configurações",
    "wheel": {
      "title": "Perfil Padrão do Volante",
      "motion_range": "Graus de Rotação",
      "total_force": "Força Total",
      "help_motion_range": "Define o ângulo máximo de rotação (180° a 1440°)",
      "help_total_force": "Intensidade geral do Force Feedback (0-100%)"
    },
    "keyboard": {
      "title": "Serviço de Mapeamento",
      "status": "Status",
      "active": "Ativo",
      "inactive": "Inativo",
      "start": "Iniciar",
      "stop": "Parar",
      "restart": "Reiniciar"
    },
    "axis": {
      "title": "Mapeamento de Eixos",
      "name": "Nome do Eixo",
      "min": "Valor Mínimo",
      "max": "Valor Máximo",
      "smoothing": "Suavização",
      "invert": "Inverter",
      "btn_low": "Botão quando Baixo",
      "btn_high": "Botão quando Alto",
      "key_low": "Tecla quando Baixo",
      "key_high": "Tecla quando Alto"
    }
  }
}
```

---

## 📦 FASE 5: Persistência (Storage)
**Tempo estimado: 30min**

### 5.1 Implementar Storage Real

**Opção 1: SQLite** (Recomendado)
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Exemplos:
INSERT INTO settings VALUES ('wheel_motion_range', '900');
INSERT INTO settings VALUES ('wheel_total_force', '100');
```

**Opção 2: TOML File**
```toml
[wheel]
motion_range = 900
total_force = 100

[keyboard_service]
active = false

[[axis_mappings]]
index = 0
name = "Throttle"
key_low = "W"
key_high = "S"
```

### 5.2 Implementar comandos de storage
```rust
fn save_setting(key: String, value: String) -> Result<()>
fn get_setting(key: String) -> Result<Option<String>>
fn get_all_settings() -> Result<HashMap<String, String>>
```

---

## 📝 CHECKLIST DE IMPLEMENTAÇÃO

### ✅ Completado (Já feito)
- [x] Estrutura básica de Settings
- [x] Navegação entre Library/Settings
- [x] Comandos Tauri básicos
- [x] Tipos TypeScript
- [x] Modal de settings

### 🔄 Em Progresso (Próximos Passos)
- [ ] **FASE 1**: Componentes base (BaseSlider, BaseCard, BaseSwitch)
- [ ] **FASE 2**: Wheel settings melhorado (+ campos)
- [ ] **FASE 3**: Sistema de mapeamento (eixos + keyboard service)
- [ ] **FASE 4**: Traduções completas
- [ ] **FASE 5**: Persistência real

---

## 🚀 ORDEM DE IMPLEMENTAÇÃO SUGERIDA

### Sprint 1 (1-2h)
1. Criar BaseSlider com help
2. Criar BaseCard  
3. Criar BaseSwitch
4. Atualizar model de WheelSettings
5. Fix motion_range default para 900

### Sprint 2 (1-2h)
6. Refatorar Settings.vue usando BaseComponents
7. Adicionar todos os sliders do EffectsTab
8. Traduções PT-BR básicas

### Sprint 3 (2-3h)
9. Backend: Copiar keyboard_service.rs
10. Backend: Criar comandos Tauri
11. Frontend: Criar AxisMapping component
12. Frontend: Integrar com Settings

### Sprint 4 (1h)
13. Implementar persistência (SQLite ou TOML)
14. Controles do keyboard service (start/stop/restart)
15. Testes finais

---

## 📌 PRIORIDADE IMEDIATA (Quick Wins)

Se quiser resultados rápidos, comece por:

1. **Fix motion_range = 900** (2 min)
2. **Criar BaseSlider** (30 min)
3. **Refatorar seção de Wheel com helpers** (30 min)

Isso já melhora muito a UX sem precisar fazer tudo de uma vez.

---

**Próximo passo**: Quer que eu comece pelo Sprint 1?
