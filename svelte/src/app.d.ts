// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: User | null
			something: string
		}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};

declare module '@fortawesome/pro-solid-svg-icons/index.es' {
	export * from '@fortawesome/pro-solid-svg-icons';
}

declare module '$env/static/private' {
	export const AXUM_URL: string;
}
