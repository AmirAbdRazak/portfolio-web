import { writable, type Writable } from 'svelte/store';

export interface ChartFormData {
	chart_type: string;
	offset?: number;
	limit?: number;
}

export const formDataStore: Writable<ChartFormData> = writable({
	chart_type: 'Artist',
	offset: 0,
	limit: 10
});
