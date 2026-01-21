<template>
  <div class="effects-tab">
    <div class="grid-layout">
      <!-- Basics -->
      <BaseCard :title="$t('groups.general')">
        <BaseSlider 
          v-model="effects.motion_range" 
          :label="$t('settings.motion_range')" 
          :help="$t('help.motion_range')"
          :min="180" 
          :max="1440" 
          :step="10" 
          suffix="°" 
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.total_effect_strength" 
          :label="$t('settings.total_strength')" 
          :help="$t('help.total_effect_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
      </BaseCard>

      <!-- Soft Stop -->
      <BaseCard :title="$t('groups.soft_stop')">
        <BaseSlider 
          v-model="effects.soft_stop_strength" 
          :label="$t('settings.soft_stop_strength')" 
          :help="$t('help.soft_stop_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.soft_stop_range" 
          :label="$t('settings.soft_stop_range')" 
          :help="$t('help.soft_stop_range')"
          suffix="°"
          :max="255"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.soft_stop_dampening_strength" 
          :label="$t('settings.soft_stop_dampening')" 
          :help="$t('help.soft_stop_dampening')"
          suffix="%"
          :max="1000"
          @update:model-value="saveEffects"
        />
      </BaseCard>

      <!-- Internal Filter -->
      <BaseCard :title="$t('groups.dampening')">
        <BaseSlider 
          v-model="effects.integrated_spring_strength" 
          :label="$t('settings.integrated_spring')" 
          :help="$t('help.integrated_spring_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.static_dampening_strength" 
          :label="$t('settings.static_dampening')" 
          :help="$t('help.static_dampening_strength')"
          suffix="%"
          :max="1000"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.dynamic_dampening_strength" 
          :label="$t('settings.dynamic_dampening')" 
          :help="$t('help.dynamic_dampening_strength')"
          suffix="%"
          :max="1000"
          @update:model-value="saveEffects"
        />
      </BaseCard>

      <!-- DirectX (Game Effects) -->
      <BaseCard :title="$t('groups.directx')">
        <BaseSlider 
          v-model="effects.direct_x_constant_strength" 
          :label="$t('settings.dx_constant')" 
          :help="$t('help.direct_x_constant_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.direct_x_periodic_strength" 
          :label="$t('settings.dx_periodic')" 
          :help="$t('help.direct_x_periodic_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
        <BaseSlider 
          v-model="effects.direct_x_spring_strength" 
          :label="$t('settings.dx_spring')" 
          :help="$t('help.direct_x_spring_strength')"
          suffix="%"
          @update:model-value="saveEffects"
        />
        <BaseSwitch 
          v-model="invertGameForce" 
          :label="$t('settings.invert_game_force')" 
          :help="$t('help.invert_game_force')"
          @update:model-value="handleInvert"
        />
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';
import BaseSwitch from '../common/BaseSwitch.vue';

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

const saveEffects = () => {
  store.updateFX({
    ...effects,
    direct_x_constant_direction: invertGameForce.value ? 1 : 0
  });
};

const handleInvert = () => {
  saveEffects();
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
