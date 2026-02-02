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
        :default-name="getDefaultName(index)"
        :mapped-key="axisMappings[index]?.key"
        :mapped-button="axisMappings[index]?.button"
        :min="store.adc?.raxis_min[index]"
        :max="store.adc?.raxis_max[index]"
        :invert="store.adc?.raxis_invert[index] === 1"
        :smoothing="store.adc?.raxis_smoothing[index]"
        :btn-low="store.adc?.raxis_to_button_low[index]"
        :btn-high="store.adc?.raxis_to_button_high[index]"
        @edit="startEditing(index)"
        @save-name="saveAxisName(index)"
        @update:min="(v: number) => updateMin(index, v)"
        @update:max="(v: number) => updateMax(index, v)"
        @update:invert="(v: boolean) => updateInvert(index, v)"
        @update:smoothing="(v: number) => updateSmoothing(index, v)"
        @update:btnLow="(v: number) => updateBtnLow(index, v)"
        @update:btnHigh="(v: number) => updateBtnHigh(index, v)"
      />
    </div>

    <!-- Mapping Modal -->
    <MappingEditModal
      v-if="editingIdx !== null"
      :show="showModal"
      :axis-index="editingIdx"
      :axis-name="axisNames[editingIdx] || getDefaultName(editingIdx)"
      :axis-value="getScaledAxisValue(editingIdx)"
      :initial-config="getCurrentMappingConfig(editingIdx)"
      @close="closeModal"
      @save="handleModalSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useHardwareStore } from '../../stores/hardware';
import { useHardwareStream } from '@shared/composables/useHardwareStream';
import AxisMappingRow from '@shared/components/organisms/AxisMappingRow.vue';
import MappingEditModal, { type MappingConfig } from '@shared/components/organisms/MappingEditModal.vue';
import type { KeyMapping } from '@shared/models/KeyMapping';
import type { KeyboardProfile, AxisMapping } from '@shared/models/KeyboardProfile';
import { HardwareService } from '../../services/hardware_service';
import { HardwareSettingId } from '../../models/HardwareSettingId';

const store = useHardwareStore();
const { t } = useI18n();
const { status: hardwareStatus } = useHardwareStream();

// State
const showModal = ref(false);
const editingIdx = ref<number | null>(null);
const activeProfile = ref<KeyboardProfile | null>(null);

// Mappings local state (reconstructed for UI)
const axisNames = ref<string[]>(Array(8).fill(''));
const axisMappings = ref<AxisMapping[]>([]);

// Computed
const activeIndices = computed(() => {
  return [0, 1, 2, 3, 4, 5].filter(i => {
    if (i < 3) return true;
    return store.gpio?.pin_mode[i] === 2; // Analog mode
  });
});

// Helper functions for labels and names
const getAxisLabel = (index: number) => {
  if (index < 3) return `R${['x', 'y', 'z'][index]}`;
  return `GPIO ${index}`;
};

const getDefaultName = (index: number) => {
  switch(index) {
    case 0: return t('axis.names.rotation_x');
    case 1: return t('axis.names.rotation_y');
    case 2: return t('axis.names.rotation_z');
    case 3: return t('axis.names.slider');
    case 4: return t('axis.names.dial');
    default: return `${t('axis.names.aux')} ${index - 4}`;
  }
};

// Implementation
const getAxisValue = (index: number) => {
  return hardwareStatus.value?.adc[index] ?? 0;
};

const getScaledAxisValue = (index: number) => {
  const raw = getAxisValue(index);
  // Scale 12-bit (0-4095) to 16-bit (0-65535)
  return Math.floor((raw * 65535) / 4095);
};


// Calibration Updates
const updateMin = (index: number, val: number) => {
  if (!store.adc) return;
  const mins = [...store.adc.raxis_min];
  mins[index] = val;
  store.updateADC({ raxis_min: mins });
  store.updateADCField(HardwareSettingId.AdcMin, index, val, true);
};

const updateMax = (index: number, val: number) => {
  if (!store.adc) return;
  const maxes = [...store.adc.raxis_max];
  maxes[index] = val;
  store.updateADC({ raxis_max: maxes });
  store.updateADCField(HardwareSettingId.AdcMax, index, val, true);
};

const updateInvert = (index: number, invert: boolean) => {
  if (!store.adc) return;
  const invs = [...store.adc.raxis_invert];
  invs[index] = invert ? 1 : 0;
  store.updateADC({ raxis_invert: invs });
  store.updateADCField(HardwareSettingId.AdcInvert, index, invert ? 1 : 0, false);
};

const updateSmoothing = (index: number, val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_smoothing];
  vals[index] = val;
  store.updateADC({ raxis_smoothing: vals });
  store.updateADCField(HardwareSettingId.AdcSmoothing, index, val, false);
};

const updateBtnLow = (index: number, val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_to_button_low];
  vals[index] = val;
  store.updateADC({ raxis_to_button_low: vals });
  store.updateADCField(HardwareSettingId.AdcButtonLow, index, val, false);
};

