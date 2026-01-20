# Roadmap: Key & Axis Mapping Service

Este documento detalha o plano de implementação para o serviço de mapeamento de entradas do controlador FFBeast para Teclado e Gamepad virtual.

## Objetivos
1. **Mapeamento de Botões**: Converter botões físicos em teclas do teclado.
2. **Mapeamento de Analógicos**: Converter eixos (Pedais/Volante) em teclas ou comandos.
3. **Multi-plataforma**: Suporte nativo para Windows e Linux.
4. **Baixa Dependência**: Utilizar chamadas de sistema nativas quando possível.

---

## Status Atual (2026-01-20)

### ✅ Implementado
- [x] Controles de serviço na aba **Ferramentas** (Start/Stop)
- [x] Persistência das configurações de mapeamento no LocalStorage
- [x] Interface de comunicação entre Store e `KeyboardService` em Rust
- [x] Arquitetura modular seguindo project guidelines:
  - `virtual_key.rs`: Enum de teclas cross-platform
  - `native_keyboard.rs`: Simulação nativa (Windows SendInput)
  - `keyboard_service.rs`: Serviço principal de mapeamento
- [x] UI para configuração de mapeamentos na aba Inputs
- [x] Logs detalhados para debugging
- [x] Internacionalização (i18n) de todas as mensagens

### ⚠️ Problemas Conhecidos
- **Mapeamento de Eixos para Teclado**: Não funciona - teclas não são pressionadas quando eixos RX, RY, RZ mudam
- **Detecção de Botões Físicos**: Funciona corretamente - botões são detectados via bitmask `status.buttons`
- **Simulação de Teclado**: Não funciona - `SendInput` não está gerando teclas no Windows
- **Possível causa principal**: 
  - Problema com a API `SendInput` do Windows (pode requerer privilégios elevados)
  - Discrepância entre índices de eixos da UI e índices reais do hardware ADC
- **Log Buffer**: Removido devido a deadlock - logs iniciais não aparecem no frontend

### 🔧 Próximos Passos
1. **Debug de Índices**: Verificar mapeamento correto entre UI e hardware
2. **Teste de Botões**: Confirmar leitura de `status.buttons` bitmask
3. **Validação Windows API**: Confirmar que `SendInput` está funcionando (pode requerer privilégios elevados)
4. **Implementação Linux**: Adicionar suporte a `/dev/uinput`

---

## Fase 1: Controle do Serviço e Infraestrutura ✅
- [x] Adicionar controles de serviço na aba **Ferramentas**.
- [x] Implementar persistência das configurações de mapeamento no LocalStorage.
- [x] Criar interface de comunicação entre Store e o `KeyboardService` em Rust.

## Fase 2: Simulação de Teclado (Windows/Linux) 🚧
### Windows (`SendInput`) - Implementado mas não funcional
- [x] Utilizar a API Win32 `SendInput` para simular pressionamento de teclas
- [x] Implementação nativa sem dependências externas (removido `enigo`)
- [ ] **Pendente**: Validar funcionamento (possível problema de permissões)
- **Vantagem**: Não requer drivers externos.

### Linux (`uinput`) - Não implementado
- [ ] Utilizar `/dev/uinput` para criar um dispositivo de entrada virtual a nível de kernel.
- **Vantagem**: Funciona em todos os ambientes de desktop (X11 e Wayland).

## Fase 3: Mapeamento Avançado de Eixos 🚧
- [x] Implementar lógica de limites (thresholds) para analógicos (high: 3500, low: 500)
- [ ] Criar modos de "Zona Morta" e "Curva de Resposta" para o mapeamento.
- [ ] **Desafio**: Mapear analógico para analógico virtual (requer ViGEmBus no Windows ou uinput Gamepad no Linux).

## Fase 4: Interface de Configuração (UI) 🚧
- [x] Criar modal dedicado para associar entradas → saídas (aba Inputs)
- [x] Seleção de teclas via dropdown
- [ ] Implementar modo de "Captura Inteligente" (pressione o botão para mapear).
- [ ] Indicador visual de eixo ativo em tempo real

---

## Estudo de Possibilidades Sem Dependências Externas

### No Windows ✅
Para simular teclado/mouse sem a biblioteca `enigo` ou similares:
- ✅ Implementado usando `windows-sys` para chamar `SendInput` diretamente.
- Para simular um **Gamepad**, a única forma nativa é via `Hid-compliant game controller`, mas é extremamente complexo implementar sem o driver `ViGEmBus`.

### No Linux ⏳
Para simular sem dependências:
- Abrir e escrever diretamente no device `/dev/uinput`. É a forma mais direta e "limpa" no Linux, permitindo criar teclados, mouses e até gamepads virtuais sem bibliotecas externas complexas.
- **Status**: Estrutura preparada, implementação pendente.

---

## Notas Técnicas

### Arquitetura Atual
```
keyboard_service.rs (Serviço principal)
├── virtual_key.rs (Enum VirtualKey + parse_key)
├── native_keyboard.rs (NativeKeyboard::send_key)
│   ├── Windows: SendInput API
│   └── Linux: /dev/uinput (TODO)
└── KeyMapping (struct de configuração)
```

### Mapeamento de Eixos
- **UI**: Mostra eixos ativos (X, Y, Z + GPIO 3-5 se configurados como analógicos)
- **Hardware**: ADC array [0-5] onde:
  - 0-2: X, Y, Z (primários)
  - 3-5: RX, RY, RZ (pedais/auxiliares)
- **Problema**: Conversão entre índice de array da UI e índice real do hardware

### Logs de Debug
- `KeyboardService: Processing` - Mostra estado dos eixos a cada segundo
- `KeyboardService: Triggering Key Press` - Quando tecla deveria ser pressionada
- `Key press successful/FAILED` - Resultado da chamada SendInput
