# Arquitetura e Roadmap do SODevs Launcher

## 1. Refatoração e Arquitetura 🏗️
O projeto será reestruturado para um **Cargo Workspace** visando modularidade estrita:

### Estrutura de Pastas
- `launcher/`: **Apenas** código do Launcher (Gerenciamento de jogos, UI Principal, Runner).
- `libs/controller/`: Biblioteca Rust contendo **toda** a lógica de hardware (HID, Protocolo FFBeast). Deve ser agnóstica de UI.
- `apps/configurator/` (ou módulo isolado): Código específico para a Tela de Configuração da Controladora.

### Objetivos de Modularização
- [ ] Criar Cargo Workspace na raiz.
- [ ] Extrair `HardwareService` para crate `libs/controller`.
- [ ] Desacoplar tipos do Tauri da biblioteca de hardware.

## 2. Universalidade e Funcionalidades do Launcher 🎮
O Launcher deve gerenciar **qualquer jogo**, com ou sem volante.

### Gerenciamento de Jogos
- [ ] **Suporte Universal**: Jogos sem perfil de volante devem rodar normalmente (o launcher apenas executa).
- [ ] **Edição Avançada de Jogos**:
    - [ ] **Ícones**: Integração com API (ex: SteamGridDB) para buscar/selecionar ícones.
    - [ ] **Variáveis de Ambiente**: Tabela de chave/valor editável per-game.
    - [ ] **Argumentos**: Campo de texto para argumentos de linha de comando.
    - [ ] **Runner Personalizado**: Escolher executável (Wine, Proton, Nativo).

## 3. Tela de Configuração da Controladora ⚙️
Ambiente dedicado para ajuste fino do hardware.
- [ ] Interface isolada para calibração e setup.
- [ ] Mapeamento de pinos e diagnóstico.
- [ ] Monitoramento visual dos eixos em tempo real.

## 4. Cronograma Atualizado V2
1. **Refactor**: Criar Workspace e mover `libs/controller`.
2. **Backend**: Implementar edição de EnvVars e Args.
3. **Frontend**: Criar modal de edição avançada e busca de ícones.
4. **Configurator**: Migrar a tela `advanced_config` para a nova estrutura.
