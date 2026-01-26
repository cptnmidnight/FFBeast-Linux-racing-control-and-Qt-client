<template>
  <div class="effects-tab">
    <div class="grid-layout">
      <!-- General Effects -->
      <BaseCard :title="$t('groups.general')">
        <ThemedSlider 
          v-model="effects.motion_range" 
          :label="$t('settings.motion_range')" 
          :min="180" 
          :max="1440" 
          :step="10" 
          value-suffix="°" 
          help-key="help.motion_range"
          @change="saveEffects(HardwareSettingId.MotionRange, true)"
        />
        <ThemedSlider 
          v-model="effects.total_effect_strength" 
          :label="$t('settings.total_strength')" 
          value-suffix="%"
          help-key="help.total_effect_strength"
          @change="saveEffects(HardwareSettingId.TotalEffectStrength)"
        />
      </BaseCard>

      <!-- Soft Stop Configuration -->
      <BaseCard :title="$t('groups.soft_stop')">
        <ThemedSlider 
          v-model="effects.soft_stop_strength" 
          :label="$t('settings.soft_stop_strength')" 
          value-suffix="%"
          help-key="help.soft_stop_strength"
          @change="saveEffects(HardwareSettingId.SoftStopStrength)"
        />
        <ThemedSlider 
          v-model="effects.soft_stop_range" 
          :label="$t('settings.soft_stop_range')" 
          value-suffix="°"
          :max="255"
          help-key="help.soft_stop_range"
          @change="saveEffects(HardwareSettingId.SoftStopRange)"
        />
        <ThemedSlider 
          v-model="effects.soft_stop_dampening_strength" 
          :label="$t('settings.soft_stop_dampening')" 
          value-suffix="%"
          :max="100"
          help-key="help.soft_stop_dampening"
          @change="saveEffects(HardwareSettingId.SoftStopDampening)"
        />
      </BaseCard>

      <!-- Dampening & Resistance -->
      <BaseCard :title="$t('groups.dampening')">
        <ThemedSlider 
          v-model="effects.integrated_spring_strength" 
          :label="$t('settings.integrated_spring')" 
          value-suffix="%"
          help-key="help.integrated_spring_strength"
          @change="saveEffects(HardwareSettingId.IntegratedSpring)"
        />
        <ThemedSlider 
          v-model="effects.static_dampening_strength" 
          :label="$t('settings.static_dampening')" 
          value-suffix="%"
          :max="100"
          help-key="help.static_dampening_strength"
          @change="saveEffects(HardwareSettingId.StaticDampening, true)"
        />
        <ThemedSlider 
          v-model="effects.dynamic_dampening_strength" 
          :label="$t('settings.dynamic_dampening')" 
          value-suffix="%"
          :max="100"
          help-key="help.dynamic_dampening_strength"
          @change="saveEffects(HardwareSettingId.DynamicDampening, true)"
        />
      </BaseCard>

      <!-- Game API / DirectX -->
      <BaseCard :title="$t('groups.directx')">
        <ThemedSlider 
          v-model="effects.direct_x_constant_strength" 
          :label="$t('settings.dx_constant')" 
          value-suffix="%"
          help-key="help.direct_x_constant_strength"
          @change="saveEffects(HardwareSettingId.DirectXConstant)"
        />
        <ThemedSlider 
          v-model="effects.direct_x_periodic_strength" 
          :label="$t('settings.dx_periodic')" 
          value-suffix="%"
          help-key="help.direct_x_periodic_strength"
          @change="saveEffects(HardwareSettingId.DirectXPeriodic)"
        />
        <ThemedSlider 
          v-model="effects.direct_x_spring_strength" 
          :label="$t('settings.dx_spring')" 
          value-suffix="%"
          help-key="help.direct_x_spring_strength"
          @change="saveEffects(HardwareSettingId.DirectXSpring)"
        />
        <ThemedSwitch 
          v-model="invertGameForce" 
          :label="$t('settings.invert_game_force')" 
          help-key="help.invert_game_force"
          @change="handleInvert"
        />
      </BaseCard>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import ThemedSlider from '@shared/components/atoms/ThemedSlider.vue';
import ThemedSwitch from '@shared/components/atoms/ThemedSwitch.vue';
import { HardwareSettingId } from '../../models/HardwareSettingId';

const store = useHardwareStore();

// Use a local reactive copy to avoid laggy UI
const effects = reactive({
  motion_range: store.effects?.motion_range ?? 900,
  total_effect_strength: store.effects?.total_effect_strength ?? 100,
  integrated_spring_strength: store.effects?.integrated_spring_strength ?? 0,
  static_dampening_strength: store.effects?.static_dampening_strength ?? 0,
  dynamic_dampening_strength: store.effects?.dynamic_dampening_strength ?? 0,
  soft_stop_strength: store.effects?.soft_stop_strength ?? 50,
  soft_stop_range: store.effects?.soft_stop_range ?? 10,
  soft_stop_dampening_strength: store.effects?.soft_stop_dampening_strength ?? 0,
  direct_x_constant_strength: store.effects?.direct_x_constant_strength ?? 100,
  direct_x_periodic_strength: store.effects?.direct_x_periodic_strength ?? 100,
  direct_x_spring_strength: store.effects?.direct_x_spring_strength ?? 100,
  direct_x_constant_direction: store.effects?.direct_x_constant_direction ?? 0,
});

const invertGameForce = ref(effects.direct_x_constant_direction === 1);

const saveEffects = (fieldId?: number, isU16: boolean = false) => {
  const currentFX = {
    ...effects,
    direct_x_constant_direction: invertGameForce.value ? 1 : 0
  };

  if (fieldId !== undefined) {
    let value = 0;
    switch (fieldId) {
      case HardwareSettingId.MotionRange: value = currentFX.motion_range; break;
      case HardwareSettingId.TotalEffectStrength: value = currentFX.total_effect_strength; break;
      case HardwareSettingId.SoftStopStrength: value = currentFX.soft_stop_strength; break;
      case HardwareSettingId.SoftStopRange: value = currentFX.soft_stop_range; break;
      case HardwareSettingId.SoftStopDampening: value = currentFX.soft_stop_dampening_strength; break;
      case HardwareSettingId.IntegratedSpring: value = currentFX.integrated_spring_strength; break;
      case HardwareSettingId.StaticDampening: value = currentFX.static_dampening_strength; break;
      case HardwareSettingId.DynamicDampening: value = currentFX.dynamic_dampening_strength; break;
      case HardwareSettingId.DirectXConstant: value = currentFX.direct_x_constant_strength; break;
      case HardwareSettingId.DirectXPeriodic: value = currentFX.direct_x_periodic_strength; break;
      case HardwareSettingId.DirectXSpring: value = currentFX.direct_x_spring_strength; break;
      case HardwareSettingId.DirectXConstantDirection: value = currentFX.direct_x_constant_direction; break;
    }
    store.updateFXField(fieldId, 0, value, isU16);
  }

  store.updateFX(currentFX);
};

const handleInvert = () => {
  saveEffects(HardwareSettingId.DirectXConstantDirection);
};

// Sync back from store if it changes
watch(() => store.effects, (newVal) => {
  if (newVal) {
    Object.assign(effects, newVal);
    invertGameForce.value = newVal.direct_x_constant_direction === 1;
  }
}, { deep: true });
</script>

<style scoped>
.effects-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}
</style>
