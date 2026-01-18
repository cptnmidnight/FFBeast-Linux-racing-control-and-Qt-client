# Bugs Conhecidos

Lista de bugs identificados para correção e rastreamento.

## FFBeast Controller (Backend/Firmware Interaction)

1. **Feedback dos eixos analógicos e botões não funciona**
   - **Descrição**: A interface de monitoramento não reflete as entradas físicas (butões pressionados, movimento de eixos) em tempo real.
   - **Status**: Crítico. Possível falha na leitura do HID Report ou na transmissão de eventos para o Frontend.

2. **Mapeamento de teclas não funciona**
   - **Descrição**: O mapeamento não funciona nem para botões HID nem para teclado. Suspeita de mal funcionamento do serviço de teclado em Rust.
   - **Status**: Crítico.

---
*Gerado automaticamente em 18/01/2026*
