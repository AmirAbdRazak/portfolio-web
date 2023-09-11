<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';

	import { Download, Info, Linkedin, Mail, Phone } from 'lucide-svelte';
	import PortfolioCard from './PortfolioCard.svelte';
	import TechStackImage from './TechStackImage.svelte';

	import resumePDF from '$lib/assets/Amir Razak SWE Resume .pdf';

	import chartImage from '$lib/assets/chartexample.png';
	import ecommImage from '$lib/assets/ecommfypexample.png';
	import rustLogo from '$lib/assets/rust.png';
	import tsLogo from '$lib/assets/ts.png';
	import jsLogo from '$lib/assets/js.png';
	import pyLogo from '$lib/assets/python.png';
	import dockerLogo from '$lib/assets/docker.png';
	import djangoLogo from '$lib/assets/django.png';
	import gqlLogo from '$lib/assets/gql.png';
	import nextLogo from '$lib/assets/nextjs.png';
	import linuxLogo from '$lib/assets/linux.png';
	import svelteLogo from '$lib/assets/svelte.png';
	import { GithubIcon } from 'lucide-svelte';
</script>

<section class="bg-gradient-to-b from-indigo-200 to-violet-300">
	<div class="mx-auto max-w-screen-xl px-4 py-8 text-left lg:px-12 lg:py-16">
		<div class="flex flex-col justify-around lg:flex-row">
			<Avatar.Root
				class="mx-auto mb-10 block h-40 w-40 border-4 border-zinc-800 lg:hidden"
			>
				<Avatar.Image
					src="https://github.com/amirabdrazak.png"
					alt="@amirabdrazak"
				/>
				<Avatar.Fallback class="bg-transparent">Amir Razak</Avatar.Fallback>
			</Avatar.Root>
			<div>
				<h1
					class="mb-4 pl-5 text-3xl font-extrabold leading-none tracking-tight text-zinc-900 md:text-5xl lg:text-5xl"
				>
					Backend Engineer
				</h1>
				<p class="text-md mb-8 pl-5 font-normal text-zinc-800 lg:text-lg">
					I'm Amir Razak, a backend engineer with an insatiable curiosity for
					bleeding-edge technology. My expertise spans a wide range of areas,
					from building robust RESTful APIs to seamlessly managing database
					migrations.
				</p>
				<ul
					class="text-md mx-auto mb-8 grid grid-cols-2 items-baseline space-y-2 font-normal text-zinc-800 sm:pl-5 lg:text-lg"
				>
					<li
						class="flex flex-col space-x-2 space-y-2 sm:flex-row sm:space-y-0"
					>
						<Phone class="mx-auto sm:mx-0" />
						<p class="hidden sm:block">:</p>
						<p class="text-center sm:text-left">+60-175005516</p>
					</li>
					<li
						class="flex flex-col space-x-2 space-y-2 sm:flex-row sm:space-y-0"
					>
						<Mail class="mx-auto sm:mx-0" />
						<p class="hidden sm:block">:</p>
						<p class="text-center sm:text-left">amrrzk02@gmail.com</p>
					</li>
					<li
						class="flex flex-col space-x-2 space-y-2 sm:flex-row sm:space-y-0"
					>
						<Linkedin class="mx-auto sm:mx-0" />
						<p class="hidden sm:block">:</p>
						<a
							href="https://www.linkedin.com/in/AmirAbdRazak"
							class="text-center underline sm:text-left">@AmirAbdRazak</a
						>
					</li>
					<li
						class="flex flex-col space-x-2 space-y-2 sm:flex-row sm:space-y-0"
					>
						<GithubIcon class="mx-auto sm:mx-0" />
						<p class="hidden sm:block">:</p>
						<a
							href="https://www.github.com/AmirAbdRazak"
							class="text-center underline sm:text-left"
						>
							@AmirAbdRazak</a
						>
					</li>
				</ul>
				<div class="flex flex-row items-center pl-5">
					<Button
						class="flex flex-row space-x-2 rounded-lg bg-zinc-800 px-5 text-lg font-semibold text-violet-200 "
						href={resumePDF}
						download="Amir Razak SWE Resume"
						><p class="hidden sm:block">Download Resume</p>
						<p class="block sm:hidden">Resume</p>
						<Download /></Button
					>
					<a href="/resume" class="hidden px-5 text-zinc-800 underline sm:block"
						>Or view it on the web</a
					>
					<a href="/resume" class="block px-5 text-zinc-800 underline sm:hidden"
						>Browser View</a
					>
				</div>
			</div>
			<Avatar.Root
				class="my-auto ml-20 mr-20 hidden h-52 w-52 border-4 border-zinc-800 lg:block"
			>
				<Avatar.Image
					src="https://github.com/amirabdrazak.png"
					alt="@amirabdrazak"
				/>
				<Avatar.Fallback class="bg-transparent">Amir Razak</Avatar.Fallback>
			</Avatar.Root>
		</div>
	</div>
