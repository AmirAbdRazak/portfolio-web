import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (params) => {
	return {
		backend_url:
			params.url.origin == 'http://localhost:5173'
				? 'http://localhost:8000/'
				: 'https://axum-backend.fly.dev/'
	};
};
