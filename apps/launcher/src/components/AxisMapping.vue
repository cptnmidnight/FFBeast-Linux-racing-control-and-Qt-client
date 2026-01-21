<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from './common/BaseCard.vue';
import BaseSlider from './common/BaseSlider.vue';
import BaseSwitch from './common/BaseSwitch.vue';
import BaseModal from './common/BaseModal.vue';
import BaseSelect from './common/BaseSelect.vue';
import type { KeyMapping } from '../types/keyboard';

export default defineComponent({
  name: 'AxisMapping',
  components: {
    BaseCard,
    BaseSlider,
    BaseSwitch,
    BaseModal,
    BaseSelect
  },
  setup() {
    const mappings = ref<KeyMapping[]>([]);
    const editingIndex = ref<number | null>(null);
    const showModal = ref(false);
    
    // Temp mapping for editing
    const currentMapping = ref<KeyMapping>({
      index: 0,
      name: '',
      key_low: '',
      key_high: '',
      btn_low: '',
      btn_high: ''
    });

    const axisLabels = ['Eixo 1 (Direção)', 'Eixo 2 (Acelerador)', 'Eixo 3 (Freio)', 'Eixo 4 (Embreagem)', 'Eixo 5', 'Eixo 6'];

    // Options for selects
    const keyOptions = [
      { label: 'Nenhum', value: '' },
      ...Array.from({ length: 26 }, (_, i) => ({
        label: String.fromCharCode(65 + i),
        value: String.fromCharCode(65 + i)
      })),
      { label: 'Seta Cima', value: 'Up' },
      { label: 'Seta Baixo', value: 'Down' },
      { label: 'Seta Esquerda', value: 'Left' },
      { label: 'Seta Direita', value: 'Right' },
      { label: 'Space', value: 'Space' },
      { label: 'Enter', value: 'Return' },
      { label: 'Shift', value: 'Shift' },
      { label: 'Ctrl', value: 'Control' },
      { label: 'Alt', value: 'Alt' }
    ];

    const btnOptions = [
      { label: 'Nenhum', value: '' },
      ...Array.from({ length: 32 }, (_, i) => ({
        label: `Botão ${i}`,
        value: `Button${i}`
      }))
    ];

    const loadMappings = async () => {
      try {
        const result = await invoke<KeyMapping[]>('get_keyboard_mapping');
        if (result && result.length > 0) {
            mappings.value = result;
        } else {
            // Initialize defaults if empty
            mappings.value = axisLabels.map((label, idx) => ({
                index: idx,
                name: label,
                key_low: '',
                key_high: '',
                btn_low: '',
                btn_high: ''
            }));
        }
      } catch (e) {
        console.error("Failed to load mappings", e);
      }
    };

    const editMapping = (index: number) => {
      editingIndex.value = index;
      // ensure we have default object structure
      const existing = mappings.value.find(m => m.index === index);
      if (existing) {
          currentMapping.value = { ...existing };
      } else {
          currentMapping.value = {
              index: index,
              name: axisLabels[index] || `Axis ${index}`,
              key_low: '',
              key_high: '',
              btn_low: '',
              btn_high: ''
          };
      }
      showModal.value = true;
    };

    const saveCurrentMapping = async () => {
      if (editingIndex.value !== null) {
          const idx = mappings.value.findIndex(m => m.index === editingIndex.value);
          if (idx >= 0) {
              mappings.value[idx] = { ...currentMapping.value };
          } else {
              mappings.value.push({ ...currentMapping.value });
          }
          await saveAll();
          showModal.value = false;
      }
    };

    const saveAll = async () => {
        try {
            await invoke('set_keyboard_mapping', { mappings: mappings.value });
        } catch(e) {
            console.error("Failed to save", e);
        }
    };

    onMounted(loadMappings);

    return {
      mappings,
      axisLabels,
      showModal,
      currentMapping,
      editMapping,
      saveCurrentMapping,
      keyOptions,
      btnOptions
    };
  }
});
</script>

<template>
  <div class="axis-mapping">
    <BaseCard title="Mapeamento de Eixos para Teclado/Botões">
        <p class="desc">
            Configure quais teclas ou botões serão acionados quando você mover os eixos do volante.
            Útil para jogos que não suportam volante nativamente.
        </p>

        <div class="mappings-list">
            <div v-for="(label, index) in axisLabels" :key="index" class="mapping-row">
                <div class="mapping-info">
                    <span class="axis-name">{{ mappings.find(m => m.index === index)?.name || label }}</span>
                    <div class="mapping-tags">
                        <span v-if="mappings.find(m => m.index === index)?.key_low" class="tag">Low: {{ mappings.find(m => m.index === index)?.key_low }}</span>
                        <span v-if="mappings.find(m => m.index === index)?.key_high" class="tag">High: {{ mappings.find(m => m.index === index)?.key_high }}</span>
                    </div>
                </div>
                <button class="btn-edit" @click="editMapping(index)">
                    ⚙️ Editar
                </button>
            </div>
        </div>
    </BaseCard>

    <BaseModal 
        v-if="showModal" 
        :title="`Editando: ${currentMapping.name}`"
        @close="showModal = false"
    >
        <div class="modal-form">
            <div class="form-group">
                <label>Nome do Eixo</label>
                <input v-model="currentMapping.name" type="text" class="text-input" />
            </div>

            <div class="split-col">
                <div class="col">
                    <h4>Quando eixo está Baixo (0%)</h4>
                    <BaseSelect 
                        v-model="currentMapping.key_low"
                        label="Emular Tecla"
                        :options="keyOptions"
                    />
                    <BaseSelect 
                        v-model="currentMapping.btn_low"
                        label="Emular Botão"
                        :options="btnOptions"
                    />
                </div>
                <div class="col">
                    <h4>Quando eixo está Alto (100%)</h4>
                    <BaseSelect 
                        v-model="currentMapping.key_high"
                        label="Emular Tecla"
                        :options="keyOptions"
                    />
                    <BaseSelect 
                        v-model="currentMapping.btn_high"
                        label="Emular Botão"
                        :options="btnOptions"
                    />
                </div>
            </div>
        </div>

        <template #footer>
            <button class="btn-secondary" @click="showModal = false">Cancelar</button>
            <button class="btn-primary" @click="saveCurrentMapping">Salvar</button>
        </template>
    </BaseModal>
  </div>
</template>

<style scoped>
.desc {
    opacity: 0.7;
    margin-bottom: 2rem;
    font-size: 0.9rem;
}

.mappings-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.mapping-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: background 0.2s;
}

.mapping-row:hover {
    background: rgba(255, 255, 255, 0.05);
}

.axis-name {
    font-weight: 600;
    display: block;
    margin-bottom: 0.25rem;
}

.mapping-tags {
    display: flex;
    gap: 0.5rem;
    font-size: 0.8rem;
}

.tag {
    background: rgba(124, 58, 237, 0.2);
    color: var(--primary-color);
    padding: 2px 6px;
    border-radius: 4px;
}

.btn-edit {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-color);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}

.btn-edit:hover {
    background: var(--primary-color);
    border-color: var(--primary-color);
    color: white;
}

.modal-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.text-input {
  width: 100%;
  padding: 0.75rem;
  background: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-color);
  font-size: 1rem;
}

.split-col {
    display: flex;
    gap: 1.5rem;
}

.col {
    flex: 1;
}

.col h4 {
    font-size: 0.9rem;
    margin-bottom: 1rem;
    opacity: 0.8;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 0.5rem;
}

.btn-primary, .btn-secondary {
  padding: 0.8rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-secondary {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
}
</style>
