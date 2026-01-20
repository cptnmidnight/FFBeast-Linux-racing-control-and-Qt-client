// Helper to get translations outside Vue components
// This uses the global i18n instance that's created in main.ts

let i18nInstance: any = null;

export function setI18nInstance(instance: any) {
    i18nInstance = instance;
}

export function t(key: string): string {
    if (!i18nInstance) {
        console.warn('[i18n] Instance not set, returning key:', key);
        return key;
    }
    return i18nInstance.global.t(key);
}
