<script>
	import { GithubIcon, Instagram, Linkedin } from 'lucide-svelte';
	import '../app.postcss';
	import NavBar from './NavBar.svelte';
</script>

<svelte:head>
	<style>
		@import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;600;800&display=swap');

		body {
			font-family: 'Montserrat';
			font-style: 'normal';
			font-weight: 400;
			background-color: #18181b;
		}
	</style>
</svelte:head>

<NavBar />
<slot />

<section class="space-y-5 bg-zinc-900 py-10">
	<div class="mx-auto flex flex-row justify-center space-x-5">
		<a href="https://www.linkedin.com/in/AmirAbdRazak">
			<div class="rounded-full border-4 border-violet-200 p-2 text-violet-200">
				<Linkedin />
			</div>
		</a>
		<a href="https://www.github.com/AmirAbdRazak">
			<div
				class="rounded-full border-4 border-fuchsia-200 p-2 text-fuchsia-200"
			>
				<GithubIcon />
			</div>
		</a>
		<a href="https://www.instagram.com/amr_rzk02/">
			<div class="rounded-full border-4 border-rose-200 p-2 text-rose-200">
				<Instagram />
			</div>
		</a>
	</div>
	<p class="text-center text-zinc-200">
		© {new Date().getFullYear()} Muhammad Amir Abdul Razak
	</p>
</section>
