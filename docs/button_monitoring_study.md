# Estudo de Caso: Monitoramento de Botões no Linux

**Data:** 2026-01-19  
**Autor:** Análise Técnica - FFBeast Controller  
**Objetivo:** Avaliar opções para monitoramento de botões do volante FFBeast no Linux e propor soluções

> ⚠️ **RESTRIÇÃO IMPORTANTE:** O firmware do FFBeast é **fechado e proprietário**. NÃO temos acesso ao código-fonte nem possibilidade de modificá-lo. Todas as soluções devem ser implementadas **exclusivamente no software**, sem depender de alterações no firmware.

---

## 1. Situação Atual

### 1.1 Como o Output do Volante é Capturado Atualmente

Analisando o código em `libs/controller/src/hardware_service.rs`, o sistema atual usa **duas fontes de dados**:

#### **Fonte Principal: HID Reports via HIDAPI**
```rust
// Linha 254-456 em hardware_service.rs
fn read_status(&self) -> Result<WheelStatus> {
    let mut buf = [0u8; 64];
    match dev.read_timeout(&mut buf, 10) {
        Ok(res) if res >= 8 => {
            // Lê do HID Report (Report ID 0xA3 ou 0x01)
            let pos = i16::from_le_bytes([buf[6], buf[7]]);      // Posição do volante
            let torque = i16::from_le_bytes([buf[8], buf[9]]);   // Torque
            let buttons = u32::from_le_bytes([buf[10..14]]);     // Botões (32 bits)
            let adc = [...];                                      // ADC (6 canais)
        }
    }
}
```

**Características:**
- Usa `hidapi` crate para comunicação USB HID
- Lê reports periódicos do dispositivo via `/dev/hidraw*` (Linux)
- Report ID: `0xA3` (REPORT_GENERIC_INPUT_OUTPUT)
- Estrutura do pacote:
  - Bytes 0: Report ID
  - Bytes 1-4: Firmware version
  - Byte 5: Is registered
  - Bytes 6-7: Position (i16)
  - Bytes 8-9: Torque (i16)
  - **Bytes 10-13: Buttons (u32 - 32 botões)**
  - **Bytes 14+: ADC values (6x u16)**

#### **Fonte Secundária: Gamepad via Gilrs**
```rust
// Linha 236-248 em hardware_service.rs
match create_gamepad_reader() {
    Ok(reader) => {
        // Inicializa leitor de gamepad cross-platform
        // Usa biblioteca 'gilrs' que lê de /dev/input/event* no Linux
    }
}

// Linha 347-377: Merge de dados
if buttons == 0 && gamepad_state.buttons != 0 {
    buttons = gamepad_state.buttons;  // Fallback para gamepad
}
```

**Características:**
- Usa `gilrs` crate (Game Input Library for Rust)
- No Linux, lê de `/dev/input/event*` via `evdev`
- Serve como **fallback** quando HID Report não traz dados de botões
- Também captura eixos analógicos (axes)

---

## 2. Problema Identificado

### 2.1 Sintomas
- Botões não estão sendo detectados consistentemente no Linux
- O HID Report às vezes retorna `buttons = 0` mesmo quando botões estão pressionados
- O fallback do gamepad também não está funcionando adequadamente

### 2.2 Possíveis Causas

1. **HID Report não inclui dados de botões no Linux**
   - O firmware pode estar enviando reports diferentes no Linux vs Windows
   - O driver `hidraw` pode estar filtrando ou interpretando dados diferentemente

2. **Gamepad não está sendo reconhecido corretamente**
   - O FFBeast pode não estar se registrando como gamepad no Linux
   - O `gilrs` pode não estar mapeando os botões corretamente

3. **Conflito entre interfaces HID**
   - O dispositivo pode ter múltiplas interfaces HID
   - Uma interface para force feedback, outra para input
   - Estamos lendo da interface errada

---

## 3. Opções de Solução

### 3.1 Opção 1: Melhorar Leitura via HID Report (Atual)

**Descrição:** Corrigir a leitura atual do HID Report para garantir que os botões sejam capturados.

**Vantagens:**
- ✅ Usa a mesma interface que já captura posição e torque
- ✅ Dados sincronizados (botões + posição no mesmo pacote)
- ✅ Baixa latência
- ✅ Não depende de drivers adicionais

**Desvantagens:**
- ❌ **CRÍTICO:** Pode exigir mudanças no firmware se o report não incluir botões - **IMPOSSÍVEL pois o firmware é fechado**
- ❌ Pode ser específico para cada plataforma (Windows vs Linux)
- ❌ Dependemos completamente do que o firmware decidir enviar

