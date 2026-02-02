<template>
  <div class="hardware-tab">
    <div class="grid-layout">
      <!-- Motor & Power Limits -->
      <BaseCard :title="$t('groups.motor')">
        <ThemedSwitch 
          v-model="ffbEnabled" 
          :label="$t('labels.ffb_active')" 
          @change="toggleFFB"
        />
        <ThemedSlider 
          v-model="hardware.power_limit" 
          :label="$t('settings.power_limit')" 
          value-suffix="%"
          help-key="help.power_limit"
          @change="saveHardware(HardwareSettingId.PowerLimit)"
        />
        <ThemedSlider 
          v-model="hardware.braking_limit" 
          :label="$t('settings.braking_limit')" 
          value-suffix="%"
          help-key="help.braking_limit"
          @change="saveHardware(HardwareSettingId.BrakingLimit)"
        />
        <ThemedSelect 
          :model-value="hardware.amplifier_gain" 
          :label="$t('settings.amplifier_gain')" 
          :options="amplifierGainOptions"
          help-key="help.amplifier_gain"
          @change="(v) => updateAmplifierGain(v)"
        />
      </BaseCard>

      <!-- Precision & Filtering -->
      <BaseCard :title="$t('settings.advanced')">
        <ThemedSlider 
          v-model="hardware.encoder_cpr" 
          :label="$t('settings.encoder_cpr')" 
          :min="1" 
          :max="65535" 
          help-key="help.encoder_cpr"
          @change="saveHardware(HardwareSettingId.EncoderCPR, true)"
        />
        <ThemedSlider 
          v-model="hardware.speed_buffer_size" 
          :label="$t('settings.speed_buffer')" 
          :min="1" 
          :max="255" 
          help-key="help.speed_buffer"
          @change="saveHardware(HardwareSettingId.SpeedBufferSize)"
        />
        <ThemedSlider 
          v-model="hardware.position_smoothing" 
          :label="$t('settings.pos_smoothing')" 
          value-suffix="%"
          :max="255"
          help-key="help.position_smoothing"
          @change="saveHardware(HardwareSettingId.PositionSmoothing)"
        />
      </BaseCard>

      <!-- Mechanical Parameters -->
      <BaseCard :title="$t('settings.mech_config')">
        <ThemedSlider 
          v-model="hardware.pole_pairs" 
          :label="$t('settings.pole_pairs')" 
          :min="1" 
          :max="50" 
          help-key="help.pole_pairs"
          @change="saveHardware(HardwareSettingId.PolePairs)"
        />
        <div class="switch-group">
          <ThemedSwitch 
            v-model="encoderDir" 
            :label="$t('settings.encoder_dir')" 
            help-key="help.encoder_direction"
            @change="saveHardware(HardwareSettingId.EncoderDirection)"
          />
          <ThemedSwitch 
            v-model="forceDir" 
            :label="$t('settings.force_dir')" 
            help-key="help.force_direction"
            @change="saveHardware(HardwareSettingId.ForceInvert)"
          />
          <ThemedSwitch 
            v-model="debugTorque" 
            :label="$t('settings.debug_torque')" 
            help-key="help.debug_torque"
            @change="saveHardware(HardwareSettingId.DebugTorque)"
          />
        </div>
      </BaseCard>

      <!-- Calibration Parameters -->
      <BaseCard :title="$t('settings.calibration')">
        <ThemedSlider 
          v-model="hardware.calibration_speed" 
          :label="$t('settings.calibration_speed')" 
          value-suffix="%"
          help-key="help.calibration_speed"
          @change="saveHardware(HardwareSettingId.CalibrationSpeed)"
        />
        <ThemedSlider 
          v-model="hardware.calibration_magnitude" 
          :label="$t('settings.calibration_magnitude')" 
          value-suffix="%"
          help-key="help.calibration_magnitude"
          @change="saveHardware(HardwareSettingId.CalibrationMagnitude)"
        />
      </BaseCard>

      <!-- PID Controller Theory -->
      <BaseCard :title="$t('groups.pid')">
        <p class="description">{{ $t('groups.pid_desc') }}</p>
        <ThemedSlider 
          v-model="hardware.proportional_gain" 
          :label="$t('settings.p_gain')" 
          value-suffix="%"
          :max="100" 
          help-key="help.proportional_gain"
          @change="saveHardware(HardwareSettingId.PGain)"
        />
        <ThemedSlider 
          v-model="hardware.integral_gain" 
          :label="$t('settings.i_gain')" 
          value-suffix="%"
          :max="500" 
          help-key="help.integral_gain"
          @change="saveHardware(HardwareSettingId.IGain, true)"
        />
      </BaseCard>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, reactive, watch, onMounted } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import ThemedSlider from '@shared/components/atoms/ThemedSlider.vue';
