<template>
  <div v-if="show" class="mapping-modal-overlay" @click.self="close">
    <div class="mapping-modal">
      <div class="modal-header">
        <h3 class="modal-title">
          {{ $t('axis.edit_title') }} <span class="highlight">{{ axisName }}</span>
        </h3>
        <button class="close-btn" @click="close">×</button>
      </div>

      <div class="modal-content">
        <!-- Axis Name -->
        <div class="form-group">
          <ThemedInput
            v-model="localName"
            :label="$t('axis.custom_name')"
          />
        </div>
        
        <div class="mapping-section">
          <div class="section-divider">{{ $t('axis.joystick_mapping') }}</div>
          <div class="form-row">
            <ThemedSelect
              v-model="btnLowProxy"
              :options="buttonOptions"
              :label="$t('axis.button_low')"
              :placeholder="$t('options.none')"
            />
            <ThemedSelect
              v-model="btnHighProxy"
              :options="buttonOptions"
              :label="$t('axis.button_high')"
              :placeholder="$t('options.none')"
            />
          </div>
        </div>

        <div class="mapping-section">
          <div class="section-divider">{{ $t('axis.keyboard_mapping') }}</div>
          
              <!-- Deadzone/Threshold Sliders -->
          <div class="deadzone-config">
            <DualThresholdSlider
              v-model:lowValue="config.thresholdLow"
              v-model:highValue="config.thresholdHigh"
              :min="0"
              :max="32767"
              :max-low="13106"
              :min-high="19660"
              :label="$t('settings.deadzone') || 'Thresholds'"
              :raw-value="liveValue"
            />
          </div>

          <div class="form-row">
            <ThemedSelect
              v-model="config.keyLow"
              :options="keyOptions"
              :label="$t('axis.key_low')"
              :placeholder="$t('options.none')"
            />
            <ThemedSelect
              v-model="config.keyHigh"
              :options="keyOptions"
              :label="$t('axis.key_high')"
              :placeholder="$t('options.none')"
            />
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-outline" @click="close">{{ $t('modals.cancel') }}</button>
        <button class="btn-primary" @click="save">{{ $t('modals.save') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import ThemedInput from '../shared/atoms/ThemedInput.vue';
import ThemedSelect, { type SelectOption } from '../shared/atoms/ThemedSelect.vue';
import DualThresholdSlider from '../shared/molecules/DualThresholdSlider.vue';
import { useHardwareStream } from '../../composables/useHardwareStream';

export interface MappingConfig {
  keyLow: string;
  thresholdLow: number;
  keyHigh: string;
  thresholdHigh: number;
  btnLow: number | null;
  btnHigh: number | null;
}

interface Props {
  show: boolean;
  axisIndex: number | null;
  axisName: string;
  axisValue: number; // Initial value
  initialConfig: MappingConfig;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'close': [];
  'save': [name: string, config: MappingConfig];
}>();

const { t } = useI18n();
const { status: hardwareStream } = useHardwareStream();

// Real-time value from stream, fallback to prop
const liveValue = computed(() => {
  if (props.axisIndex !== null && hardwareStream.value?.adc) {
    // Map logical axis (0, 1, 2) to hardware analogs (3, 4, 5)
    // Hardware often sends 12-bit (0-4095). We need to scale this to 16-bit (0-32767)
    // 4095 * 8 ~= 32760
    const raw = hardwareStream.value.adc[props.axisIndex + 3] ?? 0;
    return Math.min(32767, raw * 8);
  }
  return props.axisValue;
});

// Local state
const localName = ref('');
const config = ref<MappingConfig>({
  keyLow: '',
  thresholdLow: 2000,
  keyHigh: '',
  thresholdHigh: 30000,
  btnLow: null,
  btnHigh: null,
});

// Proxies for ThemedSelect (which doesn't accept null)
const btnLowProxy = computed({
  get: () => config.value.btnLow ?? -1,
  set: (val: string | number) => {
    const num = Number(val);
    config.value.btnLow = num < 0 ? null : num;
  }
});

const btnHighProxy = computed({
  get: () => config.value.btnHigh ?? -1,
  set: (val: string | number) => {
    const num = Number(val);
    config.value.btnHigh = num < 0 ? null : num;
  }
});

// Watch triggers to sync props to local state
watch(() => props.show, (newVal) => {
  if (newVal) {
    localName.value = props.axisName;
    config.value = { ...props.initialConfig };
  }
}, { immediate: true });


// Options generation
const buttonOptions = computed(() => {
  const opts: SelectOption[] = [{ label: t('options.none'), value: -1 }]; // Using -1 or null properly
  // Note: ThemedSelect handles value matching.
  // Ideally use null for 'none', but Select value prop might default to string.
  
  for (let i = 0; i < 32; i++) {
    opts.push({ label: `${t('buttons.label')} ${i + 1}`, value: i });
  }
  return opts;
});

const KEY_LIST = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "Up", "Down", "Left", "Right", "Space", "Enter", "Tab", "Shift", "Ctrl", "Alt", "Esc"];

const keyOptions = computed(() => {
  const opts: SelectOption[] = [{ label: t('options.none'), value: '' }];
  KEY_LIST.forEach(k => {
    opts.push({ label: k, value: k });
  });
  return opts;
});

function close() {
  emit('close');
}

function save() {
  // Normalize button values (convert -1 back to null if needed, though simpler to keep checks consistent)
  const finalConfig = { ...config.value };
  if (Number(finalConfig.btnLow) < 0) finalConfig.btnLow = null;
  if (Number(finalConfig.btnHigh) < 0) finalConfig.btnHigh = null;
  
  emit('save', localName.value, finalConfig);
}
</script>

<style scoped>
.mapping-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  padding: 20px;
}

.mapping-modal {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 12px;
  width: 100%;
  max-width: 800px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-main);
}

.highlight {
  color: var(--accent);
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-dim);
  font-size: 24px;
  cursor: pointer;
  line-height: 1;
  padding: 0;
  transition: color 0.2s;
}

.close-btn:hover {
  color: var(--text-main);
}

.modal-content {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.preview-monitor {
  padding: 12px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
}

.section-divider {
  font-size: 12px;
  font-weight: 800;
  text-transform: uppercase;
  color: var(--accent);
  border-bottom: 1px solid var(--accent-muted);
  padding-bottom: 4px;
  margin-bottom: 12px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.deadzone-config {
  margin-bottom: 16px;
}

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  background: rgba(0, 0, 0, 0.1);
}

.btn-primary, .btn-outline {
  padding: 8px 16px;
  border-radius: var(--radius-sm);
  font-family: 'Outfit', sans-serif;
  font-weight: 600;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent);
  color: #000;
  border: none;
}

.btn-primary:hover {
  background: var(--accent-muted);
  transform: translateY(-1px);
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--border);
  color: var(--text-dim);
}

.btn-outline:hover {
  border-color: var(--text-main);
  color: var(--text-main);
}
</style>
