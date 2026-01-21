# Plano de Implementação - Sistema de Configurações

## 🎯 Objetivo
Criar uma tela de configurações completa no launcher com:
1. Configurações padrão do volante (global)
2. Mapeamento de teclas do gamepad
3. Opções de customização da interface
4. Menu especial para controladora

---

## 📦 Estrutura de Componentes

### 1. **SettingsView.vue** (Página Principal)
- Grid layout similar ao configurator
- Abas/cards organizados por categoria

### 2. **Componentes de Configuração**

#### **WheelDefaultSettings.vue**
```
- Motion Range (Graus de Rotação)
- Total Force (Força Total)
- Dynamic Dampening
- Static Dampening
- Power Limit
- Braking Limit
```

#### **GamepadMappingSettings.vue**
```
- Serviço de mapeamento de teclas
- Configuração de eixos (X, Y, Z, RZ, etc)
- Mapeamento de botões
- Dead zones
- Inversão de eixos
```

#### **UICustomizationSettings.vue**
```
Sistema de Acesso | Configurações Disponíveis
-------------------|----------------------------
Cores & Tema      | • Cor principal (accent)
                  | • Modo escuro/claro (futuro)
                  | • Transparência de cards
Tipografia        | • Fonte principal
                  | • Tamanho da fonte
Notificações      | • Posição dos toasts
                  | • Duração
                  | • Margem
Avançado          | • Debug mode
                  | • Log level
                  | • Auto-start com sistema
```

---

## 🗄️ Armazenamento

### **LocalStorage** (UI Settings)
```typescript
{
  "launcher_accent_color": "#00d4ff",
  "launcher_font_family": "Outfit",
  "launcher_font_size": 16,
  "launcher_toast_position": "bottom-right"
}
```

### **SQLite** (Wheel & Gamepad Settings)
```sql
CREATE TABLE default_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Exemplos:
-- key: "wheel_motion_range", value: "900"
-- key: "wheel_total_force", value: "100"
-- key: "gamepad_axis_x_inverted", value: "false"
```

---

## 🔌 Backend (Rust)

### Novos Comandos Tauri
```rust
// Configurações padrão do volante
#[tauri::command]
fn get_default_wheel_settings() -> Result<WheelSettings, String>

#[tauri::command]
fn save_default_wheel_settings(settings: WheelSettings) -> Result<(), String>

// Mapeamento de gamepad
#[tauri::command]
fn get_gamepad_mapping() -> Result<GamepadMapping, String>

#[tauri::command]
fn save_gamepad_mapping(mapping: GamepadMapping) -> Result<(), String>

// Aplicar configurações ao hardware
#[tauri::command]
fn apply_wheel_settings_to_hardware(settings: WheelSettings) -> Result<(), String>
```

---

## 🎮 Integração com Jogos

### Fluxo ao Lançar Jogo
```
1. [✅] Detectar se é Steam ou não-Steam
2. [✅] Detectar se é .exe (Proton)
3. [🆕] ANTES de lançar:
   a. Verificar se jogo tem perfil customizado
   b. Se SIM: aplicar perfil customizado ao hardware
   c. Se NÃO: aplicar configurações padrão globais
4. [✅] Lançar jogo
5. [🆕] Monitorar conexão do volante durante jogo
```

---

## 🎨 UI/UX

### Menu de Navegação
```
📚 Biblioteca  (atual)
⚙️ Configurações (NOVO)
   ├─ 🎮 Controladora
   │   ├─ Perfil Padrão do Volante
   │   └─ Mapeamento de Teclas
   └─ 🎨 Interface
       ├─ Aparência
       └─ Notificações
```

### Layout Inspirado no Configurator
- Grid responsivo (auto-fit, minmax(350px, 1fr))
- BaseCard components
- BaseSlider para valores numéricos
- BaseSwitch para toggles
- BaseSelect para opções

---

## 📝 Checklist de Implementação

### Fase 1: Backend ✅
- [✅] Aplicar perfil antes de lançar (game_runner.rs)
- [ ] Comandos Tauri para settings
- [ ] Modelo de dados para WheelSettings
- [ ] Modelo de dados para GamepadMapping
- [ ] Integração com HardwareService

### Fase 2: Frontend - Estrutura
- [ ] Criar SettingsView.vue
- [ ] Componentes base (se não existirem)
- [ ] Routing para /settings

### Fase 3: Frontend - Controladora
- [ ] WheelDefaultSettings component
- [ ] GamepadMapping component
- [ ] Integração com backend

### Fase 4: Frontend - UI Customization
- [ ] UICustomization component
- [ ] Aplicação de temas em tempo real
- [ ] Persistência em localStorage

### Fase 5: Testes & Polish
- [ ] Testar aplicação de perfil
- [ ] Testar salvamento de configurações
- [ ] Validações de input
- [ ] Tradução PT-BR

---

## 🚀 Próximos Passos

**Aguardando aprovação para continuar com:**
1. Criar modelos de dados (WheelSettings, GamepadMapping)
2. Implementar comandos Tauri
3. Criar componentes Vue
4. Integrar tudo

**Estimativa**: ~2-3 horas de implementação completa

---

## ❓ Perguntas para o Usuário

1. **Gamepad Mapping**: Quer usar biblioteca existente (gilrs-rs) ou implementação custom?
2. **Auto-aplicar ao hardware**: Deve aplicar configurações toda vez que conectar o volante?
3. **Reset defaults**: Botão para restaurar configurações de fábrica?
4. **Prioridade**: Qual parte implementar primeiro? (Volante, Gamepad, ou UI?)
