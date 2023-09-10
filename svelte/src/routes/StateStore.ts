import { writable, type Writable } from 'svelte/store';

export interface StateData {
	isDarkMode: boolean;
}

export const stateStore: Writable<StateData> = writable({
	isDarkMode: true
});
