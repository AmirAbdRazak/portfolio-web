import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const ssr = false;
export const load: PageServerLoad = ({ params }) => {
	const backend_url = env.AXUM_URL || 'http://localhost:8000/';
	const username = params.username;

	return { username, backend_url };
};
