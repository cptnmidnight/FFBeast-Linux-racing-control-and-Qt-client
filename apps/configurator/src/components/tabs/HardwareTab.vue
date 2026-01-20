<template>
  <div class="hardware-tab">
    <div class="grid-layout">
      <!-- Motor Limits -->
      <BaseCard :title="$t('group_motor')">
        <BaseSwitch 
          v-model="ffbEnabled" 
          :label="$t('ffb_active_label')" 
          :help="$t('help_force_enabled')"
          @update:model-value="toggleFFB"
        />
        <BaseSlider 
          v-model="hardware.power_limit" 
          :label="$t('setting_power_limit')" 
          :help="$t('help_power_limit')"
          suffix="%"
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.braking_limit" 
          :label="$t('setting_braking_limit')" 
          :help="$t('help_braking_limit')"
          suffix="%"
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.amplifier_gain" 
          :label="$t('setting_amplifier_gain')" 
          :help="$t('help_amplifier_gain')"
          suffix="%"
          :max="500" 
          @update:model-value="saveHardware"
        />
      </BaseCard>

      <!-- Advanced Motor Config -->
      <BaseCard :title="$t('settings_advanced')">
        <BaseSlider 
          v-model="hardware.encoder_cpr" 
          :label="$t('setting_encoder_cpr')" 
          :min="1" 
          :max="65535" 
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.speed_buffer_size" 
          :label="$t('setting_speed_buffer')" 
          :min="1" 
          :max="255" 
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.position_smoothing" 
          :label="$t('setting_pos_smoothing')" 
          suffix="%"
          :max="255"
          @update:model-value="saveHardware"
        />
      </BaseCard>

      <!-- Mechanical Config -->
      <BaseCard :title="$t('setting_mech_config')">
        <BaseSlider 
          v-model="hardware.pole_pairs" 
          :label="$t('setting_pole_pairs')" 
          :help="$t('help_pole_pairs')"
          :min="1" 
          :max="50" 
          @update:model-value="saveHardware"
        />
        <div class="switch-group">
          <BaseSwitch 
            v-model="encoderDir" 
            :label="$t('setting_encoder_dir')" 
            :help="$t('help_encoder_direction')"
            @update:model-value="handleSwitches"
          />
          <BaseSwitch 
            v-model="forceDir" 
            :label="$t('setting_force_dir')" 
            :help="$t('help_force_direction')"
            @update:model-value="handleSwitches"
          />
          <BaseSwitch 
            v-model="debugTorque" 
            :label="$t('setting_debug_torque')" 
            :help="$t('help_debug_torque')"
            @update:model-value="handleSwitches"
          />
        </div>
      </BaseCard>

      <!-- Calibration -->
      <BaseCard :title="$t('setting_calibration')">
        <BaseSlider 
          v-model="hardware.calibration_speed" 
          :label="$t('setting_calibration_speed')" 
          :help="$t('help_calibration_speed')"
          suffix="%"
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.calibration_magnitude" 
          :label="$t('setting_calibration_magnitude')" 
          :help="$t('help_calibration_magnitude')"
          suffix="%"
          @update:model-value="saveHardware"
        />
      </BaseCard>

      <!-- PID Controller -->
      <BaseCard :title="$t('group_pid')">
        <p class="description">{{ $t('group_pid_desc') }}</p>
        <BaseSlider 
          v-model="hardware.proportional_gain" 
          :label="$t('setting_p_gain')" 
          :help="$t('help_proportional_gain')"
          :max="2000" 
          @update:model-value="saveHardware"
        />
        <BaseSlider 
          v-model="hardware.integral_gain" 
          :label="$t('setting_i_gain')" 
          :help="$t('help_integral_gain')"
          :max="1000" 
          @update:model-value="saveHardware"
        />
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';
import BaseSwitch from '../common/BaseSwitch.vue';

const store = useHardwareStore();

const hardware = reactive({
  encoder_cpr: store.hardware?.encoder_cpr ?? 600,
  power_limit: store.hardware?.power_limit ?? 100,
  braking_limit: store.hardware?.braking_limit ?? 100,
  amplifier_gain: store.hardware?.amplifier_gain ?? 100,
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

const saveHardware = () => {
  store.updateHW({
    ...hardware,
    force_enabled: ffbEnabled.value ? 1 : 0,
    encoder_direction: encoderDir.value ? 1 : 0,
    force_direction: forceDir.value ? 1 : 0,
    debug_torque: debugTorque.value ? 1 : 0,
  });
};

const handleSwitches = () => {
  saveHardware();
};

const toggleFFB = async (val: boolean) => {
  ffbEnabled.value = val;
  saveHardware();
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
