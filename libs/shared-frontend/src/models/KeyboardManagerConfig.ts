import type { KeyboardProfile } from './KeyboardProfile';

export interface KeyboardManagerConfig {
    active_profile_id: string | null;
    profiles: KeyboardProfile[];
}
