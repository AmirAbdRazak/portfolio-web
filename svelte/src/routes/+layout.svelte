<script>
	import { GithubIcon, Instagram, Linkedin } from 'lucide-svelte';
	import '../app.postcss';
	import NavBar from './NavBar.svelte';
	import { MetaTags } from 'svelte-meta-tags';
	import MetaCover from '$lib/assets/metacover.png';
</script>

<svelte:head>
	<style>
		@import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;600;800&display=swap');
		@import url('https://fonts.googleapis.com/css2?family=Spectral:wght@100;400;600&display=swap');

		body {
			font-family: 'Montserrat';
			font-style: 'normal';
			font-weight: 400;
			background-color: #18181b;
		}

		.font-serif-spectral {
			font-family: 'Spectral', 'Helvetica';
		}

		.font-spectral-thin {
			font-weight: 100;
		}
	</style>
</svelte:head>

<MetaTags
	title="Amir Razak"
	titleTemplate="%s | Portfolio Website"
	description="Backend Engineer in his early careers who's looking forward for establishing digital footprints globally"
	canonical="https://www.amirrazak.com"
	openGraph={{
		type: 'website',
		url: 'https://www.amirrazak.com',
		locale: 'en_US',
		title: "Amir Razak's Portfolio Website",
		description:
			"Backend Engineer in his early careers who's looking forward to establish digital footprints globally",
		images: [
			{
				url: MetaCover,
				alt: "Amir Razak's Portfolio Cover Page",
				width: 1200,
				height: 630,
				secureUrl: MetaCover,
				type: 'photo'
			}
		],
		siteName: 'Amir Razak'
	}}
/>

<NavBar
	homeNav={[
		{ link: '/charts', title: 'Charts' },
		{ link: '/resume', title: 'Resume' }
	]}
/>
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
