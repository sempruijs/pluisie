<script lang="ts">
	import '../app.css';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	let { children } = $props();
</script>

<div class="min-h-screen flex flex-col">
	<Header />
	
	<div class="flex flex-1">
		<Sidebar />
		<main class="flex-1 p-4">
			{@render children()}
		</main>
	</div>
	
	<Footer />
</div>
