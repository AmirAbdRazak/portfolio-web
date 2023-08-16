import { env } from '$env/dynamic/private';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	return {
		backend_url: env.AXUM_URL || 'http://localhost:8000/'
	};
};