**Implementação:**
```rust
// Adicionar logging detalhado para debug
tracing::info!("HID Report completo: {:02X?}", &buf[..res]);

// Verificar se o report realmente contém dados de botões
if res >= 14 {
    let buttons = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]);
    tracing::info!("Botões lidos: 0x{:08X} (binário: {:032b})", buttons, buttons);
}

// Testar leitura de diferentes Report IDs
// Pode haver um report específico para input (0x01) vs output (0xA3)
```

**Próximos Passos:**
1. Adicionar logging detalhado dos HID Reports recebidos
2. Testar pressionar botões e verificar se algum byte muda
3. Comparar reports entre Windows e Linux
4. ~~Se necessário, solicitar ao fabricante documentação do HID Report Descriptor~~ **NÃO APLICÁVEL - firmware fechado**

**⚠️ LIMITAÇÃO:** Como o firmware é fechado, se os botões não estiverem nos HID Reports, **não há como corrigi-los nesta camada**. Esta opção só funciona se o firmware JÁ enviar os dados de botões.

---

### 3.2 Opção 2: Usar Evdev Diretamente (Recomendado)

**Descrição:** Ler botões diretamente de `/dev/input/event*` usando `evdev`, separado da leitura de posição/torque via HID.

**Vantagens:**
- ✅ Interface padrão do Linux para input devices
- ✅ Funciona com qualquer dispositivo que se registre como gamepad/joystick
- ✅ Suporte nativo a hot-plugging
- ✅ Não depende do firmware enviar botões via HID Report
- ✅ Pode capturar eventos de múltiplos dispositivos simultaneamente

**Desvantagens:**
- ❌ Dados vêm de duas fontes diferentes (HID para posição, evdev para botões)
- ❌ Possível dessincronização temporal (pequena)
- ❌ Requer permissões de leitura em `/dev/input/event*`

**Implementação:**

#### Opção 2A: Usar `evdev` crate diretamente
```rust
use evdev::{Device, InputEventKind};

pub struct EvdevButtonReader {
    device: Device,
    button_state: u32,
}

impl EvdevButtonReader {
    pub fn new() -> Result<Self> {
        // Encontrar dispositivo FFBeast
        let mut devices = evdev::enumerate();
        let device = devices
            .find(|(_, dev)| {
                dev.name().map_or(false, |n| 
                    n.contains("FFBeast") || 
                    n.contains("vjoy") ||
                    n.contains("Wheel")
                )
            })
            .map(|(_, dev)| dev)
            .ok_or_else(|| anyhow!("FFBeast device not found"))?;
        
        device.grab()?; // Opcional: captura exclusiva
        
        Ok(Self {
            device,
            button_state: 0,
        })
    }
    
    pub fn read_buttons(&mut self) -> Result<u32> {
        // Processar eventos pendentes
        for event in self.device.fetch_events()? {
            match event.kind() {
                InputEventKind::Key(key) => {
                    let button_num = key.code(); // BTN_0, BTN_1, etc.
                    let pressed = event.value() != 0;
                    
                    if pressed {
                        self.button_state |= 1 << button_num;
                    } else {
                        self.button_state &= !(1 << button_num);
                    }
                }
                _ => {}
            }
        }
        
        Ok(self.button_state)
    }
}
```

#### Opção 2B: Melhorar o `gilrs` atual
```rust
// O código atual já usa gilrs, mas pode não estar configurado corretamente
// Verificar se o dispositivo está sendo detectado:

impl GilrsGamepadReader {
    pub fn new() -> Result<Self> {
        let gilrs = gilrs::Gilrs::new()?;
        
        // Listar TODOS os dispositivos detectados
        for (id, gamepad) in gilrs.gamepads() {
            tracing::info!(
                "Gamepad detectado: ID={:?}, Nome='{}', UUID={:?}, Mapeado={}",
                id,
                gamepad.name(),
                gamepad.uuid(),
                gamepad.is_ff_supported()
            );
        }
        
        // Procurar especificamente por FFBeast
        let gamepad_id = gilrs.gamepads()
            .find(|(_, gp)| {
                let name = gp.name().to_lowercase();
                name.contains("ffbeast") || 
                name.contains("wheel") ||
                name.contains("vjoy")
            })
            .map(|(id, _)| id);
        
        // ...
    }
}
```

**Próximos Passos:**
1. Verificar se o FFBeast aparece em `/dev/input/` como `event*` ou `js*`
2. Usar `evtest` ou `jstest` para confirmar que botões são detectados
3. Implementar leitor evdev dedicado
4. Integrar com o `read_status()` existente

---

### 3.3 Opção 3: Usar Joystick API Legada

**Descrição:** Usar a antiga Joystick API (`/dev/input/js*`) para ler botões.

**Vantagens:**
- ✅ API simples e bem documentada
- ✅ Funciona em sistemas Linux antigos