import ThemedSwitch from '@shared/components/atoms/ThemedSwitch.vue';
import ThemedSelect from '@shared/components/atoms/ThemedSelect.vue';
import { HardwareSettingId } from '../../models/HardwareSettingId';

const store = useHardwareStore();

// Amplifier Gain enum options (from hardware API)
const amplifierGainOptions = [
  { value: 0, label: '80 V/V' },
  { value: 1, label: '40 V/V' },
  { value: 2, label: '20 V/V' },
  { value: 3, label: '10 V/V' },
];

const hardware = reactive({
  encoder_cpr: store.hardware?.encoder_cpr ?? 600,
  power_limit: store.hardware?.power_limit ?? 100,
  braking_limit: store.hardware?.braking_limit ?? 100,
  amplifier_gain: store.hardware?.amplifier_gain ?? 0, // Default: 80 V/V
  pole_pairs: store.hardware?.pole_pairs ?? 7,
  calibration_speed: store.hardware?.calibration_speed ?? 10,
  calibration_magnitude: store.hardware?.calibration_magnitude ?? 50,
  proportional_gain: store.hardware?.proportional_gain ?? 100,
  integral_gain: store.hardware?.integral_gain ?? 0,
  force_enabled: store.hardware?.force_enabled ?? 1,
  speed_buffer_size: store.hardware?.speed_buffer_size ?? 10,
  position_smoothing: store.hardware?.position_smoothing ?? 0,
});

const encoderDir = ref(false);
const forceDir = ref(false);
const debugTorque = ref(false);
const ffbEnabled = ref(true);

const syncFromStore = () => {
  if (store.hardware) {
    Object.assign(hardware, store.hardware);
    encoderDir.value = store.hardware.encoder_direction === 1;
    forceDir.value = store.hardware.force_direction === 1;
    debugTorque.value = store.hardware.debug_torque === 1;
    ffbEnabled.value = store.hardware.force_enabled === 1;
  }
};

onMounted(syncFromStore);
watch(() => store.hardware, syncFromStore, { deep: true });

const saveHardware = (fieldId?: number, isU16: boolean = false) => {
  const currentHW = {
    ...hardware,
    force_enabled: ffbEnabled.value ? 1 : 0,
    encoder_direction: encoderDir.value ? 1 : -1,
    force_direction: forceDir.value ? 1 : -1,
    debug_torque: debugTorque.value ? 1 : 0,
  };

  if (fieldId !== undefined) {
    let value = 0;
    // Map fieldId to value
    switch (fieldId) {
      case HardwareSettingId.EncoderCPR: value = currentHW.encoder_cpr; break;
      case HardwareSettingId.IGain: value = currentHW.integral_gain; break;
      case HardwareSettingId.PGain: value = currentHW.proportional_gain; break;
      case HardwareSettingId.ForceEnabled: value = currentHW.force_enabled; break;
      case HardwareSettingId.DebugTorque: value = currentHW.debug_torque; break;
      case HardwareSettingId.AmplifierGain: value = currentHW.amplifier_gain; break;
      case HardwareSettingId.CalibrationMagnitude: value = currentHW.calibration_magnitude; break;
      case HardwareSettingId.CalibrationSpeed: value = currentHW.calibration_speed; break;
      case HardwareSettingId.PowerLimit: value = currentHW.power_limit; break;
      case HardwareSettingId.BrakingLimit: value = currentHW.braking_limit; break;
      case HardwareSettingId.PositionSmoothing: value = currentHW.position_smoothing; break;
      case HardwareSettingId.SpeedBufferSize: value = currentHW.speed_buffer_size; break;
      case HardwareSettingId.EncoderDirection: value = currentHW.encoder_direction; break;
      case HardwareSettingId.ForceInvert: value = currentHW.force_direction; break;
      case HardwareSettingId.PolePairs: value = currentHW.pole_pairs; break;
    }
    store.updateHWField(fieldId, 0, value, isU16);
  }

  // Update store state without triggering a full hardware send
  store.updateHW(currentHW);
};

const updateAmplifierGain = (value: string | number) => {
  hardware.amplifier_gain = typeof value === 'string' ? parseInt(value) : value;
  saveHardware(HardwareSettingId.AmplifierGain);
};

const toggleFFB = async (val: boolean) => {
  ffbEnabled.value = val;
  saveHardware(HardwareSettingId.ForceEnabled);
};
</script>

<style scoped>
.hardware-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}

.switch-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-top: 1rem;
}
</style>
