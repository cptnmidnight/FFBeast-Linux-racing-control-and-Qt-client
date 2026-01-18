let currentTranslations = {};

/**
 * Loads translation file based on language code
 * @param {string} lang 
 */
export async function loadTranslations(lang) {
    try {
        const response = await fetch(`./i18n/${lang}.json`);
        if (!response.ok) throw new Error(`Could not load ${lang} translation`);
        currentTranslations = await response.json();
    } catch (error) {
        console.error(`[i18n] Error loading translations for ${lang}:`, error);
        // Fallback to English if loading fails
        if (lang !== 'en') await loadTranslations('en');
    }
}

/**
 * Gets a translation string for a given key
 * @param {string} key 
 * @returns {string} processed translation or key itself
 */
export function translate(key) {
    return currentTranslations[key] || key;
}

/**
 * Detects the browser language
 * @returns {string} detected language code (en or pt)
 */
export function detectLanguage() {
    const browserLang = navigator.language.split('-')[0];
    const supportedLangs = ['en', 'pt'];
    return supportedLangs.includes(browserLang) ? browserLang : 'en';
}