const updateBtnHigh = (index: number, val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_to_button_high];
  vals[index] = val;
  store.updateADC({ raxis_to_button_high: vals });
  store.updateADCField(HardwareSettingId.AdcButtonHigh, index, val, false);
};


const loadConfig = async () => {
  try {
    const config = await HardwareService.getKeyboardConfig();
    
    // Always initialize with 8 defaults
    const names = Array(8).fill('');
    const mappings = Array(8).fill(null).map(() => ({
      name: '',
      key: '',
      button: '',
      threshold_low: 4000,
      threshold_high: 60000
    }));

    const profile = config.active_profile_id 
      ? config.profiles.find(p => p.id === config.active_profile_id)
      : config.profiles[0];

    if (profile) {
      activeProfile.value = profile;
      
      // Populate names from axis_names map
      if (profile.axis_names) {
        Object.entries(profile.axis_names).forEach(([idx, name]) => {
          const i = parseInt(idx);
          if (i >= 0 && i < 8) {
            names[i] = name;
            mappings[i].name = name;
          }
        });
      }

      // Populate mappings values
      profile.axis_mappings.forEach((m: any, idx) => {
        if (idx < 8) {
          // Migration from old schema (key_low / key_high)
          const migratedMapping = {
            name: m.name || '',
            key: m.key || m.key_high || m.key_low || '',
            button: m.button || m.btn_high || m.btn_low || '',
            threshold_low: m.threshold_low ?? 4000,
            threshold_high: m.threshold_high ?? 60000
          };
          mappings[idx] = migratedMapping;
        }
      });
    }

    axisNames.value = names;
    axisMappings.value = mappings as AxisMapping[];
  } catch (err) {
    console.error('Failed to load keyboard config:', err);
  }
};

const saveConfig = async () => {
  try {
    const config = await HardwareService.getKeyboardConfig();
    let currentProfile = activeProfile.value;
    
    if (!currentProfile) {
      // Create a default profile if none exists
      if (config.profiles.length === 0) {
          const profile: KeyboardProfile = {
              id: Date.now().toString(),
              name: 'Default',
              key_mappings: [],
              axis_mappings: [],
              axis_names: {}
          };
          config.profiles.push(profile);
          config.active_profile_id = profile.id;
          currentProfile = profile;
          activeProfile.value = profile;
      } else {
          config.active_profile_id = config.profiles[0].id;
          currentProfile = config.profiles[0];
          activeProfile.value = currentProfile;
      }
    }

    let profileIdx = config.profiles.findIndex(p => p.id === config.active_profile_id);
    
    // Fallback to first profile if active_profile_id is missing or invalid
    if (profileIdx === -1 && config.profiles.length > 0) {
        profileIdx = 0;
        config.active_profile_id = config.profiles[0].id;
    }

    if (profileIdx !== -1) {
      config.profiles[profileIdx].axis_mappings = axisMappings.value;
      // Also update KeyMappings based on AxisMappings for the backend processing engine
      config.profiles[profileIdx].key_mappings = generateKeyMappings();
      
      // Update axis names map
      const nameMap: Record<number, string> = {};
      axisNames.value.forEach((name, i) => {
        if (name) nameMap[i] = name;
      });
      config.profiles[profileIdx].axis_names = nameMap;
      
      console.log('InputsTab: Saving keyboard config', config);
      await HardwareService.setKeyboardConfig(config);
      // Also update the live mappings in the engine if service is running
      console.log('InputsTab: Updating live engine mappings', config.profiles[profileIdx].key_mappings);
      await store.updateKeyboardMapping(config.profiles[profileIdx].key_mappings);
    }
  } catch (err) {
    console.error('Failed to save keyboard config:', err);
  }
};

const generateKeyMappings = (): KeyMapping[] => {
  const kbm: KeyMapping[] = [];
  activeIndices.value.forEach(idx => {
    const m = axisMappings.value[idx];
    if (!m) return;
    
    // Only create a mapping if a key or button is assigned
    if (m.key || m.button) {
      kbm.push({
        id: `axis.${idx + 3}`,
        source_type: 'axis',
        index: idx + 3,
        trigger: 'high',
        key: m.key || '',
        button: m.button || '',
        threshold_min: m.threshold_low,
        threshold_max: m.threshold_high
      });
    }
  });
  return kbm;
};

const getCurrentMappingConfig = (index: number): MappingConfig => {
  const m = axisMappings.value[index];
  return {
    key: m?.key || '',
    button: m?.button || '',
    thresholdLow: m?.threshold_low ?? 4000,
    thresholdHigh: m?.threshold_high ?? 60000
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

const saveAxisName = (index: number) => {
  if (axisMappings.value[index]) {
    axisMappings.value[index].name = axisNames.value[index];
  }
  saveConfig();
};

const handleModalSave = async (newName: string, cfg: MappingConfig) => {
  if (editingIdx.value === null) return;
  const idx = editingIdx.value;

  // Update local state
  axisNames.value[idx] = newName;
  axisMappings.value[idx] = {
    name: newName,
    key: cfg.key,
    button: cfg.button,
    threshold_low: cfg.thresholdLow,
    threshold_high: cfg.thresholdHigh
  };

  await saveConfig();
  closeModal();
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
