<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import { Game } from '../../types';
import { GameService } from '../../services/tauri';

export default defineComponent({
  name: 'GameList',
  setup() {
    const games = ref<Game[]>([]);
    const loading = ref(false);
    const error = ref<string | null>(null);

    const loadGames = async () => {
      loading.value = true;
      error.value = null;
      try {
        const [saved, scanned] = await Promise.all([
          GameService.getGames().catch(() => []),
          GameService.scanGames().catch(() => [])
        ]);

        // Merge logic
        const gameMap = new Map<string, Game>();
        scanned.forEach(g => gameMap.set(g.id, g));
        saved.forEach(g => gameMap.set(g.id, g)); // Saved overrides scanned

        games.value = Array.from(gameMap.values());
      } catch (e) {
        error.value = "Failed to load games";
        console.error(e);
      } finally {
        loading.value = false;
      }
    };

    const handleLaunch = async (id: string) => {
        try {
            await GameService.launchGame(id);
        } catch(e) {
            alert("Error launching game: " + e);
        }
    };

    onMounted(() => {
      loadGames();
    });

    return {
      games,
      loading,
      error,
      loadGames,
      handleLaunch
    };
  }
});
</script>

<template>
  <div class="game-list">
    <div v-if="loading" class="loading">Scanning Library...</div>
    
    <div v-else-if="games.length === 0" class="empty-state">
      <p>{{ $t('library.no_games') }}</p>
      <button @click="loadGames" class="btn-refresh">{{ $t('library.scan_retry') }}</button>
    </div>

    <div v-else class="grid">
      <div v-for="game in games" :key="game.id" class="game-card">
        <div class="cover-placeholder">
            <span class="icon">🎮</span>
        </div>
        <div class="info">
            <h3>{{ game.name }}</h3>
            <button @click="handleLaunch(game.id)" class="btn-launch">{{ $t('library.launch') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.game-list {
    width: 100%;
}

.loading {
    padding: 2rem;
    text-align: center;
    color: var(--text-color);
    opacity: 0.7;
}

.empty-state {
    padding: 4rem;
    text-align: center;
    background: var(--card-bg);
    border-radius: 12px;
    border: 1px dashed var(--border-color);
}

.grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
}

.game-card {
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    overflow: hidden;
    transition: transform 0.2s, border-color 0.2s;
    display: flex;
    flex-direction: column;
}

.game-card:hover {
    transform: translateY(-4px);
    border-color: var(--primary-color);
}

.cover-placeholder {
    height: 140px;
    background: linear-gradient(135deg, #2b2b2b 0%, #1a1a1a 100%);
    display: flex;
    align-items: center;
    justify-content: center;
}

.cover-placeholder .icon {
    font-size: 3rem;
    opacity: 0.5;
}

.info {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    flex: 1;
}

.info h3 {
    font-size: 1.1rem;
    font-weight: 600;
}

.btn-launch {
    margin-top: auto;
    width: 100%;
    padding: 0.8rem;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
}

.btn-launch:hover {
    background: var(--primary-hover);
}

.btn-refresh {
    margin-top: 1rem;
    padding: 0.6rem 1.2rem;
    background: transparent;
    border: 1px solid var(--primary-color);
    color: var(--primary-color);
    border-radius: 8px;
    cursor: pointer;
}
</style>
