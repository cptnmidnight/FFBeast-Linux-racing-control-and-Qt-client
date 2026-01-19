<template>
  <div class="buttons-tab">
    <div class="grid-layout">
      <!-- Live Status -->
      <BaseCard :title="$t('buttons_title')">
        <ButtonsGrid :buttons="currentButtons" />
      </BaseCard>

      <!-- Digital Mapping -->
      <BaseCard title="Digital Mapping">
        <p class="description">Map physical buttons to virtual joystick buttons.</p>
        <div class="mapping-list">
          <div v-for="n in 12" :key="n" class="mapping-item">
            <span class="index">#{{ n }}</span>
            <BaseSelect 
              v-model="dummyMapping[n-1]" 
              :options="buttonOptions" 
              class="compact"
            />
          </div>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';
import ButtonsGrid from '../monitor/ButtonsGrid.vue';

const store = useHardwareStore();
const currentButtons = computed(() => store.status?.buttons ?? 0);

const dummyMapping = reactive(Array(12).fill(0));
const buttonOptions = [
  { label: 'Button 1', value: 0 },
  { label: 'Button 2', value: 1 },
  { label: 'Button 3', value: 2 },
  // ...
];
</script>

<style scoped>
.buttons-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

.description {
  font-size: 0.85rem;
  color: var(--text-dim);
  margin-bottom: 1rem;
}

.mapping-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
}

.mapping-item {
  display: flex;
  align-items: center;
  gap: 10px;
  background: rgba(0,0,0,0.2);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
}

.index {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--text-dim);
  width: 30px;
}

.compact {
  margin-bottom: 0 !important;
  flex: 1;
}
</style>
