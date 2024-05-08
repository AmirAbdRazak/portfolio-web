import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({ locals }) => {
	console.log(locals);
}

export const actions: Actions = {

	login: async ({ cookies, locals }) => {
		console.log(locals);
		cookies.set("auth", "regularusertoken", {
			path: "/workout",
			httpOnly: true,
			sameSite: "strict",
			secure: !env.DEV_MODE,
			maxAge: 60 * 60 * 24 * 7,
		})

		throw redirect(303, "/")
	},
}