</section>
<section class="bg-zinc-800">
	<div
		class="mx-auto flex flex-row items-center justify-center space-x-5 py-10 text-violet-100"
	>
		<h1 class="text-center text-4xl font-semibold">Programming Languages</h1>
		<div class="flex flex-row items-center space-x-2 opacity-70">
			<Tooltip.Root openDelay={2}>
				<Tooltip.Trigger>
					<Info class="" size={20} />
				</Tooltip.Trigger>
				<Tooltip.Content
					class="bg-gradient-to-br from-indigo-100 to-violet-300 text-zinc-800"
				>
					<p>Hover over icons in this section to get more details</p>
				</Tooltip.Content>
			</Tooltip.Root>
		</div>
	</div>
	<div
		class="grid grid-cols-2 gap-10 md:mx-auto md:flex md:flex-row md:justify-center md:space-x-10"
	>
		<TechStackImage imgSrc={pyLogo} imgAlt="Python Programming Language" />
		<TechStackImage imgSrc={tsLogo} imgAlt="TypeScript Programming Language" />
		<TechStackImage imgSrc={jsLogo} imgAlt="JavaScript Programming Language" />
		<TechStackImage imgSrc={rustLogo} imgAlt="Rust Programming Language" />
	</div>
	<div
		class="mx-auto flex flex-row items-center justify-center space-x-5 py-10 text-violet-100"
	>
		<h1 class="text-center text-4xl font-semibold">
			Backend & Infra Technology
		</h1>
	</div>
	<div
		class="grid grid-cols-2 gap-10 pb-10 md:grid-cols-3 lg:mx-auto lg:flex lg:flex-row lg:justify-center lg:space-x-10"
	>
		<TechStackImage imgSrc={dockerLogo} imgAlt="Docker" />
		<TechStackImage imgSrc={djangoLogo} imgAlt="Django Backend Framework" />
		<TechStackImage imgSrc={gqlLogo} imgAlt="GraphQL API Server" />
		<TechStackImage imgSrc={linuxLogo} imgAlt="Linux Operating System" />
		<TechStackImage imgSrc={nextLogo} imgAlt="NextJS Full Stack Framework" />
		<TechStackImage
			imgSrc={svelteLogo}
			imgAlt="SvelteKit Full Stack Framework"
		/>
		<TechStackImage
			><GithubIcon
				class="mx-auto h-24 w-24 rounded-lg object-contain"
			/></TechStackImage
		>
	</div>
</section>
<section class="bg-gradient-to-br from-indigo-100 to-violet-300">
	<h1 class="py-10 text-center text-4xl font-semibold text-zinc-800">
		Professional Experience
	</h1>
	<div class="px-10 pb-10 text-zinc-800 md:pl-20 lg:pl-24">
		<p>Feb 2023 - Aug 2023</p>
		<p class="text-xl font-semibold">
			Backend Engineer Intern @ Infin8co Sdn Bhd
		</p>
		<p class="md:pr-24 lg:pr-48 xl:pr-72">
			Joined the team to work on an enterprise chat platform named Talkhub as a
			Backend Engineer Intern. Primarily responsible for 2 different feature
			sections, namely CSV Importing and Reporting Metrics within the respective
			GraphQL endpoints, inside the Django and Docker ecosystem. Committed a few
			thousand lines of code into production including rigorous test cases for
			reliability.
		</p>
	</div>
</section>
<section class="bg-zinc-800">
	<div
		class="mx-auto flex flex-row items-center justify-center space-x-5 py-10 text-violet-100"
	>
		<h1 class="text-center text-4xl font-semibold">Portfolio</h1>
		<div class="flex flex-row items-center space-x-2 opacity-70">
			<Tooltip.Root openDelay={2}>
				<Tooltip.Trigger>
					<Info class="" size={20} />
				</Tooltip.Trigger>
				<Tooltip.Content class="border-2 bg-zinc-700 text-violet-100">
					<p>
						Hover over portfolio card for more info about the tech stack used
					</p>
				</Tooltip.Content>
			</Tooltip.Root>
		</div>
	</div>
	<div class="grid transform grid-cols-1 gap-10 px-10 pb-20 md:grid-cols-2">
		<PortfolioCard
			title="LastFM Chart"
			description="Listening history data visualization for LastFM users"
			cardImage={chartImage}
			githubRepo="https://github.com/AmirAbdRazak/portfolio-web"
			cardLink="/charts"
			techStack={[
				'Rust',
				'TypeScript',
				'JavaScript',
				'Svelte',
				'Axum',
				'Docker',
				'GraphQL API',
				'PostgreSQL'
			]}
		/>
		<PortfolioCard
			title="Mock E-Commerce FYP"
			description="Mock website for showcasing recommendation algorithm (deprecated) during my Final Year Project"
			cardImage={ecommImage}
			githubRepo="https://github.com/AmirAbdRazak/ecomm-fyp"
			cardLink="https://ecomm-fyp.vercel.app/"
			techStack={[
				'TypeScript',
				'NextJS',
				'RESTful API',
				'Python',
				'PostgreSQL'
			]}
		/>
	</div>
</section>
