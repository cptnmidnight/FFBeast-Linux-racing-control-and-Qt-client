import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import { createI18n } from 'vue-i18n';
import { setI18nInstance } from './i18n';

// Styles
import './styles/global.css';

// i18n
import en from './locales/en.json';
import br from './locales/pt-BR.json';
import es from './locales/es.json';
import ru from './locales/ru.json';

// Detect browser language
const browserLang = navigator.language.toLowerCase();
let locale = 'en'; // default

if (browserLang.startsWith('pt')) {
    locale = 'pt-BR';
} else if (browserLang.startsWith('es')) {
    locale = 'es';
} else if (browserLang.startsWith('ru')) {
    locale = 'ru';
}

const i18n = createI18n({
    legacy: false,
    locale: locale,
    fallbackLocale: 'en',
    messages: {
        en,
        'pt-BR': br,
        'pt': br,
        'es': es,
        'ru': ru
    }
});

// Make i18n available to stores
setI18nInstance(i18n);

const app = createApp(App);
app.use(createPinia());
app.use(i18n);
app.mount('#app');
