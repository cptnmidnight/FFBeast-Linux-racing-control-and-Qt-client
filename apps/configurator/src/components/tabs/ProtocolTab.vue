<template>
  <div class="protocol-tab">
    <BaseCard title="Communication Protocol">
      <p class="description">Select the protocol used to communicate with the device.</p>
      <BaseSelect 
        v-model="extensionMode" 
        :options="options" 
        label="Active Extension" 
        @update:model-value="save"
      />
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';

const store = useHardwareStore();
const extensionMode = ref(store.gpio?.extension_mode ?? 0);

const options = [
  { label: 'None', value: 0 },
  { label: 'FFBeast Legacy', value: 1 },
  { label: 'DirectHID', value: 2 },
  { label: 'CAN-Bus', value: 3 },
];

const save = () => {
  // store.updateGPIO({ extension_mode: Number(extensionMode.value) });
};
</script>

<style scoped>
.protocol-tab {
  padding: var(--content-padding);
}

.description {
  color: var(--text-dim);
  font-size: 0.9rem;
  margin-bottom: 2rem;
}
</style>
