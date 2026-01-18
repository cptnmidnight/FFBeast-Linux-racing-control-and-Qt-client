<script lang="ts">
import { defineComponent, ref } from 'vue';
import GameList from './components/GameList/GameList.vue';
import HardwareMonitor from './components/HardwareMonitor/HardwareMonitor.vue';
import WheelConfigModal from './components/WheelConfig/WheelConfigModal.vue';
import AddGameModal from './components/AddGameModal/AddGameModal.vue';

export default defineComponent({
  name: 'App',
  components: {
    GameList,
    HardwareMonitor,
    WheelConfigModal,
    AddGameModal
  },
  setup() {
    const showAddModal = ref(false);
    const showConfigModal = ref(false);
    const gameListRef = ref<InstanceType<typeof GameList> | null>(null);

    const onGameSaved = () => {
        // Refresh list
        if(gameListRef.value) {
            gameListRef.value.loadGames();
        }
    };

    return {
        showAddModal,
        showConfigModal,
        gameListRef,
        onGameSaved
    };
  }
});
</script>

<template>
  <div class="container">
    <header>
      <h1>{{ $t('app.title') }}</h1>
      <!-- Language selector implementation later -->
    </header>
    
    <main>
       <!-- Top Section: Hardware Info and Config -->
       <HardwareMonitor @open-config="showConfigModal = true" />

       <!-- Bottom Section: Library -->
       <section class="library-section">
          <div class="section-header">
            <h2>{{ $t('library.title') }}</h2>
            <button class="btn-add" @click="showAddModal = true">+ {{ $t('library.add_game') }}</button>
          </div>
          <GameList ref="gameListRef" />
       </section>
    </main>
    
    <AddGameModal 
        v-if="showAddModal" 
        @close="showAddModal = false"
        @saved="onGameSaved"
    />
    
    <WheelConfigModal
        v-if="showConfigModal"
        @close="showConfigModal = false"
    />

    <footer>
        <p>SODevs Launcher v0.1.0 (Vue + Rust)</p>
    </footer>
  </div>
</template>


<style scoped>
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

header {
    margin-bottom: 3rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

h1 {
  background: linear-gradient(to right, #8b5cf6, #d946ef);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  font-weight: 800;
}

.section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
}

.btn-add {
    background: var(--primary-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.2s;
}

.btn-add:hover {
    background: var(--primary-hover);
}

main {
    min-height: 60vh;
}

footer {
    margin-top: 4rem;
    text-align: center;
    opacity: 0.4;
    font-size: 0.8rem;
}
</style>
