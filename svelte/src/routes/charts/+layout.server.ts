import {env} from '$env/dynamic/private';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	return {
		backend_url: env.DEV_MODE ? 'http://localhost:8000/' : 'https://axum-backend.fly.dev/'
	};
};
