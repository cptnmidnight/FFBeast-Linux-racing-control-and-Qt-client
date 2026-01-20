export interface KeyMapping {
    id: string;
    source_type: 'button' | 'axis';
    index: number;
    trigger: 'press' | 'high' | 'low';
    key: string;
    threshold?: number;
}
