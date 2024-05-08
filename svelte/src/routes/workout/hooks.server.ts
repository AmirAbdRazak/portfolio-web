import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Stage 1
	//
	//
	event.locals.something = 'whatever i want';

	const response = await resolve(event) // Stage 2

	// Stage 3

	return response;

}
