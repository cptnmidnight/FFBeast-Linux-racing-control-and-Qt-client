<template>
  <div class="inputs-tab">
    <div class="axes-grid">
      <AxisMappingRow
        v-for="index in activeIndices"
        :key="index"
        :index="index"
        :label="getAxisLabel(index)"
        v-model="axisNames[index]"
        :raw-value="getAxisValue(index)"
        :default-name="DEFAULT_NAMES[index]"
        @edit="startEditing(index)"
        @save-name="saveAxisName"
      />
    </div>

    <!-- Mapping Modal -->
    <MappingEditModal
      v-if="editingIdx !== null"
      :show="showModal"
      :axis-index="editingIdx"
      :axis-name="axisNames[editingIdx] || DEFAULT_NAMES[editingIdx]"
      :axis-value="getAxisValue(editingIdx)"
      :initial-config="getCurrentMappingConfig(editingIdx)"
      @close="closeModal"
      @save="handleModalSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import { useHardwareStream } from '../../composables/useHardwareStream';
import AxisMappingRow from '../widgets/AxisMappingRow.vue';
import MappingEditModal, { type MappingConfig } from '../widgets/MappingEditModal.vue';
import type { KeyMapping } from '../../models/KeyMapping';
import { useMappingPersistence, DEFAULT_NAMES } from '../../composables/useMappingPersistence';

const store = useHardwareStore();
const { status: hardwareStatus } = useHardwareStream();

// State
const showModal = ref(false);
const editingIdx = ref<number | null>(null);

const { axisNames, mappings, load: loadConfig, save: saveConfig } = useMappingPersistence();

// Computed
const activeIndices = computed(() => {
  return [0, 1, 2, 3, 4, 5, 6, 7].filter(i => {
    if (i < 3) return true;
    return store.gpio?.pin_mode[i] === 2; // Analog mode
  });
});

// Implementation
const getAxisLabel = (index: number) => {
  if (index < 3) return `AXIS ${['X', 'Y', 'Z'][index]}`;
  return `GPIO ${index}`;
};

const getAxisValue = (index: number) => {
  return hardwareStatus.value?.adc[index] ?? 0;
};

const getCurrentMappingConfig = (index: number): MappingConfig => {
  const m = mappings.value[index];
  // Ensure we return a valid config object, filling missing fields if necessary
  // Assuming 'm' has the compatible structure, otherwise defaults.
  return {
    keyLow: m?.keyLow || '',
    thresholdLow: m?.thresholdLow ?? 2000,
    keyHigh: m?.keyHigh || '',
    thresholdHigh: m?.thresholdHigh ?? 30000,
    btnLow: m?.btnLow ?? null,
    btnHigh: m?.btnHigh ?? null
  };
};

const startEditing = (index: number) => {
  editingIdx.value = index;
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
  editingIdx.value = null;
};

const saveAxisName = () => {
  saveConfig(axisNames.value);
};

const handleModalSave = async (newName: string, config: MappingConfig) => {
  if (editingIdx.value === null) return;
  const idx = editingIdx.value;

  // Update local state
  axisNames.value[idx] = newName;
  mappings.value[idx] = { ...mappings.value[idx], ...config };

  // Persist
  saveConfig(axisNames.value, mappings.value);

  // Generate KeyMappings for backend
  await pushKeyMappingsToBackend();
  
  closeModal();
};

const pushKeyMappingsToBackend = async () => {
  const keyMappings: KeyMapping[] = [];
  
  activeIndices.value.forEach(actualIndex => {
    const mapping = mappings.value[actualIndex];
    if (!mapping) return;
    
    // High threshold mapping
    if (mapping.keyHigh) {
      keyMappings.push({
        id: `axis.${actualIndex}_high_${mapping.keyHigh}`,
        source_type: 'axis',
        index: actualIndex,
        trigger: 'high',
        key: mapping.keyHigh,
        threshold: mapping.thresholdHigh ?? 30000
      });
    }
    
    // Low threshold mapping
    if (mapping.keyLow) {
      keyMappings.push({
        id: `axis.${actualIndex}_low_${mapping.keyLow}`,
        source_type: 'axis',
        index: actualIndex,
        trigger: 'low',
        key: mapping.keyLow,
        threshold: mapping.thresholdLow ?? 2000
      });
    }
  });

  if (keyMappings.length > 0) {
    try {
      await store.updateKeyboardMapping(keyMappings);
      store.log('info', `Configured ${keyMappings.length} axis keyboard mappings`);
    } catch (err) {
      store.log('error', `Failed to set keyboard mappings: ${err}`);
    }
  }
};

onMounted(() => {
  loadConfig();
});
</script>

<style scoped>
.inputs-tab {
  padding: var(--content-padding);
}

.axes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
}
</style>