**Desvantagens:**
- ❌ API legada, não recomendada para novos projetos
- ❌ Menos informações que evdev (sem timestamps precisos)
- ❌ Limitado a 32 botões e 8 eixos

**Implementação:**
```rust
use std::fs::File;
use std::io::Read;

#[repr(C)]
struct JsEvent {
    time: u32,      // timestamp em ms
    value: i16,     // valor do evento
    type_: u8,      // JS_EVENT_BUTTON ou JS_EVENT_AXIS
    number: u8,     // número do botão/eixo
}

pub fn read_joystick_buttons(device_path: &str) -> Result<u32> {
    let mut file = File::open(device_path)?;
    let mut event = JsEvent { time: 0, value: 0, type_: 0, number: 0 };
    let mut buttons = 0u32;
    
    loop {
        let bytes_read = file.read(std::slice::from_raw_parts_mut(
            &mut event as *mut _ as *mut u8,
            std::mem::size_of::<JsEvent>()
        ))?;
        
        if bytes_read == 0 { break; }
        
        if event.type_ & 0x01 != 0 { // JS_EVENT_BUTTON
            if event.value != 0 {
                buttons |= 1 << event.number;
            } else {
                buttons &= !(1 << event.number);
            }
        }
    }
    
    Ok(buttons)
}
```

**Não Recomendado:** Esta opção é inferior ao evdev moderno.

---

### 3.4 Opção 4: Híbrido - HID + Evdev

**Descrição:** Usar HID para posição/torque (dados críticos de baixa latência) e evdev para botões (menos críticos).

**Vantagens:**
- ✅ Melhor de ambos os mundos
- ✅ Posição/torque sincronizados via HID
- ✅ Botões garantidos via evdev
- ✅ Redundância: se HID tiver botões, usa; senão, usa evdev

**Desvantagens:**
- ❌ Mais complexo de implementar
- ❌ Duas fontes de dados para gerenciar

**Implementação:**
```rust
pub struct HybridWheelReader {
    hid_device: HidDevice,           // Para posição/torque
    evdev_reader: EvdevButtonReader, // Para botões
}

impl HybridWheelReader {
    fn read_status(&self) -> Result<WheelStatus> {
        // 1. Ler HID Report (posição, torque, talvez botões)
        let hid_data = self.read_hid_report()?;
        
        // 2. Ler botões via evdev
        let evdev_buttons = self.evdev_reader.read_buttons()?;
        
        // 3. Merge: priorizar HID se tiver botões, senão usar evdev
        let final_buttons = if hid_data.buttons != 0 {
            hid_data.buttons
        } else {
            evdev_buttons
        };
        
        Ok(WheelStatus {
            position: hid_data.position,
            torque: hid_data.torque,
            buttons: final_buttons,
            // ...
        })
    }
}
```

---

## 4. Comparação das Opções

| Critério | Opção 1: HID | Opção 2: Evdev | Opção 3: Joystick | Opção 4: Híbrido |
|----------|--------------|----------------|-------------------|------------------|
| **Complexidade** | Baixa | Média | Baixa | Alta |
| **Confiabilidade** | ⚠️ Depende do firmware | ✅ Alta | ✅ Alta | ✅ Muito Alta |
| **Latência** | ✅ Muito Baixa | ✅ Baixa | ✅ Baixa | ✅ Muito Baixa |
| **Sincronização** | ✅ Perfeita | ⚠️ Boa | ⚠️ Boa | ✅ Perfeita |
| **Portabilidade** | ✅ Cross-platform | ❌ Linux only | ❌ Linux only | ⚠️ Requer código específico |
| **Manutenção** | ✅ Fácil | ✅ Fácil | ✅ Fácil | ⚠️ Moderada |

---

## 5. Recomendação

### **Recomendação Primária: Opção 2 (Evdev Direto)**

**Justificativa:**
1. O código atual já usa `gilrs` (que usa evdev internamente), mas pode não estar configurado corretamente
2. Evdev é a interface padrão e recomendada para input devices no Linux
3. **CRÍTICO:** Não depende do firmware enviar botões via HID Report - **essencial pois o firmware é fechado e não pode ser modificado**
4. Permite debug fácil com ferramentas como `evtest`
5. Solução 100% em software, sem necessidade de alterações no hardware/firmware

**Plano de Implementação:**

#### Fase 1: Diagnóstico (1-2 horas)
```bash
# 1. Verificar se FFBeast aparece como input device
ls -la /dev/input/by-id/

# 2. Testar com evtest
sudo evtest

# 3. Verificar eventos de botões
sudo evtest /dev/input/eventX  # Substituir X pelo número correto

# 4. Verificar com jstest (se disponível)
jstest /dev/input/js0
```

#### Fase 2: Correção do Código (2-4 horas)
1. Melhorar logging do `GilrsGamepadReader` para ver se dispositivo está sendo detectado
2. Se gilrs não detectar, implementar `EvdevButtonReader` direto
3. Adicionar fallback: HID → Gilrs → Evdev direto
4. Testar em diferentes distribuições Linux

#### Fase 3: Otimização (1-2 horas)
1. Ajustar mapeamento de botões se necessário
2. Adicionar configuração para usuário escolher fonte de botões
3. Documentar requisitos de permissões (`udev` rules se necessário)

---

### **Recomendação Secundária: Opção 4 (Híbrido)**

Se a Opção 2 não funcionar ou se quisermos máxima confiabilidade:
- Manter HID para dados críticos (posição/torque)
- Adicionar evdev dedicado para botões
- Implementar merge inteligente com priorização

---

## 6. Código de Exemplo para Teste Rápido

### Teste 1: Verificar se evdev detecta botões
```rust
// Adicionar em libs/controller/src/gamepad_reader.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Rodar manualmente com: cargo test -- --ignored
    fn test_evdev_button_detection() {
        use evdev::Device;
        
        println!("Dispositivos de input disponíveis:");
        for (path, device) in evdev::enumerate() {
            println!("  {:?}: {}", path, device.name().unwrap_or("Unknown"));
            
            if let Some(name) = device.name() {
                if name.to_lowercase().contains("ffbeast") {
                    println!("    ✅ FFBeast encontrado!");
                    println!("    Botões suportados: {:?}", device.supported_keys());
                }
            }
        }
    }
}
```

### Teste 2: Monitorar botões em tempo real
```rust
// Adicionar comando CLI temporário para debug

fn main() {
    // ... código existente ...
    
    #[cfg(debug_assertions)]
    if std::env::args().any(|a| a == "--test-buttons") {
        test_button_monitoring();
        return;
    }
}

#[cfg(debug_assertions)]
fn test_button_monitoring() {
    use evdev::{Device, InputEventKind};
    
    let mut device = evdev::enumerate()
        .find(|(_, d)| d.name().map_or(false, |n| n.contains("FFBeast")))
        .map(|(_, d)| d)
        .expect("FFBeast não encontrado");
    
    println!("Monitorando botões do FFBeast. Pressione Ctrl+C para sair.");
    
    loop {
        for event in device.fetch_events().unwrap() {
            if let InputEventKind::Key(key) = event.kind() {
                println!(
                    "Botão {}: {}",
                    key.code(),
                    if event.value() != 0 { "PRESSIONADO" } else { "SOLTO" }
                );
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
```

---

## 7. Dependências Necessárias

### Para Opção 2 (Evdev):
```toml
# Adicionar em libs/controller/Cargo.toml
[dependencies]
evdev = "0.12"  # Para acesso direto ao evdev
```

### Permissões no Linux:
```bash
# Criar regra udev para permitir acesso sem sudo
sudo nano /etc/udev/rules.d/99-ffbeast.rules

# Adicionar:
SUBSYSTEM=="input", ATTRS{idVendor}=="045b", ATTRS{idProduct}=="59d7", MODE="0666"
# Substituir idVendor e idProduct pelos valores corretos do FFBeast

# Recarregar regras
sudo udevadm control --reload-rules
sudo udevadm trigger
```

---

## 8. Conclusão

O problema de monitoramento de botões no Linux pode ser resolvido de forma robusta usando a interface `evdev`, que é o padrão do kernel Linux para dispositivos de input. A solução recomendada é:

1. **Curto prazo:** Melhorar o código `gilrs` existente com melhor logging e detecção
2. **Médio prazo:** Implementar leitor `evdev` direto como fallback
3. **Longo prazo:** Considerar solução híbrida para máxima confiabilidade

A mesma interface (`evdev`) que captura eixos analógicos pode e deve ser usada para botões, garantindo consistência e confiabilidade no Linux.

### Restrições do Projeto

**⚠️ IMPORTANTE:** Como o firmware do FFBeast é **fechado e proprietário**, todas as soluções propostas são **exclusivamente em software**. Isso significa:

- ✅ Podemos ler dados de qualquer interface que o dispositivo exponha (HID, evdev, joystick)
- ✅ Podemos implementar lógica de fallback e merge de dados
- ✅ Podemos melhorar detecção e mapeamento de botões
- ❌ **NÃO** podemos modificar o que o firmware envia via HID Reports
- ❌ **NÃO** podemos adicionar novos reports ou mudar a estrutura de dados
- ❌ **NÃO** podemos corrigir bugs no firmware

Portanto, a **Opção 2 (Evdev Direto)** é a mais adequada, pois não depende do comportamento do firmware e utiliza a interface padrão do Linux que o kernel já gerencia automaticamente.
